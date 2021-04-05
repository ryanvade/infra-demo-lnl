use super::todos::{create_todo, delete_todo, get_todo, list_todos};
use actix_web::web;

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/todos")
            .route(web::post().to(create_todo))
            .route(web::get().to(list_todos)),
    );
    cfg.service(
        web::resource("/todos/{id}")
            .route(web::get().to(get_todo))
            .route(web::delete().to(delete_todo)),
    );
}
