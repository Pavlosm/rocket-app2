use mongodb::{Client, options::{ClientOptions, ResolverConfig}, Database};
use std::env;
use rocket::fairing::AdHoc;

const MONGODB_URI: &str = "MONGODB_URI";
const MONGODB_RUST_APP: &str = "MONGODB_RUST_APP";
const MONGODB_PROTOCOL: &str = "MONGODB_PROTOCOL";
const MONGODB_PORT: &str = "MONGODB_PORT";
const MONGODB_USERNAME: &str = "MONGODB_USERNAME";
const MONGODB_PASSWORD: &str = "MONGODB_PASSWORD";

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

    let db_name: String = env::var(MONGODB_RUST_APP)
        .expect(&format!("You must set {} environment variable", MONGODB_RUST_APP));

    println!("conn string: {}, DB: {}", con_str, db_name);
    
    let options = ClientOptions::parse_with_resolver_config(con_str, ResolverConfig::cloudflare())
        .await
        .unwrap();

    let client = Client::with_options(options).unwrap();

    for db_names in client.list_database_names(None, None).await? {
        println!("{}", db_names);
    }

    Ok(client.database(&db_name))
}

fn configure_remote() -> String {

    println!("configure remote mongo db");

    let user_name = env::var(MONGODB_USERNAME)
        .expect(&format!("You must set the {} environment var!", MONGODB_USERNAME));
    
    let password = env::var(MONGODB_PASSWORD)
        .expect(&format!("You must set the {} environment var!", MONGODB_PASSWORD));

    let protocol = env::var(MONGODB_PROTOCOL)
      .expect(&format!("You must set the {} environment var!", MONGODB_PROTOCOL));

    let uri = env::var(MONGODB_URI)
      .expect(&format!("You must set the {} environment var!", MONGODB_URI));
    
    let conn_str = format!("{}://{}:{}@{}/", protocol, user_name, password, uri);
    
    println!("conn string: {}", conn_str);

    conn_str
}

fn configure_local(port: String) -> String {

    println!("configure local mongo db for port: {}", port);

    let protocol = env::var(MONGODB_PROTOCOL)
      .expect(&format!("You must set the {} environment var!", MONGODB_PROTOCOL));

    let uri = env::var(MONGODB_URI) 
        .expect(&format!("You must set the {} environment var!", MONGODB_URI));
    
    let conn_str = format!("{}://{}:{}/", protocol, uri, port);

    println!("conn string: {}", conn_str);

    conn_str    
}