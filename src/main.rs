use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

const DEFAULT_IP: &str = "127.0.0.1";
const DEFAULT_PORT: u32 = 6379;

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0u8; 1024];

    loop {
        match stream.read(&mut buf) {
            Ok(0) => {
                break;
            }
            Ok(_n) => {
                if let Err(err) = stream.write_all(b"+PONG\r\n") {
                    eprint!("write error: {err}");
                    break;
                }
            }
            Err(err) => {
                eprint!("read error: {err}");
                break;
            }
        }
    }
}

fn main() {
    let addr = format!("{DEFAULT_IP}:{DEFAULT_PORT}");
    let listener = TcpListener::bind(addr).expect("failed to bind to 127.0.0.0");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
