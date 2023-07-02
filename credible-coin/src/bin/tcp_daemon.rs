use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read_exact(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}

pub fn main() {
    let listener = TcpListener::bind("127.0.0.1:2300").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
