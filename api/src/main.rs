use actix_files;
use actix_web::{ middleware, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

mod todo;
mod database;

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub struct AppState {
    todo_service: todo::ToDoService
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database = database::init().await.unwrap();



    HttpServer::new(move || {
        let todo_service = todo::ToDoService::new(database.collection("todos"));

        App::new()
            .wrap(middleware::Logger::default())
            .data(AppState {
                todo_service
            })
            .service(
                web::scope("/api").configure(todo::register)
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}