use rsycodec::hex;
fn main() {
    let hex_string = hex::encode("Hello world!");
    println!("{}", hex_string); // Prints "48656c6c6f20776f726c6421"
    println!(
        "decoded: {:?}",
        String::from_utf8(hex::decode("48656c6c6f20776f726c6421").unwrap()).unwrap()
    );
}
