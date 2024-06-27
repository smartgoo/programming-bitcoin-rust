use crate::error::Error;
use crate::hex::HexStream;

pub struct TxIn {
    previous_transaction_id: String,
    previous_index: u32,
}

impl TxIn {
    pub fn new(s: &mut HexStream) -> Result<TxIn, Error> {
        let tx_id_bytes_le = s.read(32).unwrap();
        let tx_id_bytes_be = tx_id_bytes_le.into_iter().rev().collect::<Vec<u8>>();
        let tx_id = HexStream::encode(tx_id_bytes_be.as_slice());

        let index_bytes_le = s.read(4).unwrap();
        let index = u32::from_le_bytes(index_bytes_le.as_slice().try_into()?);

        Ok(TxIn {
            previous_transaction_id: tx_id,
            previous_index: index
        })
    }
}