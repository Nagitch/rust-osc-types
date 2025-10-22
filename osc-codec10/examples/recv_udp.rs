use osc_codec10::{decode_bundle, decode_message};
use osc_types10::OscPacket;
use std::net::UdpSocket;

fn count_items_in_bundle(bundle: &osc_types10::Bundle) -> (usize, usize) {
    let mut messages = 0;
    let mut bundles = 0;
    for packet in &bundle.packets {
        match packet {
            OscPacket::Message(_) => messages += 1,
            OscPacket::Bundle(nested) => {
                bundles += 1;
                let (nested_msgs, nested_bundles) = count_items_in_bundle(nested);
                messages += nested_msgs;
                bundles += nested_bundles;
            }
        }
    }
    (messages, bundles)
}

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
                let (total_messages, total_bundles) = count_items_in_bundle(&bundle);
                println!(
                    "bundle timetag={} direct_packets={} total_messages={} total_bundles={}",
                    bundle.timetag,
                    bundle.packets.len(),
                    total_messages,
                    total_bundles
                );
            }
            _ => eprintln!("unknown packet"),
        }
    }
}
