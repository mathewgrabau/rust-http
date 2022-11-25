use std::net::TcpStream;

fn main() {
    const ADDR: &str = "localhost";
    const PORT: i32 = 3000;
    let server_address = format!("{}:{}", ADDR, PORT);
    let connect_result = TcpStream::connect(server_address.as_str());
    if connect_result.is_ok() {
        println!("Server listening on {}", server_address.as_str());
        let _connection = connect_result.unwrap();
    } else {
        eprintln!("bind() failed:\n{}", connect_result.unwrap_err())
    }
}
