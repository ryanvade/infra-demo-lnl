use actix_web::{middleware, web, App, HttpServer};

mod api;
mod database;
mod todos;

use todos::todo_service::ToDoService;

pub struct AppState {
    todo_service: ToDoService,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database = database::init().await.unwrap();

    HttpServer::new(move || {
        let todo_service = ToDoService::new(database.collection("todos"));

        App::new()
            .wrap(middleware::Logger::default())
            .data(AppState { todo_service })
            .service(web::scope("/api").configure(api::register))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
