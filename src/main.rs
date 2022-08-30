use lrwn::{Payload, PhyPayload, MType};

fn main() {
    println!("Hello, world!");
    let trace =
    [
        "AFbpuZpW+YFgIgBIADMla0fZRDW+IoI=",
        "AFbpuZpW+YFgIgBIADMla0fZRDW+IoI=",
        "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"
    ];
    for s in trace {
        println!(r#"{:?}"#, s);
        let bytes = base64::decode(s).unwrap();
        println!("{:?}", bytes);
        let p = PhyPayload::from_slice(&bytes);
        println!("{:?}", p);
    }
    test_proprietary();
}

// fn pv_a(p_strings: &[str]) {
//     for s in p_strings {
//         println!(r#"{:?}"#, s);
//         let bytes = base64::decode(s).unwrap();
//         println!("{:?}", bytes);
//         let p = PhyPayload::from_slice(&bytes);
//         println!("{:?}", p); 
//     }
// }

struct PayloadTest {
    pl: Payload,
    bytes: Vec<u8>,
}

fn test_proprietary() {

    let tests = vec![PayloadTest {
        pl: Payload::Raw(vec![0x01, 0x02, 0x3]),
        bytes: vec![0x01, 0x02, 0x03],
    }];

    for tst in tests {
        assert_eq!(tst.bytes, tst.pl.to_vec().unwrap());
        assert_eq!(
            tst.pl,
            Payload::from_slice(MType::Proprietary, &tst.bytes).unwrap()
        );
    }
}
