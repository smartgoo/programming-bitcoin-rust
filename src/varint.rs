use crate::hex::HexStream;

pub struct VarInt;

impl VarInt {
    pub fn decode(bytea: [u8; 9]) -> u64 {
        let num_inputs = match bytea[0] {
            0..=252 => bytea[0] as u64,
            253 => u64::from_le_bytes([bytea[1], bytea[2], 0, 0, 0, 0, 0, 0]),
            254 => u64::from_le_bytes([bytea[1], bytea[2], bytea[3], bytea[4], 0, 0, 0, 0]),
            255 => u64::from_le_bytes(
                [bytea[1], bytea[2], bytea[3], bytea[4], bytea[5], bytea[6], bytea[7], bytea[8]]
            )
        };
        num_inputs
    }

    // pub fn encode(num_inputs: u64) -> Vec<u8> {}
}

impl VarInt {
    pub fn num_bytes(first_byte: u8) -> u8 {
        let num_bytes = match first_byte {
            0..=252 => 1,
            253 => 2,
            254 => 4,
            255 => 8,
        };

        num_bytes
    }

    pub fn num_inputs(bytea: &[u8]) -> u64 {
        let num_inputs = match bytea.len() {
            1 => bytea[0] as u64,
            2 => u64::from_le_bytes([bytea[1], bytea[2], 0, 0, 0, 0, 0, 0]),
            4 => u64::from_le_bytes([bytea[1], bytea[2], bytea[3], bytea[4], 0, 0, 0, 0]),
            8 => u64::from_le_bytes(
                [bytea[1], bytea[2], bytea[3], bytea[4], bytea[5], bytea[6], bytea[7], bytea[8]]
            ),
            _ => panic!()
        };
        num_inputs
    }

    pub fn from_stream(s: &mut HexStream) -> u64 {
        // TODO cleanup, should be able to do direclty in this function?
        let num_input_bytes = VarInt::num_bytes(s.read(1).unwrap()[0]) as usize;
        let num_inputs = VarInt::num_inputs(s.read(num_input_bytes).unwrap().as_slice());
        num_inputs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_byte_varint() {
        let bytea = vec![200, 0, 0, 0, 0, 0, 0, 0, 0];
        let num = VarInt::decode(bytea.try_into().unwrap());
        assert_eq!(num, 200);

        let bytea = vec![200, 150, 0, 90, 10, 15, 91, 29, 38];
        let num = VarInt::decode(bytea.try_into().unwrap());
        assert_eq!(num, 200);
    }

    #[test]
    fn two_byte_varint() {
        let bytea = vec![0xfd, 2, 99, 0, 0, 0, 0, 0, 0];
        let num = VarInt::decode(bytea.try_into().unwrap());
        assert_eq!(num, 25346);
    }

    #[test]
    fn four_byte_varint() {
        let bytea = vec![0xfe, 145, 1, 0, 55, 0, 0, 0, 0];
        let num = VarInt::decode(bytea.try_into().unwrap());
        assert_eq!(num, 922747281);
    }

    #[test]
    fn eight_byte_varint() {
        let bytea = vec![0xff, 82, 211, 17, 55, 159, 242, 187, 143];
        let num = VarInt::decode(bytea.try_into().unwrap());
        assert_eq!(num, 0x8FBBF29F3711D352);
    }
}