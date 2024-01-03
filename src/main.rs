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
    let request_path = request_line.split_whitespace().collect::<Vec<_>>()[1]
        .split_at(1)
        .1;

    let (status_line, content) = get(request_path);
    let lenght = content.len();

    let response = format!("{status_line}\r\nContent-Length: {lenght}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn get(route_path: &str) -> (&'static str, String) {
    let routes = fs::read_dir("./src/routes").unwrap();
    let mut file = String::new();

    if route_path == "" {
        file = fs::read_to_string("src/routes/index.html").unwrap();
    } else {
        for route in routes {
            let route = route.unwrap();
            let route_filename = route.file_name().into_string().unwrap();

            match route_filename == route_path {
                true => {
                    let path: String = format!("src/routes/{}/index.html", route_filename);
                    match fs::read_to_string(path) {
                        Ok(contents) => file = contents,
                        Err(e) => {
                            eprintln!("Error reading file: {}", e);
                        }
                    };
                    break;
                }
                false => {}
            }
        }
    }

    if !file.is_empty() {
        return ("HTTP/1.1 200 OK", file);
    } else {
        return (
            "HTTP/1.1 404 NOT FOUND",
            fs::read_to_string("src/routes/404.html").unwrap(),
        );
    }
}
