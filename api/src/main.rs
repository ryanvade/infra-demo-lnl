use actix_web::{middleware, web, App, HttpServer, http};
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv;
use actix_cors::Cors;

mod api;
mod auth;
mod database;
mod todos;
mod app;

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

    println!("App Starting");

    HttpServer::new(move || {
        let todo_service = ToDoService::new(database.collection("todos"));

        // TODO: Make not permissive
        let cors = Cors::permissive().allow_any_origin();

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .data(AppState {
                todo_service,
                jwks: jwks.clone(),
            })
            .service(web::scope("/api").wrap(HttpAuthentication::bearer(auth::validator)).configure(api::register))
            .service(web::scope("").configure(app::register))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
