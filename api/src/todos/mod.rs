use actix_web::{web, HttpResponse, Responder};
use bson::{doc, Bson};
use chrono::prelude::*;
use serde::Deserialize;
use serde_json::json;
use std::string::ToString;

pub mod todo;
pub mod todo_service;

#[derive(Deserialize, Debug)]
pub struct CreateTodoRequest {
    pub descr: String,
}

#[derive(Deserialize, Debug)]
pub struct ToDoPaginationOptions {
    #[serde(rename = "lastId")]
    last_id: Option<String>,
}

pub async fn create_todo(
    request: web::Json<CreateTodoRequest>,
    app_state: web::Data<crate::AppState>,
) -> impl Responder {
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
                _ => panic!("_id is not an ObjectId"),
            };
            return HttpResponse::Created().json(json!({ "id": id.to_string() }));
        }
        Err(err) => {
            println!("Failed: {}", err);
            return HttpResponse::InternalServerError().finish();
        }
    }
}

pub async fn get_todo(
    path: web::Path<(String,)>,
    app_state: web::Data<crate::AppState>,
) -> impl Responder {
    let id = path.into_inner().0;
    println!("Id: {}", id);
    let todo_service = &app_state.todo_service;

    match todo_service.get(&id).await {
        Ok(result) => {
            if result.is_none() {
                return HttpResponse::NotFound().finish();
            }
            let todo = result.unwrap();
            return HttpResponse::Ok().json(todo.to_json());
        }
        Err(err) => {
            println!("Err: {:#?}", err);
            return HttpResponse::InternalServerError().finish();
        }
    }
}

pub async fn delete_todo(
    path: web::Path<(String,)>,
    app_state: web::Data<crate::AppState>,
) -> impl Responder {
    let id = path.into_inner().0;
    let todo_service = &app_state.todo_service;

    match todo_service.delete(&id).await {
        Ok(result) => {
            if result.is_none() {
                return HttpResponse::NotFound().finish();
            }
            return HttpResponse::NoContent().finish();
        }
        Err(err) => {
            println!("Err: {:#?}", err);
            return HttpResponse::InternalServerError().finish();
        }
    }
}

pub async fn list_todos(
    query: web::Query<ToDoPaginationOptions>,
    app_state: web::Data<crate::AppState>,
) -> impl Responder {
    let todo_service = &app_state.todo_service;

    let mut last_id: Option<&String> = None;
    if let Some(query_last_id) = &query.last_id {
        last_id = Some(query_last_id);
    }

    match todo_service.list(last_id).await {
        Ok(results) => {
            if results.is_none() {
                return HttpResponse::BadRequest().finish();
            }
            let results = results.unwrap();
            let results: Vec<serde_json::Value> =
                results.into_iter().map(|t| t.to_json()).collect();
            return HttpResponse::Ok().json(json!({ "items": results }));
        }
        Err(err) => {
            println!("Err: {:#?}", err);
            return HttpResponse::InternalServerError().finish();
        }
    }
}
