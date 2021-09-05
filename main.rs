use rand::Rng;
use std::io;

mod input {
    use std::io;
    pub fn string(msg: &str) -> String {
        println!("{}", msg);
        let mut result =  String::new();
        io::stdin()
            .read_line(&mut result)
            .expect("Failed to read line.");
        result
    }

    pub fn number(msg: &str) -> u8 {
        loop {
            println!("{}", msg);
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");
            match input.trim().parse::<u8>() {
                Ok(num) => return num,
                Err(_) => {
                    println!("\nFailed to parse number\n");
                    continue;
                }
            };
        }
    }
}

fn xor_cypher(msg: &str, key: &str) -> String {
    let mut output = String::new();
    for (m, k) in msg.chars().zip(key.chars()) {
        output.push(
            ((m as u8) ^ (k as u8)) as char
        );
    }
    output
}

fn key_from(msg: &str) -> String {
    let mut rng = rand::thread_rng();
    let mut key = String::new();
    
    for _i in 0..msg.len() {
        let rand_byte: u8 = rng.gen();
        key.push(rand_byte as char)
    }
    key
}


fn main() {
    loop {
        match input::number("Press '1' to encode a message,\nand '2' to decode a message") {
            1 => {
                let msg = input::string("Enter a message to be encoded: ");
                let key = key_from(&msg);
                
                println!("(debug) msg: [{}]", msg);

                
                println!("Your encoded message: [{}]", xor_cypher(&msg, &key));
                println!("\n..and your key: [{}]", key);
            } 2 => {
                let msg = input::string("Enter a message to be decoded: ");
                let key = input::string("Enter your key: ");

                println!("(debug) msg: [{}]", msg);
                
                println!("Your decoded message: [{}]", xor_cypher(&msg, &key));
            } _ => {
                println!("\nInvalid choice.\n");
                continue;
            }
        }
        println!("Paused...");
        io::stdin().read_line(&mut String::new()).unwrap();
    }
}
