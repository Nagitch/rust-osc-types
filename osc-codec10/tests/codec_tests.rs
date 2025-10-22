use osc_codec10::{decode_bundle, decode_message, encode_bundle, encode_message};
use osc_types10::{Bundle, Message, OscType};

#[test]
fn roundtrip_message_basic() {
    let msg = Message::new(
        "/synth/volume",
        vec![
            OscType::Float(0.5),
            OscType::String("foo"),
            OscType::Int(42),
        ],
    );
    let bytes = encode_message(&msg);
    let (m2, used) = decode_message(&bytes).unwrap();
    assert_eq!(used, bytes.len());
    assert_eq!(m2.address, "/synth/volume");
    assert_eq!(m2.args.len(), 3);
}

#[test]
fn roundtrip_blob_padding() {
    let blob = [1u8, 2, 3, 4, 5]; // 5 -> pad 3
    let msg = Message::new("/blob", vec![OscType::Blob(&blob)]);
    let bytes = encode_message(&msg);
    let (m2, used) = decode_message(&bytes).unwrap();
    assert_eq!(used, bytes.len());
    match &m2.args[0] {
        OscType::Blob(b) => assert_eq!(*b, &blob),
        _ => panic!(),
    }
}

#[test]
fn roundtrip_bundle_basic() {
    let msg = Message::new("/ping", vec![OscType::Int(7)]);
    let b = Bundle::with_messages(1u64, vec![msg]);
    let bytes = encode_bundle(&b);
    let (b2, used) = decode_bundle(&bytes).unwrap();
    assert_eq!(used, bytes.len());
    assert_eq!(b2.timetag, 1);
    assert_eq!(b2.packets.len(), 1);
}
