use std::io;

use actix_web::{web::{self, Form,}, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;
use std::net::TcpListener;

#[derive(serde::Deserialize)]
struct FormData{
    email: String,
    name: String
}

async fn health_checker() -> impl Responder {
    HttpResponse::Ok()
}

async fn subscribe(form: web::Form<FormData>) -> HttpResponse {

    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_checker))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)? // error occurs quit
    .run();
    
    Ok(server)
}