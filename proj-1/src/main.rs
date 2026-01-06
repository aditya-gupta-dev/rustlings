use proj_1::{EncryptionAlgorithmTrait, XOREncryptionAlgorithm};
use std::{io::{self}, process, string::FromUtf8Error};

fn main() {

    println!("enter something to encrypt: ");

    let mut buffer: String = String::new();
    match io::stdin().read_line(&mut buffer) { 
        Ok(n) => println!("read {} bytes",n),
        Err(e) => { 
            println!("failed to get input: {}", e); 
            process::exit(1);
        }
    }

    let xor_hasher: XOREncryptionAlgorithm = XOREncryptionAlgorithm{ 
        debug: false 
    }; 

    let encrypted: Result<String, FromUtf8Error> = xor_hasher.encrypt(&buffer, 20); 
    let decrypted: Result<String, FromUtf8Error> = xor_hasher.decrypt(&buffer, 20); 

    println!("{}", encrypted.unwrap());
    println!("{}", decrypted.unwrap());
}

