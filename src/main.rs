use actix_files::Files;
use actix_web::error::Error;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use std::io;
use std::process::Command;

#[get("/")]
async fn index() -> impl Responder {
    let html = std::fs::read_to_string("public/index.html").unwrap();
    HttpResponse::Ok().content_type("text/html").body(html)
}

#[post("/execute-command")]
async fn execute_command() -> Result<HttpResponse, Error> {
    let output = Command::new("echo")
        .arg("Hello from the command!")
        .output()
        .expect("Failed to execute command");

    let output_str = String::from_utf8(output.stdout).map_err(|e| {
        actix_web::error::InternalError::new(e, actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
    })?;
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(output_str))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(Files::new("/public", "public/"))
            .service(Files::new("/images", "images/"))
            .service(execute_command)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
