use mongodb::{Client, options::{ClientOptions, ResolverConfig}, Database};
use std::env;
use rocket::fairing::AdHoc;

const MONGODB_URI: &str = "MONGODB_URI";
const MONGODB_RUST_APP: &str = "MONGODB_RUST_APP";

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
    let connection_string = env::var(MONGODB_URI)
        .expect(&format!("You must set the {} environment var!", MONGODB_URI));
    let db_name: String = env::var(MONGODB_RUST_APP)
        .expect(&format!("You must set {} environment variable", MONGODB_RUST_APP));

    println!("conn string: {}, DB: {}", connection_string, db_name);
    
    let options = ClientOptions::parse_with_resolver_config(
        connection_string, 
        ResolverConfig::cloudflare())
        .await
        .unwrap();

    let client = Client::with_options(options).unwrap();

    Ok(client.database(&db_name))
}