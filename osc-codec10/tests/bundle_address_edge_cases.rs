use osc_codec10::{decode_bundle, decode_message, encode_bundle, encode_message};
use osc_types10::{Bundle, Message, OscPacket, OscType};

#[test]
fn bundle_vs_message_with_bundle_address() {
    // Test that messages with "#bundle" address are not misclassified as bundles

    // Create a message with "#bundle" as the address
    let bundle_address_msg = Message::new("#bundle", vec![OscType::String("not a bundle")]);

    // Create a bundle containing this message
    let bundle = Bundle::with_messages(42, vec![bundle_address_msg.clone()]);

    // Encode and decode the bundle
    let encoded = encode_bundle(&bundle);
    let (decoded_bundle, used) = decode_bundle(&encoded).unwrap();

    assert_eq!(used, encoded.len());
    assert_eq!(decoded_bundle.timetag, 42);
    assert_eq!(decoded_bundle.packets.len(), 1);

    // Verify that the message with "#bundle" address was correctly decoded as a message
    if let OscPacket::Message(ref msg) = decoded_bundle.packets[0] {
        assert_eq!(msg.address, "#bundle");
        assert_eq!(msg.args.len(), 1);
        if let OscType::String(s) = &msg.args[0] {
            assert_eq!(*s, "not a bundle");
        } else {
            panic!("Expected string argument");
        }
    } else {
        panic!("Expected message, not bundle");
    }
}

#[test]
fn message_with_bundle_prefix_address() {
    // Test various addresses that start with "#bundle" but are still messages
    let test_cases = vec![
        "#bundle/sub/path",
        "#bundle_extended",
        "#bundled",
        "#bundle123",
    ];

    for address in test_cases {
        let msg = Message::new(address, vec![OscType::Int(123)]);
        let bundle = Bundle::with_messages(100, vec![msg.clone()]);

        let encoded = encode_bundle(&bundle);
        let (decoded_bundle, _) = decode_bundle(&encoded).unwrap();

        assert_eq!(decoded_bundle.packets.len(), 1);
        if let OscPacket::Message(ref decoded_msg) = decoded_bundle.packets[0] {
            assert_eq!(decoded_msg.address, address);
            assert_eq!(decoded_msg.args.len(), 1);
        } else {
            panic!("Expected message for address: {}", address);
        }
    }
}

#[test]
fn actual_nested_bundle_still_works() {
    // Ensure that real nested bundles still work correctly
    let inner_msg = Message::new("/real/message", vec![OscType::Float(3.14)]);
    let inner_bundle = Bundle::with_messages(200, vec![inner_msg]);

    let outer_msg = Message::new("#bundle", vec![OscType::String("confusing message")]);
    let mut outer_bundle = Bundle::empty(100);
    outer_bundle.add_message(outer_msg);
    outer_bundle.add_bundle(inner_bundle.clone());

    let encoded = encode_bundle(&outer_bundle);
    let (decoded_bundle, _) = decode_bundle(&encoded).unwrap();

    assert_eq!(decoded_bundle.timetag, 100);
    assert_eq!(decoded_bundle.packets.len(), 2);

    // First should be the confusing message
    if let OscPacket::Message(ref msg) = decoded_bundle.packets[0] {
        assert_eq!(msg.address, "#bundle");
    } else {
        panic!("Expected message at index 0");
    }

    // Second should be the actual nested bundle
    if let OscPacket::Bundle(ref bundle) = decoded_bundle.packets[1] {
        assert_eq!(bundle.timetag, 200);
        assert_eq!(bundle.packets.len(), 1);
    } else {
        panic!("Expected bundle at index 1");
    }
}

#[test]
fn roundtrip_individual_message_with_bundle_address() {
    // Test that a standalone message with "#bundle" address can be encoded/decoded correctly
    let msg = Message::new("#bundle", vec![OscType::String("standalone")]);

    let encoded = encode_message(&msg);
    let (decoded_msg, used) = decode_message(&encoded).unwrap();

    assert_eq!(used, encoded.len());
    assert_eq!(decoded_msg.address, "#bundle");
    assert_eq!(decoded_msg.args.len(), 1);
    if let OscType::String(s) = &decoded_msg.args[0] {
        assert_eq!(*s, "standalone");
    } else {
        panic!("Expected string argument");
    }
}
