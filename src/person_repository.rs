use mongodb::{Database, Collection, bson::{doc, DateTime, oid::ObjectId}};
use rocket::{Data, futures::TryStreamExt};

use crate::person_models::Person;
use serde::{Deserialize, Serialize};

pub struct PersonMongoRerpository {
    db: Database
}

impl PersonMongoRerpository  {

    pub fn new(db: Database) -> PersonMongoRerpository {
        PersonMongoRerpository { db }
    }

    pub async fn get_all(&self) -> Result<Vec<Person>, mongodb::error::Error> {

        let mut cursor = self.get_person_collection()
            .find(None, None)
            .await?;

        let mut persons: Vec<Person> = vec![];

        while let Some(person) = cursor.try_next().await? {
            persons.push(PersonMongoRerpository::map_doc_to_model(person));
        }
        
        Ok(persons)
    }

    pub async fn get_by_id(&self, id: String) -> Result<Option<Person>, mongodb::error::Error> {

        let oid = ObjectId::parse_str(&id);

        if oid.is_err() {
            let e: Result<Option<Person>, mongodb::error::Error> = Err(mongodb::error::Error::custom("oops"));
            return e;
        }

        let person_result = self.get_person_collection()
            .find_one(doc! {"_id": oid.unwrap() }, None)
            .await;
        
        match person_result {
            Ok(person_doc_option) if person_doc_option.is_none() => Ok(Option::None),
            Ok(person_doc_option) => {
                let person = Self::map_doc_to_model(person_doc_option.unwrap());
                Ok(Option::Some(person))
            },
            Err(err) => Err(err)
        }
    }

    pub async fn create(&self, person: Person) -> Result<Option<Person>, mongodb::error::Error> {
        let documet = Self::map_model_to_doc(person);
        let result = self.get_person_collection().insert_one(documet, None).await?;
        self.get_by_id(result.inserted_id.to_string()).await
    }

    fn get_person_collection(&self) -> Collection<PersonDocument> {
        self.db.collection::<PersonDocument>("person")
    }

    fn map_doc_to_model(person: PersonDocument) -> Person {
        let owned = person.to_owned();
        Person::create_from_db(owned._id.unwrap().to_hex(), person.name, person.email)
    }

    fn map_model_to_doc(person: Person) -> PersonDocument {
        PersonDocument {
            _id: Option::None,
            age: person.age,
            email: person.email,
            name: person.name 
        }
    }
    
    //async fn update(person: Person) -> Result<(), T>;
    //async fn delete(id: u32) -> Result<(), T>;
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PersonDocument {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<u32>,
    //pub date_of_birth: Option<DateTime>,
    //pub gender: Option<Gender>,
    pub email: String,
    //pub tasks: Vec<PersonTask>
}