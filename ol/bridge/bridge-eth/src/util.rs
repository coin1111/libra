use ethers::abi::Token::Address;
use ethers::abi::Token::FixedBytes;
use ethers::abi::Token::Uint;
use ethers::types::H160;
use ethers_core::abi::Token::Bool;
use std::convert::TryInto;
use uuid::Uuid;

/// Transfer id to track bridge transactions
#[derive(Debug, Clone)]
pub struct TransferId {
    pub id: String,
    pub bytes: [u8; 16],
}
impl TransferId {
    pub fn new() -> Result<TransferId, String> {
        let uuid = *Uuid::new_v4().as_bytes();
        let tid =
            TransferId{
                id : hex::encode(uuid),
                bytes: uuid.clone(),
            };

        Ok(tid)
    }
    /// Initialize using string literal
    pub fn from(id: &str) -> Result<TransferId, String> {
        let str = String::from(id);
        let bytes: [u8; 16] = hex_to_bytes(&str)
            .unwrap()
            .try_into()
            .map_err(|e| format!("cannot convert to hex: {:?}", e))?;
        Ok(TransferId {
            id: str,
            bytes: bytes,
        })
    }
}

/// Converts string to hex bytes.
/// Must not have 0x prefix.
/// Must have event number of characters.
pub fn hex_to_bytes(s: &String) -> Option<Vec<u8>> {
    if s.len() % 2 == 0 {
        (0..s.len())
            .step_by(2)
            .map(|i| {
                s.get(i..i + 2)
                    .and_then(|sub| u8::from_str_radix(sub, 16).ok())
            })
            .collect()
    } else {
        None
    }
}

#[derive(Debug)]
pub struct AccountInfo {
    pub sender_this: H160,
    pub sender_other: [u8; 16],
    pub receiver_this: H160,
    pub receiver_other: [u8; 16],
    pub balance: u64,
    pub transfer_id: [u8; 16],
    pub idx: ethers::prelude::U256,
    pub is_closed: bool,
}
pub fn vec_to_array<T, const N: usize>(v: Vec<T>) -> Result<[T; N], String> {
    v.try_into()
        .map_err(|v: Vec<T>| format!("Expected a Vec of length {} but it was {}", N, v.len()))
}

impl AccountInfo {
    pub fn from(tuple: ethers::abi::Token) -> Result<AccountInfo, String> {
        match tuple.clone() {
            ethers::abi::Token::Tuple(a) => match &a[..] {
                [Address(sender_this), FixedBytes(sender_other), Address(receiver_this), FixedBytes(receiver_other), Uint(balance), FixedBytes(transfer_id), Uint(idx), Bool(is_closed)] =>
                {
                    let sender_other = vec_to_array(sender_other.to_vec().clone())?;
                    let receiver_other = vec_to_array(receiver_other.to_vec().clone())?;
                    let transfer_id = vec_to_array(transfer_id.to_vec().clone())?;
                    Ok(AccountInfo {
                        sender_this: *sender_this,
                        sender_other: sender_other,
                        receiver_this: *receiver_this,
                        receiver_other: receiver_other,
                        balance: balance.as_u64(),
                        transfer_id: transfer_id,
                        idx: *idx,
                        is_closed: *is_closed,
                    })
                }
                _ => Err(format!("Cannot match array {:?}", a)),
            },
            _ => Err(format!("Cannot match tuple {:?}", tuple)),
        }
    }
}
