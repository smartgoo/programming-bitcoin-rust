use crate::{
    error::Error,
    hex::HexCursor,
    varint::VarInt,
    transaction_input::TxIn,
    transaction_output::TxOut,
};

pub struct Tx {
    version: u32,
    num_inputs: u64,
    tx_inputs: Vec<TxIn>,
    num_outputs: u64,
    tx_outputs: Vec<TxOut>,
    locktime: u32,
    // testnet
}

impl Tx {
    // fn id(&self) -> ? {}

    // fn hash(&self) -> ? {}

    pub fn parse(s: &str) -> Result<Tx, Error> {
        let mut s = HexCursor::new(s.into())?;

        // Version (first 4 bytes, LE)
        let version_bytes: [u8; 4] = s.read(4).unwrap().as_slice().try_into()?;
        let version = u32::from_le_bytes(version_bytes);

        // Parse num inputs (varint, LE)
        let num_inputs = VarInt::from_stream(&mut s).unwrap();

        // Parse tx inputs
        let mut tx_inputs = Vec::<TxIn>::new();
        for _ in 0..num_inputs {
            tx_inputs.push(TxIn::parse(&mut s)?);
        } 

        // Parse num outputs (varint, LE)
        let num_outputs = VarInt::from_stream(&mut s).unwrap();

        // Parse tx outputs
        let mut tx_outputs = Vec::<TxOut>::new();
        for _ in 0..num_outputs {
            tx_outputs.push(TxOut::parse(&mut s)?);
        } 

        // Locktime
        let locktime_bytes = s.read(4).unwrap();
        let locktime = u32::from_le_bytes(locktime_bytes.try_into().unwrap());

        Ok(Tx { version, num_inputs, tx_inputs, num_outputs, tx_outputs, locktime })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let tx_hex = "010000000456919960ac691763688d3d3bcea9ad6ecaf875df5339e148a1fc61c6ed7a069e010000006a47304402204585bcdef85e6b1c6af5c2669d4830ff86e42dd205c0e089bc2a821657e951c002201024a10366077f87d6bce1f7100ad8cfa8a064b39d4e8fe4ea13a7b71aa8180f012102f0da57e85eec2934a82a585ea337ce2f4998b50ae699dd79f5880e253dafafb7feffffffeb8f51f4038dc17e6313cf831d4f02281c2a468bde0fafd37f1bf882729e7fd3000000006a47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937feffffff567bf40595119d1bb8a3037c356efd56170b64cbcc160fb028fa10704b45d775000000006a47304402204c7c7818424c7f7911da6cddc59655a70af1cb5eaf17c69dadbfc74ffa0b662f02207599e08bc8023693ad4e9527dc42c34210f7a7d1d1ddfc8492b654a11e7620a0012102158b46fbdff65d0172b7989aec8850aa0dae49abfb84c81ae6e5b251a58ace5cfeffffffd63a5e6c16e620f86f375925b21cabaf736c779f88fd04dcad51d26690f7f345010000006a47304402200633ea0d3314bea0d95b3cd8dadb2ef79ea8331ffe1e61f762c0f6daea0fabde022029f23b3e9c30f080446150b23852028751635dcee2be669c2a1686a4b5edf304012103ffd6f4a67e94aba353a00882e563ff2722eb4cff0ad6006e86ee20dfe7520d55feffffff0251430f00000000001976a914ab0c0b2e98b1ab6dbf67d4750b0a56244948a87988ac005a6202000000001976a9143c82d7df364eb6c75be8c80df2b3eda8db57397088ac46430600";

        let tx = Tx::parse(tx_hex).unwrap();
        assert_eq!(tx.version, 1);
        assert_eq!(tx.num_inputs, 4);
        assert_eq!(tx.tx_inputs.len(), 4);
        assert_eq!(tx.num_outputs, 2);
        assert_eq!(tx.tx_outputs.len(), 2);
    }
}