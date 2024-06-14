mod db;
use actix_web::{get, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
struct User {
    name: String,
}

#[get("/greet/{id}")]
async fn greet(user_name: web::Path<String>) -> impl Responder {
    format!("Hello World {user_name}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let port = 8080;
    println!("Starting https:://localhost:8080");

    HttpServer::new(|| App::new().service(greet))
            .bind(("127.0.0.1", port))?
            .workers(2)
            .run()
            .await

}