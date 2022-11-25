use std::net::TcpListener;

fn main() {
    const ADDR: &str = "127.0.0.1";
    const PORT: i32 = 3000;
    let bind_address = format!("{}:{}", ADDR, PORT);
    let listener_result = TcpListener::bind(bind_address.as_str());
    if listener_result.is_ok() {
        println!("Server listening on {}", bind_address.as_str());
        let listener = listener_result.unwrap();

        for stream in listener.incoming() {
            let _stream = stream.unwrap();
            println!("Server established a connection");
        }
    } else {
        eprintln!("bind() failed:\n{}", listener_result.unwrap_err())
    }
}
