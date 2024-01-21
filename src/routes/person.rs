use rocket::{serde::json::Json, State};
use rocket_app2::{person_repository::*, person_models::Person};
use rocket_app2::api_errors::error::ApiError;
use rocket_okapi::openapi;

#[openapi(tag = "Person")]
#[get("/person")]
pub async fn get_all_persons(person_service: &State<PersonMongoRerpository>) 
    -> Result<Json<Vec<Person>>, ApiError> {
    
    let people_result = person_service.get_all().await;
    
    match people_result {
        Ok(customers) => Ok(Json(customers)),
        Err(_error) => {
            println!("{:?}", _error);
            return Err(ApiError::build(
                400,
                Some(format!("Could not retrive people")),
            ));
        }
    }
}

#[openapi(tag = "Person")]
#[get("/person/<id>")]
pub async fn get_person(
    id: String,
    person_service: &State<PersonMongoRerpository>)
    -> Result<Json<Person>, ApiError> {
    
    let person_result = person_service.get_by_id(id).await;

    match person_result {
        Ok(person_option) if person_option.is_none() => Err(ApiError::build(
            400,
            Some(format!("Could not retrive people")),
        )),
        Ok(person_option) => Ok(Json(person_option.unwrap())),
        Err(_error) => {
            println!("{:?}", _error);
            return Err(ApiError::build(
                400,
                Some(format!("Could not retrive people")),
            ));
        }
    }
}

#[openapi(tag = "Person")]
#[post("/person", format = "json", data = "<person>")]
pub async fn create_person(
    person: Json<Person>,
    person_service: &State<PersonMongoRerpository>) 
    -> Result<Json<Person>, ApiError> {
    
    let create_result = person_service.create(person.0).await;
    
    match create_result {
        Ok(created_person) if created_person.is_some() => Ok(Json(created_person.unwrap())),
        Ok(_) => Err(ApiError::build_with_description(400, "Could not create person")),
        Err(err) => {
            println!("{:?}", err);
            Err(ApiError::build_with_description(400, "Could not create person"))
        }
    }
}

#[openapi(tag = "Person")]
#[put("/person/<id>", format = "json", data = "<person>")]
pub async fn update_person(
    id: String,
    person: Json<Person>,
    person_service: &State<PersonMongoRerpository>) 
    -> Result<Json<Person>, ApiError> {
    let person_clone = person.0.clone();
    let p_clone = person.clone();
    if person.0.id.is_none() || person.0.id.unwrap() != id {
        let err_msg = format!(
            "Could not update person invalid id param {} person {}", id, 
            match p_clone.id.clone() {
                Some(id) => id,
                _ => String::from("None")
            });
        let err: Result<Json<Person>, ApiError> = Err(ApiError::build(400, Some(err_msg)));
        return err;
    }

    let create_result = person_service.update(person_clone.to_owned()).await;

    match create_result {
        Ok(created_person) if created_person.is_some() => Ok(Json(created_person.unwrap())),
        Ok(_) => Err(ApiError::build_with_description(400, "Could not update person")),
        Err(err) => {
            println!("{:?}", err);
            Err(ApiError::build_with_description(400, "Could not update person"))
        }
    }
}

#[openapi(tag = "Person")] 
#[delete("/person/<id>")]
pub async fn delete_person(
    id: String, 
    person_service: &State<PersonMongoRerpository>) 
    -> Result<(), ApiError> {

    let delete_result = person_service.delete_by_id(id).await;
    
    match delete_result {
        Ok(_) => Ok(()),
        Err(_) => Err(ApiError::build_with_description(400, "Could not delete person"))
    }
}