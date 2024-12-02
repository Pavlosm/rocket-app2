use mongodb::{Client, options::{ClientOptions, ResolverConfig}, Database};
use std::env;
use rocket::fairing::AdHoc;

const MONGODB_URI: &str = "MONGODB_URI";
const MONGODB_RUST_APP: &str = "MONGODB_RUST_APP";
const MONGODB_PROTOCOL: &str = "MONGODB_PROTOCOL";
const MONGODB_PORT: &str = "MONGODB_PORT";
const MONGODB_USERNAME: &str = "MONGODB_USERNAME";
const MONGODB_PASSWORD: &str = "MONGODB_PASSWORD";
const MONGODB_CONN_PREFERENCES: &str = "MONGODB_CONN_PREFERENCES";


pub fn init() -> AdHoc {
    AdHoc::on_ignite("Connecting to MongoDB", |rocket| async {
        match connect().await {
            Ok(database) => rocket.manage(database),
            Err(error) => {
                panic!("Cannot connect to instance:: {:?}", error)
            }
        }
    })
}

pub async fn connect() -> Result<Database, mongodb::error::Error> {
        
    let con_str = match env::var(MONGODB_PORT) {
      Ok(port) if !port.is_empty() => configure_local(port),
       _ => configure_remote()
    };

    let db_name = extract_env_var_with_azure_prefix(String::from(MONGODB_RUST_APP));

    println!("conn string: {}, DB: {}", con_str, db_name);
    
    let options = ClientOptions::parse_with_resolver_config(con_str, ResolverConfig::cloudflare()).await?;

    let db = Client::with_options(options)
        .unwrap()
        .database(&db_name);

    for db_names in  db.list_collection_names(None).await? {
        println!("{}", db_names);
    }

    Ok(db)
}

fn configure_remote() -> String {

    println!("configure remote mongo db");

    let user_name = extract_env_var_with_azure_prefix(String::from(MONGODB_USERNAME));

    let password = extract_env_var_with_azure_prefix(String::from(MONGODB_PASSWORD));

    let protocol = extract_env_var_with_azure_prefix(String::from(MONGODB_PROTOCOL));

    let uri = extract_env_var_with_azure_prefix(String::from(MONGODB_URI));

    let extra = extract_optional_env_var_with_azure_prefix(String::from(MONGODB_CONN_PREFERENCES));

    let conn_str = format!("{}://{}:{}@{}/{}", protocol, user_name, password, uri, extra);
    
    conn_str
}

fn configure_local(port: String) -> String {

    println!("configure local mongo db for port: {}", port);

    let protocol = extract_env_var_with_azure_prefix(String::from(MONGODB_PROTOCOL));

    let uri = extract_env_var_with_azure_prefix(String::from(MONGODB_URI));

    let conn_str = format!("{}://{}:{}/", protocol, uri, port);

    println!("conn string: {}", conn_str);

    conn_str    
}

fn extract_env_var_with_azure_prefix(name: String) -> String {
    match env::var(name.clone()) {
        Ok(env_var) => env_var,
        Err(_) => env::var(String::from(format!("APPSETTINGS_{}", name)))
        .expect(&format!("You must set the {} environment var!", name))
    }
}

fn extract_optional_env_var_with_azure_prefix(name: String) -> String {
    match env::var(name.clone()) {
        Ok(env_var) => env_var,
        Err(_) => match env::var(String::from(format!("APPSETTINGS_{}", name))) {
            Ok(env_var) => env_var,
            Err(e) => String::from("")
        }
    }
}