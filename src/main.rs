use std::fs;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::path::Path;
use http::StatusCode;
 
 
fn main() {
    println!("Hello World");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
 
    for stream in listener.incoming() {
        println!("Incoming Connection [+]");
        let mut stream = stream.unwrap();
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        println!("Requeast:\r\n{} ", String::from_utf8_lossy(&buffer[..]));
        let request = String::from_utf8_lossy(&buffer[..]);
        let request_line = request.lines().next().unwrap();
        let mut parts = request_line.split_whitespace();
        let method = parts.next().ok_or("Method not specified").unwrap();
        let uri = Path::new(parts.next().ok_or("URI not specified").unwrap());
        let uri_string = uri.as_os_str().to_str().unwrap();
        println!("{}", method);
        println!("{}", uri_string);
        let home_content = String::from(
            fs::read_to_string(&"./www/home.html")
                .unwrap_or(format!("reading file content failed [-]")),
        );
        let four04_content = String::from(
            fs::read_to_string(&"./www/404.html")
                .unwrap_or(format!("reading file content failed [-]")),
        );
        let mut header = format!("{} {}", "HTTP/1.1", http::StatusCode::OK);
        let mut body = String::from("");
        match uri_string {
            "/" => body = home_content,
            "/home" => body = home_content,
            _ => {
                body = four04_content;
                header = format!("{} {}", "HTTP/1.1", http::StatusCode::NOT_FOUND)
            }
        }
 
        let response = format!("{} \r\n\r\n {}\r\n", header, body);
 
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}