use std::net::UdpSocket;
use osc_codec10::{decode_message, decode_bundle};

fn main() -> std::io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:9000")?;
    println!("listening on 9000");
    let mut buf = [0u8; 65536];
    loop {
        let (n, _src) = sock.recv_from(&mut buf)?;
        let data = &buf[..n];
        // very simple heuristic: leading byte '/' => message, '#' => bundle
        match data.first().copied() {
            Some(b'/') => {
                let (msg, used) = decode_message(data).expect("decode message");
                assert_eq!(used, data.len());
                println!("msg {} args={}", msg.address, msg.args.len());
            }
            Some(b'#') => {
                let (bundle, _used) = decode_bundle(data).expect("decode bundle");
                println!("bundle timetag={} messages={}", bundle.timetag, bundle.messages.len());
            }
            _ => eprintln!("unknown packet"),
        }
    }
}
