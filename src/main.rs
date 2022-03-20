use std::fs;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::path::Path;

fn main() {
    println!("Hello World");
    let listener = TcpListener::bind("0.0.0.0:80").unwrap();
    for stream in listener.incoming() {
        println!("Incoming Connection [+]");
        let mut stream = stream.unwrap();
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(_) => {}
            Err(_) => return,
        }
        println!("Request:\r\n{} ", String::from_utf8_lossy(&buffer[..]));
        let request = String::from_utf8_lossy(&buffer[..]);
        let request_line = request.lines().next().unwrap();
        let mut parts = request_line.split_whitespace();
        let vc = request_line.split_whitespace().collect::<Vec<&str>>();
        if vc.len() < 2 {
            continue;
        }
        let method = parts.next().ok_or("Method not specified").unwrap();
        let uri = Path::new(parts.next().ok_or("URI not specified").unwrap());
        let uri_string = uri.as_os_str().to_str().unwrap();
        println!("{}", method);
        println!("{}", uri_string);
        let home_content = String::from(
            fs::read_to_string(&"./www/home.html")
                .unwrap_or(format!("reading file content failed [-]")),
        );
        let index_content = String::from(
            fs::read_to_string(&"./www/index.html")
                .unwrap_or(format!("reading file content failed [-]")),
        );
        let four04_content = String::from(
            fs::read_to_string(&"./www/404.html")
                .unwrap_or(format!("reading file content failed [-]")),
        );
        let mut header = format!("{} {}", "HTTP/1.1", http::StatusCode::OK);
        let mut body = String::from("");
        match uri_string {
            "/" => body = index_content,
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
