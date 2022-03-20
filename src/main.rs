use std::fs;
use std::io::Read;
use std::io::Write;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;
use std::path::Path;
use std::ptr;

fn main() {
    println!("Hello World");
    let listener = TcpListener::bind("0.0.0.0:80").unwrap();
    for conn in listener.incoming() {
        println!("Incoming Connection [+]");
        let mut uri = "";
        let mut stream;
        match conn {
            Ok(t) => stream = t,
            _ => return,
        };
        let mut ip: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 80);
        match stream.peer_addr() {
            Ok(i) => ip = i,
            _ => uri = "/500",
        }
        println!("IP: {}", ip);
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(_) => {}
            Err(_) => uri = "/500",
        }
        println!("Request:\r\n{} ", String::from_utf8_lossy(&buffer[..]));
        let request = String::from_utf8_lossy(&buffer[..]);
        let mut request_line = "";
        match request.lines().next() {
            Some(r) => request_line = r,
            None => uri = "/500",
        }
        let mut parts = request_line.split_whitespace();
        let mut method = "";
        match parts.next() {
            Some(p) => method = p,
            None => {
                println!("Method not specified");
                uri = "/500"
            }
        }
        match parts.next() {
            Some(p) => uri = p,
            None => {
                println!("URI not specified");
                uri = "/500"
            }
        }
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
        let five00_content = String::from(
            fs::read_to_string(&"./www/500.html")
                .unwrap_or(format!("reading file content failed [-]")),
        );
        let mut header = format!("{} {}", "HTTP/1.1", http::StatusCode::OK);
        let mut body = String::from("");
        match uri {
            "/" => body = index_content,
            "/home" => body = home_content,
            "/500" => body = five00_content,
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
