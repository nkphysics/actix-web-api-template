use actix_web::{get, post, delete, put, web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use std::sync::Mutex;


struct Appstate {
    entries: Mutex<Vec<ListEntry>>
}

#[derive(Serialize, Deserialize, Clone)]
struct ListEntry {
    id: u32,
    entry: String,
}

#[derive(Deserialize, Clone)]
pub struct InsertEntrydata {
    pub entry: String,
}

#[derive(Deserialize, Clone)]
pub struct AllInsertEntryData {
    pub entries: Vec<InsertEntrydata>,
}

#[derive(Deserialize, Clone)]
pub struct DeleteEntryData {
    pub ids: Vec<u32>,
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
                      entry_info: web::Json<AllInsertEntryData>) -> impl Responder {
    let mut entries = data.entries.lock().unwrap();
    let mut maxid: u32 = 0;
    for i in 0..entries.len() {
        if entries[i].id > maxid{
            maxid = entries[i].id;
        }
    }
    for i in 0..entry_info.entries.len(){
        entries.push(ListEntry{
            id: maxid + 1,
            entry: entry_info.entries[i].entry.clone(),
        });
        maxid += 1;
    }

    HttpResponse::Ok().json(entries.to_vec())
}

#[delete("/list/entries")]
async fn delete_entry(data: web::Data<Appstate>,
                      idvec: web::Json<DeleteEntryData>) -> impl Responder{
    let mut entries = data.entries.lock().unwrap();
    for i in 0..idvec.ids.len(){
        let id = idvec.ids[i];
        *entries = entries.to_vec().into_iter().filter(|x| x.id != id).collect();
    }
    HttpResponse::Ok().json(entries.to_vec())
}

#[put("/list/entries/{id}")]
async fn update_entry(data: web::Data<Appstate>,
                      id: web::Path<u32>,
                      entry_info: web::Json<UpdateEntrydata>) -> impl Responder{
    let mut entries = data.entries.lock().unwrap();
    let id = id.into_inner();
    for i in 0..entries.len() {
        if entries[i].id == id {
            entries[i].entry = entry_info.entry.clone();
            break;
        }
    }

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
            .service(delete_entry)
            .service(update_entry)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}