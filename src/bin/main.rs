use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs;
use std::thread;
use std::time::Duration;
use rust_web_server::ThreadPool;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // use thread pool to handle multiple requests at a 
    // time but with the restrictions of pre-defined number of threads
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
        pool.execute(|| {
            handle_connection(stream);
        });       
    }
    println!("Shutting down!");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    
    stream.read(&mut buffer).unwrap();

    // check that the browser is requesting / before returning the HTML 
    // file and return an error if the browser requests anything else.
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename)  = if buffer.starts_with(get){
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        // Simulate time intensive request
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } 
    else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}