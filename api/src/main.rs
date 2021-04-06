use actix_web::{middleware, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv;

mod api;
mod auth;
mod database;
mod todos;

use todos::todo_service::ToDoService;

#[derive(Debug)]
pub struct AppState {
    todo_service: ToDoService,
    jwks: auth::jwks::JWKS,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv();
    let database = database::init().await.unwrap();
    let authority = std::env::var("AUTHORITY").expect("AUTHORITY must be set");

    let jwks = auth::jwks::fetch_jwks_async(&format!(
        "{}{}",
        authority.as_str(),
        ".well-known/jwks.json"
    ))
    .await
    .unwrap_or(auth::jwks::JWKS { keys: vec![] });

    HttpServer::new(move || {
        let todo_service = ToDoService::new(database.collection("todos"));

        App::new()
            .wrap(HttpAuthentication::bearer(auth::validator))
            .wrap(middleware::Logger::default())
            .data(AppState {
                todo_service,
                jwks: jwks.clone(),
            })
            .service(web::scope("/api").configure(api::register))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
