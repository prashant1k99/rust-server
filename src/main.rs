use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("connection established");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|lines| !lines.is_empty())
        .collect();

    println!("Request: {http_request:#?}");

    // Now to respond with 200 OK.
    let response = "HTTP/1.1 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}
