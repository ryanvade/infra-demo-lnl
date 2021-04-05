use bson::{doc, Bson, DateTime, Document, oid::ObjectId };
use bson::serde_helpers::{ iso_string_as_bson_datetime };
use mongodb::{ Database, Collection, error::Result };
use mongodb::results::{ InsertOneResult };
use serde::{ Serialize, Deserialize };
use actix_web::{ Responder, HttpResponse, web };
use actix_web::dev::Body;
use serde_json::json;
use chrono::prelude::*;
use std::str::{ FromStr };
use std::string::ToString;
use serde::{ ser, Deserializer };

#[derive(Serialize, Deserialize, Debug)]
pub struct ToDo {
    #[serde(rename = "_id")]
   pub id: ObjectId,
   pub descr: String,
   pub created_at: DateTime,
   pub completed: bool
}

impl std::convert::Into<String> for ToDo {
    fn into(self) -> String {
        return String::new();
    }
}

impl std::fmt::Display for ToDo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string_pretty(self);
        if let Ok(json) = json {
            return write!(f, "{}", json);
        }
        let err = json.err().unwrap();
        write!(f, "{:#?}", err)
    }
}

impl std::convert::Into<actix_web::dev::Body> for ToDo {
    fn into(self) -> Body {
        return Body::Bytes(web::Bytes::from(self.to_string()));
    }
}

impl ToDo {
    pub fn to_json(&self) -> serde_json::Value {
        json!({
            "id": self.id.to_string(),
            "descr": self.descr,
            "createdAt": self.created_at.to_rfc3339(),
            "completed": self.completed
        })
    }
}

#[derive(Clone)]
pub struct ToDoService {
    collection: Collection
}

impl ToDoService {
    pub fn new(collection: Collection) -> ToDoService {
        ToDoService { collection }
    }

    pub async fn create(&self, todo: Document) -> Result<InsertOneResult> {
        self.collection.insert_one(todo, None).await
    }

    pub async fn get(&self, id: &String) -> Result<Option<ToDo>> {
        let id = ObjectId::from_str(id);
        if id.is_err() {
            return Ok(None);
        }
        let filter = doc! {
            "_id": id.unwrap()
        };
        let result = self.collection.find_one(filter, None).await;
        if result.is_err() {
            return Err(result.err().unwrap())
        }
        let result = result.unwrap();
        match result {
            None => return Ok(None),
            Some(doc) => {
                let todo = bson::from_document::<ToDo>(doc).unwrap();
                return Ok(Some(todo));
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct CreateTodoRequest {
    pub descr: String
}

pub async fn create_todo(request: web::Json<CreateTodoRequest>, app_state: web::Data<crate::AppState>) -> impl Responder {
    println!("Description: {}", request.descr);
    let todo_service = &app_state.todo_service;

    let doc = doc! {
        "descr": request.descr.clone(),
        "created_at": Bson::DateTime(Utc::now()),
        "completed": false
    };
    match todo_service.create(doc).await {
        Ok(result) => {
            let id = match result.inserted_id {
                Bson::ObjectId(oid) => oid,
                _ => panic!("_id is not an ObjectId")
            };
            return HttpResponse::Created().json(json!({ "id": id.to_string() }))
        }
        Err(err) => {
            println!("Failed: {}", err);
            return HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_todo(path: web::Path<(String, )>, app_state: web::Data<crate::AppState>) -> impl Responder {
    let id = path.into_inner().0;
    println!("Id: {}", id);
    let todo_service = &app_state.todo_service;

    match todo_service.get(&id).await {
        Ok(result) => {
            if result.is_none() {
                return HttpResponse::NotFound().finish()
            }
            let todo = result.unwrap();
            return HttpResponse::Ok().json(todo.to_json())
        }
        Err(err) => {
            println!("Err: {:#?}", err);
            return HttpResponse::InternalServerError().finish()
        }
    }
}

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/todos").route(web::post().to(create_todo))
    );
    cfg.service(
        web::resource("/todos/{id}").route(web::get().to(get_todo))
    );
}