use std::io::{BufRead, BufReader, Write};
use std::net::*;








struct Host {
    socket_addr: SocketAddr
}

struct Response {

}



pub fn start_host() {
    let mut stream = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in stream.incoming() {
       handle_connection(&stream.unwrap());
    }
}

fn handle_connection(mut stream: &TcpStream) {
    let stream_reader = BufReader::new(stream);
    let request: Vec<_> = stream_reader
        .lines()
        .map(|l| l.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("request: {:#?}", request);
    let request_line = request.get(0).unwrap().to_string();
    println!("request_line: {}", request_line);


}