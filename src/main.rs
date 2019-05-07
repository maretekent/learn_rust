#[warn(unused_variables)] 
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7879").unwrap();

    for stream in listener.incoming(){
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get){
        let contents = fs::read_to_string("hello.html").unwrap();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        match stream.write(response.as_bytes()){
            Ok(_) => println!("Response content sent"),
            Err(e) => println!("Response failed, error : {}", e),
        }
        stream.flush().unwrap();
    } else  {
        let contents = fs::read_to_string("404.html").unwrap();

        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

        match stream.write(response.as_bytes()){
            Ok(_) => println!("Response sent"),
            Err(e) => println!("Response failed, error : {}", e),
        }
    }
}