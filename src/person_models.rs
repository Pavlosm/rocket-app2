//use chrono::*;
use schemars::JsonSchema;
use rocket::serde::{Serialize, Deserialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct Person {
    pub id: Option<String>,
    pub name: Option<String>,
    pub age: Option<u32>,
    //pub date_of_birth: Option<chrono::DateTime<Utc>>,
    //pub gender: Option<Gender>,
    pub email: String,
    //pub tasks: Vec<PersonTask>
}

impl Person {
    pub fn create_basic(id: &str, name: &str, email: &str) -> Person {
        Person {
            id: Some(String::from(id)),
            name: Some(String::from(name)),
            age: None,
            //date_of_birth: None,
            //gender: None,
            email: String::from(email)
            //tasks: Vec::new()
        }
    }

    pub fn create_from_db(id: String, name: Option<String>, email: String) -> Person {
        Person {
            id: Some(String::from(id)),
            name: name,
            age: None,
            //date_of_birth: None,
            //gender: None,
            email: String::from(email)
            //tasks: Vec::new()
        }
    }
}

#[derive(Serialize)]
pub struct PersonTask {
    pub id: u32,
    pub person_id: u32,
    pub name: String,
    pub description: String
    //pub expires_at: chrono::DateTime<Utc>,
    //pub closed_at: chrono::DateTime<Utc>
}

pub enum Gender {
    Male,
    Female
}

pub enum TaskStatus {
    NotStarted,
    Started,
    Completed,
    Expired,
}