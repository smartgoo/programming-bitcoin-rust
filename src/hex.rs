use std::io::{self, Cursor, Read};
use std::num::ParseIntError;

pub struct HexCursor {
    cursor: Cursor<Vec<u8>>,
}

impl HexCursor {    
    pub fn new(hex_string: String) -> Result<HexCursor, ParseIntError> {
        let bytes: Vec<u8> = (0..hex_string.len())
            .step_by(2)
            .map(|i| {
                u8::from_str_radix(&hex_string[i..i + 2], 16).unwrap()
            })
            .collect();
        Ok(HexCursor { cursor: Cursor::new(bytes) })
    }

    pub fn read(&mut self, num_bytes: usize) -> io::Result<Vec<u8>> {
        let mut buffer = vec![0u8; num_bytes];
        let num_read = self.cursor.read(&mut buffer)?;
        buffer.truncate(num_read);
        Ok(buffer)
    }
    
    pub fn get_position(&self) -> u64 {
        self.cursor.position()
    }

    pub fn set_position(&mut self, position: u64) {
        self.cursor.set_position(position);
    }

    pub fn length(&self) -> usize {
        self.cursor.get_ref().len()
    }
}

impl HexCursor {
    // fn decode() {}

    pub fn encode(bytea: &[u8]) -> String {
        let mut hex_string = String::with_capacity(bytea.len() * 2);
        for &byte in bytea {
            hex_string.push_str(&format!("{:02x}", byte));
        }
        hex_string
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hex_string() {
        let tx_hex = "010000000269adb42422fb021f38da0ebe12a8d2a14c0fe484bcb0b7cb365841871f2d5e24000000006a4730440220199a6aa56306cebcdacd1eba26b55eaf6f92eb46eb90d1b7e7724bacbe1d19140220101c0d46e033361c60536b6989efdd6fa692265fcda164676e2f49885871038a0121039ac8bac8f6d916b8a85b458e087e0cd07e6a76a6bfdde9bb766b17086d9a5c8affffffff69adb42422fb021f38da0ebe12a8d2a14c0fe484bcb0b7cb365841871f2d5e24010000006b48304502210084ec4323ed07da4af6462091b4676250c377527330191a3ff3f559a88beae2e2022077251392ec2f52327cb7296be89cc001516e4039badd2ad7bbc950c4c1b6d7cc012103b9b554e25022c2ae549b0c30c18df0a8e0495223f627ae38df0992efb4779475ffffffff0118730100000000001976a9140ce17649c1306c291ca9e587f8793b5b06563cea88ac00000000".to_string();
        HexCursor::new(tx_hex);
    }
}