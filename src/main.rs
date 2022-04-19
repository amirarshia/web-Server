use actix_files as fs;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde_derive::{Deserialize};
use regex::Regex;


#[derive(Deserialize)]
struct Register {
    email: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Web server started");
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
    
    fn remove_whitespace(s: &str) -> String {
        s.chars().filter(|c| !c.is_whitespace()).collect()
    }
    
    let email_refined = remove_whitespace(&request_body.email);
    
    let re = Regex::new(r"(?x)
    ^(?P<login>[^@\s]+)@
    ([[:word:]]+\.)+
    [[:word:]]+$
    ").unwrap();
    

    if re.is_match(&email_refined) == true {
        println!("The email is formatted correctly");
        HttpResponse::Found()
            .append_header(("Location", "/register.html"))
            .finish()
    } else {
        println!("Email isn't formatted correctly");
        HttpResponse::Found()
            .append_header(("Location", "/400.html"))
            .finish()
    }


}
