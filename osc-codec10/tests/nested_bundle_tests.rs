use osc_codec10::{decode_bundle, encode_bundle};
use osc_types10::{Bundle, Message, OscPacket, OscType};

#[test]
fn roundtrip_nested_bundle() {
    // Create some messages
    let msg1 = Message::new("/synth/freq", vec![OscType::Float(440.0)]);
    let msg2 = Message::new("/synth/amp", vec![OscType::Float(0.5)]);
    let msg3 = Message::new("/control/stop", vec![]);

    // Create an inner bundle with messages
    let inner_bundle = Bundle::with_messages(100, vec![msg1, msg2]);

    // Create an outer bundle containing a message and the inner bundle
    let mut outer_bundle = Bundle::empty(200);
    outer_bundle.add_message(msg3);
    outer_bundle.add_bundle(inner_bundle.clone());

    // Encode and decode
    let bytes = encode_bundle(&outer_bundle);
    let (decoded_bundle, used) = decode_bundle(&bytes).unwrap();

    assert_eq!(used, bytes.len());
    assert_eq!(decoded_bundle.timetag, 200);
    assert_eq!(decoded_bundle.packets.len(), 2);

    // Check the first packet (message)
    if let OscPacket::Message(ref msg) = decoded_bundle.packets[0] {
        assert_eq!(msg.address, "/control/stop");
        assert_eq!(msg.args.len(), 0);
    } else {
        panic!("Expected message at index 0");
    }

    // Check the second packet (nested bundle)
    if let OscPacket::Bundle(ref bundle) = decoded_bundle.packets[1] {
        assert_eq!(bundle.timetag, 100);
        assert_eq!(bundle.packets.len(), 2);

        // Check nested messages
        if let OscPacket::Message(ref msg) = bundle.packets[0] {
            assert_eq!(msg.address, "/synth/freq");
            if let OscType::Float(freq) = msg.args[0] {
                assert_eq!(freq, 440.0);
            } else {
                panic!("Expected float argument");
            }
        } else {
            panic!("Expected message in nested bundle");
        }

        if let OscPacket::Message(ref msg) = bundle.packets[1] {
            assert_eq!(msg.address, "/synth/amp");
            if let OscType::Float(amp) = msg.args[0] {
                assert_eq!(amp, 0.5);
            } else {
                panic!("Expected float argument");
            }
        } else {
            panic!("Expected message in nested bundle");
        }
    } else {
        panic!("Expected bundle at index 1");
    }
}

#[test]
fn roundtrip_deeply_nested_bundle() {
    // Create deeply nested bundles
    let msg = Message::new("/deep/message", vec![OscType::String("nested")]);

    let level3_bundle = Bundle::with_messages(300, vec![msg]);
    let mut level2_bundle = Bundle::empty(200);
    level2_bundle.add_bundle(level3_bundle);

    let mut level1_bundle = Bundle::empty(100);
    level1_bundle.add_bundle(level2_bundle);

    // Encode and decode
    let bytes = encode_bundle(&level1_bundle);
    let (decoded_bundle, used) = decode_bundle(&bytes).unwrap();

    assert_eq!(used, bytes.len());
    assert_eq!(decoded_bundle.timetag, 100);
    assert_eq!(decoded_bundle.packets.len(), 1);

    // Navigate through the nested structure
    if let OscPacket::Bundle(ref level2) = decoded_bundle.packets[0] {
        assert_eq!(level2.timetag, 200);
        assert_eq!(level2.packets.len(), 1);

        if let OscPacket::Bundle(ref level3) = level2.packets[0] {
            assert_eq!(level3.timetag, 300);
            assert_eq!(level3.packets.len(), 1);

            if let OscPacket::Message(ref msg) = level3.packets[0] {
                assert_eq!(msg.address, "/deep/message");
                assert_eq!(msg.args.len(), 1);
                if let OscType::String(s) = msg.args[0] {
                    assert_eq!(s, "nested");
                } else {
                    panic!("Expected string argument");
                }
            } else {
                panic!("Expected message in level 3 bundle");
            }
        } else {
            panic!("Expected bundle in level 2 bundle");
        }
    } else {
        panic!("Expected bundle in level 1 bundle");
    }
}

#[test]
fn roundtrip_mixed_bundle_content() {
    // Create a bundle with alternating messages and bundles
    let msg1 = Message::new("/msg1", vec![OscType::Int(1)]);
    let msg2 = Message::new("/msg2", vec![OscType::Int(2)]);
    let msg3 = Message::new("/msg3", vec![OscType::Int(3)]);
    let msg4 = Message::new("/msg4", vec![OscType::Int(4)]);

    let inner_bundle1 = Bundle::with_messages(150, vec![msg2]);
    let inner_bundle2 = Bundle::with_messages(250, vec![msg4]);

    let mut main_bundle = Bundle::empty(50);
    main_bundle.add_message(msg1);
    main_bundle.add_bundle(inner_bundle1);
    main_bundle.add_message(msg3);
    main_bundle.add_bundle(inner_bundle2);

    // Encode and decode
    let bytes = encode_bundle(&main_bundle);
    let (decoded_bundle, used) = decode_bundle(&bytes).unwrap();

    assert_eq!(used, bytes.len());
    assert_eq!(decoded_bundle.timetag, 50);
    assert_eq!(decoded_bundle.packets.len(), 4);

    // Verify the structure: Message, Bundle, Message, Bundle
    let expected_pattern = [
        ("Message", "/msg1", 1),
        ("Bundle", "", 150),
        ("Message", "/msg3", 3),
        ("Bundle", "", 250),
    ];

    for (i, (expected_type, expected_addr, expected_value)) in expected_pattern.iter().enumerate() {
        match &decoded_bundle.packets[i] {
            OscPacket::Message(msg) if *expected_type == "Message" => {
                assert_eq!(msg.address, *expected_addr);
                if let OscType::Int(val) = msg.args[0] {
                    assert_eq!(val, *expected_value);
                }
            }
            OscPacket::Bundle(bundle) if *expected_type == "Bundle" => {
                assert_eq!(bundle.timetag, *expected_value as u64);
                assert_eq!(bundle.packets.len(), 1);
            }
            _ => panic!("Unexpected packet type at index {}", i),
        }
    }
}
