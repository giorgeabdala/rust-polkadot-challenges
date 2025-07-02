#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ChainId(pub u32);
pub type Balance = u128;
pub type AccountId = String;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AssetId {
    MainToken,
}

#[derive(Debug, PartialEq)]
pub struct TransferMessage {
    pub from_chain: ChainId,
    pub to_chain: ChainId,
    pub from_account: AccountId,
    pub to_account: AccountId,
    pub asset_id: AssetId,
    pub amount: Balance
}

impl TransferMessage {
    pub fn new(
        from_chain: ChainId,
        to_chain: ChainId,
        from_account: AccountId,
        to_account: AccountId,
        asset_id: AssetId,
        amount: Balance
    ) -> Self {
        Self {
            from_chain,
            to_chain,
            from_account,
            to_account,
            asset_id,
            amount
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    InsufficientBalance,
    InvalidDestinationChain,
    ZeroAmountTransfer,
}

use std::collections::HashMap;

pub struct AssetPallet {
    balances: HashMap<(AccountId, AssetId), Balance>,
    chain_id: ChainId,
}

impl AssetPallet {
    pub fn new(chain_id: ChainId) -> Self {
        Self {
            balances: HashMap::new(),
            chain_id
        }
    }

    pub fn get_chain_id(&self) -> ChainId {
        self.chain_id
    }

    pub fn balance_of(&self, account: &AccountId, asset_id: &AssetId) -> Balance {
        self.balances.get(&(account.clone(), *asset_id)).copied().unwrap_or(0)
    }

    pub fn set_balance(&mut self, account: &AccountId, asset_id: AssetId, amount: Balance) {
        if amount == 0 {
            self.balances.remove(&(account.clone(), asset_id));
        } else {
            self.balances.insert((account.clone(), asset_id), amount);
        }
    }

    fn increase_balance(&mut self, account: &AccountId, asset_id: AssetId, amount: Balance) {
        let current = self.balance_of(account, &asset_id);
        self.set_balance(account, asset_id, current + amount);
    }

    fn decrease_balance(&mut self, account: &AccountId, asset_id: AssetId, amount: Balance) -> Result<(), Error> {
        let current = self.balance_of(account, &asset_id);
        if current < amount {
            return Err(Error::InsufficientBalance);
        }
        self.set_balance(account, asset_id, current - amount);
        Ok(())
    }

    pub fn initiate_transfer(
        &mut self,
        sender: &AccountId,
        destination_chain: ChainId,
        beneficiary: &AccountId,
        asset_id: AssetId,
        amount: Balance,
    ) -> Result<TransferMessage, Error> {
        if destination_chain == self.chain_id {return Err(Error::InvalidDestinationChain)};
        if amount <= 0 {return Err(Error::ZeroAmountTransfer)};
        self.decrease_balance(sender, asset_id, amount)?;
        let transfer_msg =TransferMessage::new(
            self.chain_id, destination_chain, sender.clone(), beneficiary.clone(), asset_id, amount);
        Ok(transfer_msg)
    }

    pub fn process_incoming_transfer(
        &mut self,
        message: TransferMessage,
    ) -> Result<(), Error> {
        if message.to_chain != self.chain_id {return Err(Error::InvalidDestinationChain)}
        self.increase_balance(&message.to_account, message.asset_id, message.amount);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::advanced::challenge_11::{AccountId, AssetId, AssetPallet, ChainId, Error};

    #[test]
    pub fn initiate_transfer_test() {
        let sender = &"alice".to_string();
        let to_chain = ChainId(2);
        let to = &"bob".to_string();
       let mut pallet = AssetPallet::new(ChainId(1));
        pallet.set_balance(sender, AssetId::MainToken, 20);
        let result =
            pallet.initiate_transfer(sender, to_chain, to, AssetId::MainToken, 10);
        assert!(result.is_ok());
        let transfer_msg = result.unwrap();
        assert_eq!(transfer_msg.from_account, sender.clone());
        assert_eq!(transfer_msg.to_account, to.clone());
        assert_eq!(transfer_msg.asset_id, AssetId::MainToken);
        assert_eq!(transfer_msg.amount, 10);
    }

    #[test]
    pub fn initiate_transfer_sender_balance_insufficient_fail() {
        let sender = &"alice".to_string();
        let to_chain = ChainId(2);
        let to = &"bob".to_string();
        let mut pallet = AssetPallet::new(ChainId(1));
        let result =
            pallet.initiate_transfer(sender, to_chain, to, AssetId::MainToken, 10);
        assert!(result.is_err());
        assert_eq!(result, Err(Error::InsufficientBalance));
    }

    #[test]
    pub fn initiate_transfer_invalid_destinataion_fail() {
        let sender = &"alice".to_string();
        let to = &"bob".to_string();
        let mut pallet = AssetPallet::new(ChainId(1));
        let result =
            pallet.initiate_transfer(sender, ChainId(1), to, AssetId::MainToken, 10);
        assert!(result.is_err());
        assert_eq!(result, Err(Error::InvalidDestinationChain));
    }

    #[test]
    pub fn initiate_transfer_invalid_amount_fail() {
        let sender = &"alice".to_string();
        let to = &"bob".to_string();
        let mut pallet = AssetPallet::new(ChainId(1));
        pallet.set_balance(sender, AssetId::MainToken, 20);
        let result =
            pallet.initiate_transfer(sender, ChainId(2), to, AssetId::MainToken, 0);
        assert!(result.is_err());
        assert_eq!(result, Err(Error::ZeroAmountTransfer));
    }

    #[test]
    pub fn transfer_test() {
        let sender = &"alice".to_string();
        let from_chain = ChainId(1);
        let to_chain = ChainId(2);
        let to = &"bob".to_string();
        let mut chain_a = AssetPallet::new(from_chain);
        let mut chain_b = AssetPallet::new(to_chain);
        chain_a.set_balance(sender, AssetId::MainToken, 20);
        let result =
            chain_a.initiate_transfer(sender, to_chain, to, AssetId::MainToken, 10);
        let transfer_msg = result.unwrap();

        let transfer_result = chain_b.process_incoming_transfer(transfer_msg);
        assert!(transfer_result.is_ok());

        assert_eq!(chain_a.balance_of(sender, &AssetId::MainToken), 10);
        assert_eq!(chain_b.balance_of(to, &AssetId::MainToken), 10);
    }









}




