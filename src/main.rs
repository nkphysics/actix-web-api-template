use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize};


#[derive(Serialize)]
struct Status {
    pub status: String,
}

#[get("/status")]
async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status{
        status: "Up".to_string(),
    })
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(status)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}