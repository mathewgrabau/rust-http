use std::{net::TcpStream, io::{Write, Read}, str};

fn main() {
    const ADDR: &str = "localhost";
    const PORT: i32 = 3000;
    let server_address = format!("{}:{}", ADDR, PORT);
    let connect_result = TcpStream::connect(server_address.as_str());
    if connect_result.is_ok() {
        println!("Client connected to {}", server_address.as_str());
        let mut stream = connect_result.unwrap();
        let write_result = stream.write("Hello".as_bytes());
        match write_result {
            Ok(count) => { 
                println!("Write {} bytes", count);
                let mut message = [0; 5];
                let read_result = stream.read(&mut message);
                match read_result {
                    Ok(read_count) => {
                        let message_str = str::from_utf8(&message);
                        match message_str {
                            Ok(s) => {
                                println!("Got response from server ({}): {}", read_count, s);
                            },
                            Err(msg_error) => println!("Got response from server ({}), error reading the message: {}", read_count, msg_error),
                        }
                    },
                    Err(read_error) => {
                        eprintln!("Error writing:\n{}", read_error);
                    },
                }
            },
            Err(write_error) => {
                eprintln!("Error writing:\n{}", write_error);
            },
        }
       
    } else {
        eprintln!("bind() failed:\n{}", connect_result.unwrap_err())
    }
}
