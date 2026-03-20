use std::{
    io::{Read, Result},
    net::{TcpListener, TcpStream},
};

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    loop {
        let mut len_buf = [0u8; 4];

        if stream.read_exact(&mut len_buf).is_err() {
            return Ok(());
        }

        let msg_len = u32::from_be_bytes(len_buf) as usize;

        let mut buf = vec![0u8; msg_len];
        stream.read_exact(&mut buf)?;

        let msg: String = serde_json::from_slice(&buf)?;
        println!("Received message:: {}", msg);
    }
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        handle_client(stream?)?;
    }

    Ok(())
}
