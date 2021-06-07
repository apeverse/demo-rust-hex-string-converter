
// #[macro_use]
// extern crate hex_literal;
use hex_literal::hex;
use hex_fmt::HexFmt;
use hex::encode;
use primitive_types::{H160, H256, U256};

// !!! Necessary for H160::from_str(address).expect("...");
use std::str::FromStr;
use ethereum_types::BigEndianHash;


pub fn hash_meets_difficulty(hash: &H256, difficulty: U256) -> bool {
    let num_hash = U256::from(&hash[..]);
    let (_, overflowed) = num_hash.overflowing_mul(difficulty);

    !overflowed
}


#[test]
fn hash_hex_str() {
    // use primitive_types::{H160, H256};
    let foo = H160::from(hex!("ef2d6d194084c2de36e0dabfce45d046b37d1106"));
    println!("H160 => {:?}", foo);

    let foo = H256::from(hex!("bc36789e7a1e281436464229828f817d6612f7b477d66591ff96a9e064bcc98a"));
    println!("H256 => {:x?}", foo);

    let bar = U256::from(&foo[..]);
    println!("U256 => {:?}", bar);

    let foo = H256::from_uint(&bar);
    println!("H256 from U256 => {:?}", foo);

    let bar = foo.as_bytes();
    println!("bytes (256bit) => {:x?}", bar);

    let dog : &[u8] = foo.as_bytes();
    println!("&[u8] => {:x?}", dog);

    let zee : Vec<u8> = bar.into();
    println!("Vec<u8> => {:x?}", zee);

    let cat = zee.as_slice();
    println!("&[u8] => {:x?}", cat);

    let mut bee = [0u8; 32];
    bee.copy_from_slice(cat);
    println!("[u8; 32] array => {:x?}", bee);

    let foo = H256::from(bee);
    println!("H256 => {:?}", foo);

    // !!! Must have ethereum-types = "x.y.z" in Cargo.toml
    // !!! use std::str::FromStr;
    let foo = H160::from_str("ef2d6d194084c2de36e0dabfce45d046b37d1106")
        .expect("The argument must be a valid address string.");
    println!("H160::from_str => {:?}", foo);
    println!("H160::from_str short => {}", foo);
}

#[test]
fn tx_rlp_decode() {
    // https://etherscan.io/getRawTx?tx=0x083cc4af906c0b8b67a630507f695aa0dab2bde84fada412fff608d0ee9ea1ae
    let data = hex!("
        f8ad830dd98a8502540be40083026739947c2af3a86b4bf47e6ee63ad9bde7b3
        b0ba7f95da80b844a9059cbb000000000000000000000000b34938746d316e99
        5aa81f9b3f94419a0a41e1430000000000000000000000000000000000000000
        0000026faff2dfe5c524000025a0167bf6ce1f7ecee1e5a414e3622baa14daf6
        caaf90f498b4fb94b1a91bc79491a0362191d3956065a0e14276dd4810b523e9
        3a786091d27388a2b00b6955f93161").to_vec();
    let rlp_obj = rlp::Rlp::new(&data);
    println!("data: {}", rlp_obj);
    //info!("data: {}", rlp_obj);
}

#[test]
fn hex_macro_test() {
    // use hex::encode;
    let packet = hex!("
		c679fc8fe0b8b12f06577f2e802d34f6fa257e6137a995f6f4cbfc9ee50ed3710faf6e66f932c4c8
		d81d64343f429651328758b47d3dbc02c4042f0fff6946a50f4a49037a72bb550f3a7872363a83e1
		b9ee6469856c24eb4ef80b7535bcf99c0004f9015bf90150f84d846321163782115c82115db84031
		55e1427f85f10a5c9a7755877748041af1bcd8d474ec065eb33df57a97babf54bfd2103575fa8291
		15d224c523596b401065a97f74010610fce76382c0bf32f84984010203040101b840312c55512422
		cf9b8a4097e9a6ad79402e87a15ae909a4bfefa22398f03d20951933beea1e4dfa6f968212385e82
		9f04c2d314fc2d4e255e0d3bc08792b069dbf8599020010db83c4d001500000000abcdef12820d05
		820d05b84038643200b172dcfef857492156971f0e6aa2c538d8b74010f8e140811d53b98c765dd2
		d96126051913f44582e8c199ad7c6d6819e9a56483f637feaac9448aacf8599020010db885a308d3
		13198a2e037073488203e78203e8b8408dcab8618c3253b558d459da53bd8fa68935a719aff8b811
		197101a4b2b47dd2d47295286fc00cc081bb542d760717d1bdd6bec2c37cd72eca367d6dd3b9df73
		8443b9a355010203b525a138aa34383fec3d2719a0").to_vec();
    println!("{:?}", packet);
    println!("{}", encode(&packet));
    let zee = format!("0x{}", encode(&packet));
    println!("{}", zee);
    println!("0x{}", HexFmt(&packet));
}

fn main() {
    const DATA: [u8; 4] = hex!("01020304");
    assert_eq!(DATA, [1, 2, 3, 4]);
    assert_eq!(hex!("a1 b2 c3 d4"), [0xA1, 0xB2, 0xC3, 0xD4]);
    assert_eq!(hex!("E5 E6 90 92"), [0xE5, 0xE6, 0x90, 0x92]);
    assert_eq!(hex!("0a0B 0C0d"), [10, 11, 12, 13]);
    let bytes = hex!("
        00010203 04050607
        08090a0b 0c0d0e0f
    ");
    assert_eq!(bytes, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);

}


