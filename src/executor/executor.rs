use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:80")?;
    let msg = b"Hello, my name is client!";
    stream.write(msg)?;
    stream.read(&mut [0; 128])?;
    Ok(())
} // the stream is closed here