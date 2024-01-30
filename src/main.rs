#[macro_use] extern crate rocket;

use dotenv::dotenv;
use rocket_okapi::openapi_get_routes;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

use rocket_app2::mongo_db;
use rocket_app2::person_repository::*;
use rocket_app2::fairings;

mod routes;

#[rocket::main]
async fn main() {

    dotenv().ok();
    //let mongo_db = mongo_db::connect().await.unwrap();
    //let person_service: PersonMongoRerpository = PersonMongoRerpository::new(mongo_db);

    let _ = rocket::build()
        //.attach(mongo_db::init())
        .attach(fairings::cors::CORS)
        //.manage(person_service)
        .mount("/api", openapi_get_routes![
            routes::person::get_all_persons,
            routes::person::get_person, 
            routes::person::create_person,
            routes::person::update_person,
            routes::person::delete_person
            ])
        .register("/api", catchers![
            routes::catchers::catch_not_found,
            routes::catchers::catch_unauthorized])
        .mount(
            "/api/api-docs",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .launch()
        .await;
}
