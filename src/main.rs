use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let addr: &str = "127.0.0.1:8080";
    let listener: TcpListener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();

        handle_request(stream);
    }
}

fn handle_request(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_option = buf_reader.lines().next();

    if request_option.is_none() {
        return;
    }

    let request_line = request_option.unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let content = fs::read_to_string(filename).unwrap();
    let lenght = content.len();

    let response = format!("{status_line}\r\nContent-Length: {lenght}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
}
