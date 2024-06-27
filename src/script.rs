use crate::hex::HexStream;
use crate::varint::VarInt;

pub struct Script;

impl Script {
    pub fn parse(s: &mut HexStream) -> () {
        // get length of entire field
        let length = VarInt::from_stream(s);

        let mut cmds = Vec::new();

        let mut count = 0;

        while count < length {
            let current = s.read(1).unwrap();
            count += 1;
            let current_byte = current[0];

            if current_byte >= 1 && current_byte <= 75 {
                // we have an cmd set n to be the current byte
                let n = current_byte;

                // add the next n bytes as an cmd
                cmds.push(s.read(n as usize).unwrap());

                count += n as u64;
            } else if current_byte == 76 {
                // op_pushdata1
                let length = u8::from_le_bytes(s.read(1).unwrap().try_into().unwrap());
                cmds.push(s.read(length as usize).unwrap());
                count += length as u64 + 1;
            } else if current_byte == 77 {
                let length = u8::from_le_bytes(s.read(2).unwrap().try_into().unwrap());
                cmds.push(s.read(length as usize).unwrap());
                count + length as u64 + 2;
            } else {
                // opcode
                let op_code = current_byte;
                cmds.push(vec![op_code]);
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn parse() {
        let mut s = HexStream::new("6a47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937".to_string()).unwrap();
        Script::parse(&mut s)
    }
}