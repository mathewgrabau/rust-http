use std::{net::TcpListener, io::Read, io::Write, fs::read};

fn main() {
    const ADDR: &str = "127.0.0.1";
    const PORT: i32 = 3000;
    let bind_address = format!("{}:{}", ADDR, PORT);
    let listener_result = TcpListener::bind(bind_address.as_str());
    if listener_result.is_ok() {
        println!("Server listening on {}", bind_address.as_str());
        let listener = listener_result.unwrap();

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Server established a connection");
            let mut buffer = [0; 1024];
            // Echo
            let read_result = stream.read(&mut buffer);
            match read_result {
                Ok(_) => {
                    let write_result = stream.write(&mut buffer);
                    match write_result {
                        Ok(_) => {
                            // Nothing to do here
                        },
                        Err(e) => {
                            eprintln!("Write error:\n{}", e);
                        },
                    }
                },
                Err(e) => {
                    eprintln!("Read error: {}", e);
                }
            }
        }
    } else {
        eprintln!("bind() failed:\n{}", listener_result.unwrap_err())
    }
}
