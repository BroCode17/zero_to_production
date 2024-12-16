use std::io;

use actix_web::{web::{self,}, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;

async fn health_checker() -> impl Responder {
    HttpResponse::Ok()
}


pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_checker))
    })
    .bind("127.0.0.1:8000")? // error occurs quit
    .run();
    
    Ok(server)
}