use std::{string::FromUtf8Error};

pub trait EncryptionAlgorithmTrait {
    fn encrypt(&self, input: &String, key: u8) -> Result<String, FromUtf8Error>;
    fn decrypt(&self, input: &String, key: u8) -> Result<String, FromUtf8Error>;
}

pub struct XOREncryptionAlgorithm {
    pub debug: bool 
}

impl EncryptionAlgorithmTrait for XOREncryptionAlgorithm {
    fn encrypt(&self, input: &String, key: u8) -> Result<String, FromUtf8Error> {
        return String::from_utf8(input.bytes().map(|b| b ^ key).collect());
    }

    fn decrypt(&self, input: &String, key: u8) -> Result<String, FromUtf8Error> {
        match self.encrypt(input, key) { 
            Ok(res) => self.encrypt(&res, key), 
            Err(e) => Err(e)
        }
    }
}
