use crate::error::Error;
use crate::hex::HexStream;
use crate::varint::VarInt;
use crate::transaction_input::TxIn;

pub struct Tx {
    version: u32,
    num_inputs: u64,
    tx_inputs: Vec<TxIn>,
    // tx_outs
    // locktime
    // testnet
}

impl Tx {
    // fn id(&self) -> ? {}

    // fn hash(&self) -> ? {}

    pub fn parse(s: &str) -> Result<Tx, Error> {
        let mut s = HexStream::new(s.into())?;

        // Version (first 4 bytes, LE)
        let version_bytes: [u8; 4] = s.read(4).unwrap().as_slice().try_into()?;
        let version = u32::from_le_bytes(version_bytes);

        // Parse num inputs (varint, LE)
        let num_inputs = VarInt::from_stream(&mut s);

        // Parse transactions
        let tx_in = TxIn::new(&mut s)?;

        Ok(Tx { version, num_inputs, tx_inputs: vec![tx_in] })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_parse() {
        let tx_hex = "010000000269adb42422fb021f38da0ebe12a8d2a14c0fe484bcb0b7cb365841871f2d5e24000000006a4730440220199a6aa56306cebcdacd1eba26b55eaf6f92eb46eb90d1b7e7724bacbe1d19140220101c0d46e033361c60536b6989efdd6fa692265fcda164676e2f49885871038a0121039ac8bac8f6d916b8a85b458e087e0cd07e6a76a6bfdde9bb766b17086d9a5c8affffffff69adb42422fb021f38da0ebe12a8d2a14c0fe484bcb0b7cb365841871f2d5e24010000006b48304502210084ec4323ed07da4af6462091b4676250c377527330191a3ff3f559a88beae2e2022077251392ec2f52327cb7296be89cc001516e4039badd2ad7bbc950c4c1b6d7cc012103b9b554e25022c2ae549b0c30c18df0a8e0495223f627ae38df0992efb4779475ffffffff0118730100000000001976a9140ce17649c1306c291ca9e587f8793b5b06563cea88ac00000000";

        let tx = Tx::parse(tx_hex).unwrap();
        assert_eq!(tx.version, 1);
        assert_eq!(tx.num_inputs, 38);
    }
}