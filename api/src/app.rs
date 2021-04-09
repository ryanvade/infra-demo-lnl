use actix_web::{ web, Responder, HttpResponse };
use actix_web::http::StatusCode;
use actix_files;
use std::io::Read;
use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use std::path::PathBuf;

pub async fn app()  -> impl Responder{
    let mut files = actix_files::NamedFile::open("./dist/index.html").unwrap();
    let mut body: String = "".to_string();
    files.read_to_string(&mut body).unwrap();
    return HttpResponse::build(StatusCode::from_u16(200).unwrap()).header("Content-Type", "text/html").body(body);
}

async fn assets(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(format!("./dist/{}", path.to_str().unwrap()))?)
}



pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(app)),
    );
    cfg.service(
        web::resource("/app")
            .route(web::get().to(app)),
    );
    cfg.service(
        web::resource("/{filename:.*}")
            .route(web::get().to(assets)),
    );
}