use crate::{
    error::Error,
    hex::HexCursor,
    script::Script,
};

pub struct TxIn {
    previous_transaction_id: String,
    previous_index: u32,
    script_sig: Script,
    sequence: u32,
}

impl TxIn {
    pub fn parse(s: &mut HexCursor) -> Result<TxIn, Error> {
        let tx_id_bytes_le = s.read(32).unwrap();
        let tx_id_bytes_be = tx_id_bytes_le.into_iter().rev().collect::<Vec<u8>>();
        let tx_id = HexCursor::encode(tx_id_bytes_be.as_slice());

        let index_bytes_le = s.read(4).unwrap();
        let index = u32::from_le_bytes(index_bytes_le.as_slice().try_into()?);

        let script_sig = Script::parse(s);

        let sequence_bytes = s.read(4).unwrap();
        let sequence = u32::from_le_bytes(sequence_bytes.try_into().unwrap());

        Ok(TxIn {
            previous_transaction_id: tx_id,
            previous_index: index,
            script_sig: script_sig,
            sequence: sequence,
        })
    }
}