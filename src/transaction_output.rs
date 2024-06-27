use crate::{
    error::Error, 
    hex::HexCursor, 
    script::Script,
};

pub struct TxOut {
    amount: u64,
    script_pubkey: Script,
}

impl TxOut {
    pub fn parse(s: &mut HexCursor) -> Result<TxOut, Error> {
        let amount_bytes = s.read(8).unwrap();
        let amount = u64::from_le_bytes(amount_bytes.try_into().unwrap());

        let script_pubkey = Script::parse(s);

        Ok(TxOut { amount, script_pubkey })
    }
}