use std::net::UdpSocket;
use osc_types10::{Message, OscType};
use osc_codec10::encode_message;

fn main() -> std::io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:0")?;
    let msg = Message::new("/ping", vec![OscType::Int(1)]);
    let bytes = encode_message(&msg);
    sock.send_to(&bytes, "127.0.0.1:9000")?;
    println!("sent /ping");
    Ok(())
}
