use actix_files as fs;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Register {
    email: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(register)
            .service(fs::Files::new("/", "www").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[post("/register")]
async fn register(request_body: web::Form<Register>) -> impl Responder {
    println!("Email Received: {}", request_body.email);
    
    HttpResponse::Found()
        .append_header(("Location", "/register.html"))
        .finish()
}
