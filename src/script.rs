use crate::{hex::HexCursor, varint::VarInt};

pub struct Script {
    cmds: Vec<Vec<u8>>,
}

impl Script {
    pub fn parse(s: &mut HexCursor) -> Script {
        // get length of entire field
        let script_length = VarInt::from_stream(s).unwrap();
        let mut cmds = Vec::new();
        let mut count = 0;

        while count < script_length {
            let current = s.read(1).unwrap();
            count += 1;
            let current_byte = current[0];

            match current_byte {
                1..=75 => {
                    // cmd
                    let n = current_byte as usize;
                    cmds.push(s.read(n).unwrap());
                    count += n as u64;
                },
                76 => {
                    // op_pushdata1
                    let len = u8::from_le_bytes(s.read(1).unwrap().try_into().unwrap()) as u64;
                    cmds.push(s.read(len as usize).unwrap());
                    count += len + 1;
                },
                77 => {
                    // op_pushdata2
                    let len = u16::from_le_bytes(s.read(2).unwrap().try_into().unwrap()) as u64;
                    cmds.push(s.read(len as usize).unwrap());
                    count += len + 2;
                },
                _ => {
                    cmds.push(vec![current_byte])
                }
            }
        }
        
        if count != script_length {
            println!("Count {} | Length {}", count, script_length);
            panic!("Failed to parse script!");
        }

        Script { cmds }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn parse() {
        let mut s = HexCursor::new("6a47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937".to_string()).unwrap();
        Script::parse(&mut s);
    }
}