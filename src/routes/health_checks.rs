use rocket_okapi::openapi;
use mongodb::Database;
use rocket::{serde::json::Json, State};

#[openapi(tag = "health")]
#[get("/ready")]
pub async fn ready(db: &State<Database>) -> Json<String> {
    print_dbs(db).await;
    Json(String::from("ready"))
}

#[openapi(tag = "health")]
#[get("/healthy")]
pub async fn healthy(db: &State<Database>) -> Json<String> {
    print_dbs(db).await;
    Json(String::from("healthy"))
}

pub async fn print_dbs(db: &State<Database>) {
    print!("HealthCheck promt: ");
    for db_names in  db.list_collection_names(None).await.unwrap() {
        print!("{}, ", db_names);
    }
    println!("");
}