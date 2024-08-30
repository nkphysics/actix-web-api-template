use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use std::sync::Mutex;


struct Appstate {
    entries: Mutex<Vec<ListEntry>>
}

#[derive(Serialize, Deserialize, Clone)]
struct ListEntry {
    id: i32,
    entry: String,
}

#[derive(Deserialize, Clone)]
pub struct InsertEntrydata {
    pub entry: String,
}

#[derive(Deserialize, Clone)]
struct UpdateEntrydata {
    pub entry: String,
}

#[derive(Serialize)]
struct Status {
    pub status: String,
}

#[get("/list/entries")]
async fn get_list(data: web::Data<Appstate>) -> impl Responder {
    HttpResponse::Ok().json(data.entries.lock().unwrap().to_vec())
}

#[post("/list/entries")]
async fn insert_entry(data: web::Data<Appstate>,
                      entry_info: web::Json<InsertEntrydata>) -> impl Responder {
    let mut entries = data.entries.lock().unwrap();
    let mut maxid: i32 = 0;
    for i in 0..entries.len() {
        if entries[i].id > maxid{
            maxid = entries[i].id;
        }
    }
    entries.push(ListEntry{
        id: maxid + 1,
        entry: entry_info.entry.clone(),
    });

    HttpResponse::Ok().json(entries.to_vec())
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
    let app_data = web::Data::new(Appstate {entries: Mutex::new(vec![])});
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .service(status)
            .service(get_list)
            .service(insert_entry)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}