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
    ([[:word:]]+\.)*
    [[:word:]]+$
    ").unwrap();
    
    let mut email_empty = true;

    if email_refined == "" {
        println!("Email was empty");
        HttpResponse::Found()
            .append_header(("Location", "/index.html"))
            .finish();
    } else {
        println!("Email Received: {}", email_refined);
        email_empty = false;
        HttpResponse::Found()
            .append_header(("Location", "/register.html"))
            .finish();
        
    }

    if email_empty == false && re.is_match(&email_refined) == true {
        println!("The email is formatted correctly")
    } else {
        println!("Email isn't formatted correctly")
    }

    HttpResponse::Found()
            .append_header(("Location", "/register.html"))
            .finish()

}
