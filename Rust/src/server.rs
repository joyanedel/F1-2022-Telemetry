use std::net;

fn handle_client(client: &net::TcpStream) {
    println!("{:?}", client);
    let mut buf: [u8; 4096] = [0; 4096];
    client.peek(&mut buf).expect("Failed to read buffer");

    println!("Address: {}, msg: {:?}", client.peer_addr().unwrap(), buf);
}

fn main() {
    let listener = net::TcpListener::bind("127.0.0.1:21777").unwrap();

    for stream in listener.incoming() {
        handle_client(&stream.expect("a"));
    }
}