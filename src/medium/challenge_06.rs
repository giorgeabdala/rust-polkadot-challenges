trait Encode {
    fn encode(&self) -> Vec<u8>;
}

trait Decode: Sized {
    fn decode(input: &mut &[u8]) -> Result<Self, CodecError>;
}

#[derive(Debug, PartialEq)]
enum CodecError {
    NotEnoughData,
    InvalidData(String),
}

// SCALE codec: Substrate's binary encoding format for efficient on-chain storage
#[derive(Debug, PartialEq)]
struct Account {
    id: u32,
    balance: u64,
    is_active: bool,
}

#[derive(Debug, PartialEq)]
enum TransactionType {
    Transfer { to: u32, amount: u64 },
    Stake { amount: u64 },
    Vote { proposal_id: u32 },
}

// Manual SCALE implementation - normally use derive macros in production
impl Encode for u32 {
    fn encode(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Decode for u32 {
    fn decode(input: &mut &[u8]) -> Result<Self, CodecError> {
        if input.len() < 4 {
            return Err(CodecError::NotEnoughData);
        }
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(&input[..4]);
        *input = &input[4..];
        Ok(u32::from_le_bytes(bytes))
    }
}

impl Encode for u64 {
    fn encode(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl Decode for u64 {
    fn decode(input: &mut &[u8]) -> Result<Self, CodecError> {
        if input.len() < 8 {
            return Err(CodecError::NotEnoughData);
        }
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&input[..8]);
        *input = &input[8..];
        Ok(u64::from_le_bytes(bytes))
    }
}

impl Encode for bool {
    fn encode(&self) -> Vec<u8> {
        vec![if *self { 1 } else { 0 }]
    }
}

impl Decode for bool {
    fn decode(input: &mut &[u8]) -> Result<Self, CodecError> {
        if input.is_empty() {
            return Err(CodecError::NotEnoughData);
        }
        let value = input[0];
        *input = &input[1..];
        match value {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(CodecError::InvalidData("Invalid bool value".to_string())),
        }
    }
}

impl Encode for Account {
    fn encode(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend(self.id.encode());
        result.extend(self.balance.encode());
        result.extend(self.is_active.encode());
        result
    }
}

impl Decode for Account {
    fn decode(input: &mut &[u8]) -> Result<Self, CodecError> {
        let id = u32::decode(input)?;
        let balance = u64::decode(input)?;
        let is_active = bool::decode(input)?;
        Ok(Account { id, balance, is_active })
    }
}

impl Encode for TransactionType {
    fn encode(&self) -> Vec<u8> {
        let mut result = Vec::new();
        match self {
            TransactionType::Transfer { to, amount } => {
                result.push(0);
                result.extend(to.encode());
                result.extend(amount.encode());
            }
            TransactionType::Stake { amount } => {
                result.push(1);
                result.extend(amount.encode());
            }
            TransactionType::Vote { proposal_id } => {
                result.push(2);
                result.extend(proposal_id.encode());
            }
        }
        result
    }
}

impl Decode for TransactionType {
    fn decode(input: &mut &[u8]) -> Result<Self, CodecError> {
        if input.is_empty() {
            return Err(CodecError::NotEnoughData);
        }
        let variant_index = input[0];
        *input = &input[1..];

        match variant_index {
            0 => {
                let to = u32::decode(input)?;
                let amount = u64::decode(input)?;
                Ok(TransactionType::Transfer { to, amount })
            }
            1 => {
                let amount = u64::decode(input)?;
                Ok(TransactionType::Stake { amount })
            }
            2 => {
                let proposal_id = u32::decode(input)?;
                Ok(TransactionType::Vote { proposal_id })
            }
            _ => Err(CodecError::InvalidData("Invalid transaction type".to_string())),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_encode_decode() {
        let account = Account { id: 42, balance: 1000, is_active: true };
        let encoded = account.encode();
        assert_eq!(encoded.len(), 4 + 8 + 1);

        let decoded = Account::decode(&mut encoded.as_slice()).unwrap();
        assert_eq!(decoded, account);
    }

    #[test]
    fn test_transaction_transfer_encode_decode() {
        let transaction = TransactionType::Transfer { to: 1, amount: 100 };
        let encoded = transaction.encode();
        // 1 byte for variant, 4 for `to`, 8 for `amount`
        assert_eq!(encoded.len(), 1 + 4 + 8);

        let decoded = TransactionType::decode(&mut encoded.as_slice()).unwrap();
        assert_eq!(decoded, transaction);
    }

    #[test]
    fn test_transaction_stake_encode_decode() {
        let transaction = TransactionType::Stake { amount: 500 };
        let encoded = transaction.encode();
        // 1 byte for variant, 8 for `amount`
        assert_eq!(encoded.len(), 1 + 8);

        let decoded = TransactionType::decode(&mut encoded.as_slice()).unwrap();
        assert_eq!(decoded, transaction);
    }

    #[test]
    fn test_transaction_vote_encode_decode() {
        let transaction = TransactionType::Vote { proposal_id: 7 };
        let encoded = transaction.encode();
        // 1 byte for variant, 4 for `proposal_id`
        assert_eq!(encoded.len(), 1 + 4);

        let decoded = TransactionType::decode(&mut encoded.as_slice()).unwrap();
        assert_eq!(decoded, transaction);
    }

    #[test]
    fn test_decoding_error_not_enough_data() {
        let encoded = vec![1, 2, 3];
        let result = Account::decode(&mut encoded.as_slice());
        assert_eq!(result, Err(CodecError::NotEnoughData));
    }

    #[test]
    fn test_decoding_error_invalid_transaction_type() {
        let encoded = vec![99, 0, 0, 0, 0]; // Invalid variant 99
        let result = TransactionType::decode(&mut encoded.as_slice());
        assert_eq!(result, Err(CodecError::InvalidData("Invalid transaction type".to_string())));
    }

    #[test]
    fn test_decode_consumes_input() {
        let account1 = Account { id: 1, balance: 100, is_active: true };
        let account2 = Account { id: 2, balance: 200, is_active: false };

        let mut encoded = account1.encode();
        encoded.extend(account2.encode());

        let mut slice = encoded.as_slice();

        let decoded1 = Account::decode(&mut slice).unwrap();
        assert_eq!(decoded1, account1);

        // The slice should now point to the start of the second account's data
        assert_eq!(slice.len(), 13);

        let decoded2 = Account::decode(&mut slice).unwrap();
        assert_eq!(decoded2, account2);

        // The slice should be empty now
        assert!(slice.is_empty());
    }
}