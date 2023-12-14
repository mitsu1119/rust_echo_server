use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    str, thread,
};

use crate::errors::ServerError;

type Result<T> = std::result::Result<T, ServerError>;

pub fn server(address: &str) -> Result<()> {
    let listener = TcpListener::bind(address)?;

    loop {
        let (stream, src) = listener.accept()?;
        println!("accept: {}", src);
        thread::spawn(move || {
            handler(stream, src).expect("Handler Error:");
        });
    }
}

fn handler(mut stream: TcpStream, src: SocketAddr) -> Result<()> {
    const MAX_BUFFER: usize = 1024;
    let mut buffer = [0u8; MAX_BUFFER];
    loop {
        let nbytes = stream.read(&mut buffer)?;
        if nbytes == 0 {
            println!("Connection closed: {}", src);
            return Ok(());
        }

        let raw_message = &buffer[..nbytes];

        if let Ok(s) = str::from_utf8(raw_message) {
            print!("{}", s);
        } else {
            print!("{:?}", raw_message);
        }

        stream.write_all(raw_message)?;
    }
}
