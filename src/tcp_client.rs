use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    str,
};

use crate::errors::ClientError;

type Result<T> = std::result::Result<T, ClientError>;

pub fn connect(address: &str) -> Result<()> {
    let mut stream = TcpStream::connect(address)?;

    println!("Connect to server {}", address);

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        stream.write_all(input.as_bytes())?;

        let mut reader = BufReader::new(&stream);
        let mut buffer = Vec::new();
        reader.read_until(b'\n', &mut buffer)?;

        if let Ok(s) = str::from_utf8(&buffer) {
            print!("{}", s);
        } else {
            print!("{:?}", &buffer);
        }
    }
}
