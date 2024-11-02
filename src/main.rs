fn main() {

    
    let mut input=String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.pop();
    let hex_string="0x".to_owned()+&string_to_hex(&input);
    println!("{}", hex_string);
}

fn string_to_hex(s: &str) -> String {
    s.as_bytes().iter().map(|b| format!("{:02x}", b)).collect()
}