
#[macro_use]
extern crate hex_literal;
use hex_fmt::HexFmt;
use hex::encode;

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
		8443b9a355010203b525a138aa34383fec3d2719a0
	").to_vec();
    println!("{:?}", packet);
    println!("{}", encode(&packet));
    let zee = format!("0x{}", encode(&packet));
    println!("{}", zee);
    println!("0x{}", HexFmt(&packet));

    //https://etherscan.io/getRawTx?tx=0x083cc4af906c0b8b67a630507f695aa0dab2bde84fada412fff608d0ee9ea1ae
    let data = hex!("f8ad830dd98a8502540be40083026739947c2af3a86b4bf47e6ee63ad9bde7b3b0ba7f95da80b844a9059cbb000000000000000000000000b34938746d316e995aa81f9b3f94419a0a41e14300000000000000000000000000000000000000000000026faff2dfe5c524000025a0167bf6ce1f7ecee1e5a414e3622baa14daf6caaf90f498b4fb94b1a91bc79491a0362191d3956065a0e14276dd4810b523e93a786091d27388a2b00b6955f93161").to_vec();
    let rlp_obj = rlp::Rlp::new(&data);
    println!("data: {}", rlp_obj);
    //info!("data: {}", rlp_obj);
}


