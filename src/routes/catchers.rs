use rocket::serde::json::{Value, json};

#[catch(404)]
pub fn catch_not_found() -> Value {
    json!("Not Found!")
}

#[catch(401)]
pub fn catch_unauthorized() -> Value {
    json!("Unauthorized!")
}