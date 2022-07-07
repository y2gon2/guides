use std::io::stdin;


pub fn inputName() -> String {
    println!("Please input your name.");
    let mut nameBuffer = String::new();
    let stdinName = stdin().read_line(&mut nameBuffer).expect("input error");
    nameBuffer[..nameBuffer.len() - 2].to_string()
}

pub fn inputAddress() -> String {
    println!("Please input an address.");
    let mut addrBuffer = String::new();
    let stdinAddr = stdin().read_line(&mut addrBuffer).expect("input error");
    addrBuffer[..addrBuffer.len() - 2].to_string()
}