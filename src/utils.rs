pub fn hex_dump(data: &[u8]) {
    for (i, byte) in data.iter().enumerate() {
        if i % 16 == 0 { print!("\n{:04x}: ", i); }
        print!("{:02x} ", byte);
    }
    println!();
}