// Fundamental runtime types
pub type AccountId = String; // Simplified
pub type BlockNumber = u64;
pub type Hash = [u8; 32];
pub type Balance = u128;

pub mod system {
    
    use std::collections::HashMap;

    pub trait Config {
        type AccountId: Clone + Eq + std::hash::Hash + core::fmt::Debug;
        type BlockNumber: Clone + Copy + Default + PartialEq + PartialOrd + core::fmt::Debug;
        type Hash: Clone + PartialEq + core::fmt::Debug;
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum Event<T: Config> {
        NewAccount { account: T::AccountId },
        ExtrinsicSuccess { account: T::AccountId },
    }

    pub struct Pallet<T: Config> {
        account_nonces: HashMap<T::AccountId, u32>,
        current_block_number: T::BlockNumber,
        events: Vec<Event<T>>,
        _phantom: core::marker::PhantomData<T>,
    }

    impl<T: Config> Pallet<T> {
        pub fn new() -> Self {
            Self {
                account_nonces: HashMap::new(),
                current_block_number: T::BlockNumber::default(),
                events: Vec::new(),
                _phantom: core::marker::PhantomData,
            }
        }

        pub fn inc_account_nonce(&mut self, account: &T::AccountId) {
            let nonce = self.account_nonces.entry(account.clone()).or_insert(0);
            *nonce += 1;

            if *nonce == 1 {
                self.events.push(Event::NewAccount { account: account.clone() });
            }
        }

        pub fn account_nonce(&self, account: &T::AccountId) -> u32 {
            self.account_nonces.get(account).copied().unwrap_or_default()
        }

        pub fn set_block_number(&mut self, number: T::BlockNumber) {
            self.current_block_number = number;
        }

        pub fn block_number(&self) -> T::BlockNumber {
            self.current_block_number
        }

        pub fn record_extrinsic_success(&mut self, account: T::AccountId) {
            self.events.push(Event::ExtrinsicSuccess { account });
        }

        pub fn take_events(&mut self) -> Vec<Event<T>> {
            std::mem::take(&mut self.events)
        }
    }
}


pub mod balances {
    use super::*;
    use std::collections::HashMap;

    pub trait Config: system::Config {
        type Balance: Clone + Copy + Default + PartialEq + PartialOrd + core::fmt::Debug +
        core::ops::Add<Output = Self::Balance> +
        core::ops::Sub<Output = Self::Balance>;
    }

    // Event system: emitted to off-chain consumers for state change notifications
    #[derive(Clone, Debug, PartialEq)]
    pub enum Event<T: Config> {
        Transfer {
            from: T::AccountId,
            to: T::AccountId,
            amount: T::Balance
        },
        BalanceSet {
            account: T::AccountId,
            balance: T::Balance
        },
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum Error {
        InsufficientBalance,
        ZeroAmount,
    }

    pub struct Pallet<T: Config> {
        balances: HashMap<T::AccountId, T::Balance>,
        events: Vec<Event<T>>,
        _phantom: core::marker::PhantomData<T>,
    }

    impl<T: Config> Pallet<T> {
        pub fn new() -> Self {
            Self {
                balances: HashMap::new(),
                events: Vec::new(),
                _phantom: core::marker::PhantomData,
            }
        }

        pub fn set_balance(&mut self, account: T::AccountId, balance: T::Balance) {
            self.balances.insert(account.clone(), balance);
            self.events.push(Event::BalanceSet { account, balance });
        }

        pub fn transfer(
            &mut self,
            from: T::AccountId,
            to: T::AccountId,
            amount: T::Balance,
        ) -> Result<(), Error> {
            if amount == T::Balance::default() {
                return Err(Error::ZeroAmount);
            }

            let from_balance = self.balances.get(&from).copied().unwrap_or_default();

            if from_balance < amount {
                return Err(Error::InsufficientBalance);
            }

            let to_balance = self.balances.get(&to).copied().unwrap_or_default();

            // Update balances
            self.balances.insert(from.clone(), from_balance - amount);
            self.balances.insert(to.clone(), to_balance + amount);

            self.events.push(Event::Transfer { from, to, amount });
            Ok(())
        }

        pub fn balance(&self, account: &T::AccountId) -> T::Balance {
            self.balances.get(account).copied().unwrap_or_default()
        }

        pub fn take_events(&mut self) -> Vec<Event<T>> {
            std::mem::take(&mut self.events)
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum RuntimeEvent<T: RuntimeConfig> {
    System(system::Event<T>),
    Balances(balances::Event<T>),
}

impl<T: RuntimeConfig> From<system::Event<T>> for RuntimeEvent<T> {
    fn from(event: system::Event<T>) -> Self {
        RuntimeEvent::System(event)
    }
}

impl<T: RuntimeConfig> From<balances::Event<T>> for RuntimeEvent<T> {
    fn from(event: balances::Event<T>) -> Self {
        RuntimeEvent::Balances(event)
    }
}


pub trait RuntimeConfig: system::Config + balances::Config {}


pub struct Runtime<T: RuntimeConfig> {
    pub system: system::Pallet<T>,
    pub balances: balances::Pallet<T>,
    events: Vec<RuntimeEvent<T>>,
    _phantom: core::marker::PhantomData<T>,
}

impl<T: RuntimeConfig> Runtime<T> {
    pub fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
            events: Vec::new(),
            _phantom: core::marker::PhantomData,
        }
    }

    pub fn execute_transfer(
        &mut self,
        origin: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> Result<(), balances::Error> {
        self.balances.transfer(origin.clone(), to, amount)?;
        self.system.inc_account_nonce(&origin);
        self.system.record_extrinsic_success(origin);
        self.collect_events();
        Ok(())
    }

    pub fn finalize_block(&mut self, block_number: T::BlockNumber) {
        self.system.set_block_number(block_number);
        self.collect_events();
    }
    

    // Genesis configuration - set initial state
    pub fn genesis_config(
        &mut self,
        initial_balances: Vec<(T::AccountId, T::Balance)>,
    ) {
        for (account, balance) in initial_balances {
            self.balances.set_balance(account, balance);
        }

        // Collect genesis events
        self.collect_events();
    }

    // Collect events from all pallets
    fn collect_events(&mut self)  {
        // Collect system events
        for event in self.system.take_events() {
            self.events.push(RuntimeEvent::System(event));
        }

        // Collect balances events
        for event in self.balances.take_events() {
            self.events.push(RuntimeEvent::Balances(event));
        }
    }

    // Get all runtime events
    pub fn take_events(&mut self) -> Vec<RuntimeEvent<T>> {
        std::mem::take(&mut self.events)
    }

    // Query methods
    pub fn account_balance(&self, account: &T::AccountId) -> T::Balance {
        self.balances.balance(account)
    }

    pub fn account_nonce(&self, account: &T::AccountId) -> u32 {
        self.system.account_nonce(account)
    }

    pub fn current_block(&self) -> T::BlockNumber {
        self.system.block_number()
    }
}

// Test configuration
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TestRuntimeConfig;

impl system::Config for TestRuntimeConfig {
    type AccountId = AccountId;
    type BlockNumber = BlockNumber;
    type Hash = Hash;
}

impl balances::Config for TestRuntimeConfig {
    type Balance = Balance;
}

impl RuntimeConfig for TestRuntimeConfig {}


#[cfg(test)]
mod tests {
    use crate::advanced::challenge_12::balances::{Error, Event};
    
    
    
    use super::*;

    type TestRuntime = Runtime<TestRuntimeConfig>;

    
    #[test]
    fn genesis_config_test() {
        let mut runtime = TestRuntime::new();
        let alice = "alice".to_string();
        let bob = "bob".to_string();
        runtime.genesis_config(vec![(alice.clone(), 1000), (bob.clone(), 500)]);

        assert_eq!(runtime.account_balance(&alice), 1000);
        assert_eq!(runtime.account_balance(&bob), 500);
        runtime.collect_events();
        let events = runtime.take_events();
        assert_eq!(events[0], RuntimeEvent::Balances(Event::BalanceSet {account: alice.clone(), balance: 1000}));
        assert_eq!(events[1], RuntimeEvent::Balances(Event::BalanceSet {account: bob.clone(), balance: 500}));
    }

    #[test]
    fn execute_transfer_test() {
        let mut runtime = TestRuntime::new();
        let alice = "alice".to_string();
        let bob = "bob".to_string();
        runtime.genesis_config(vec![(alice.clone(), 1000), (bob.clone(), 500)]);
        runtime.collect_events();
        runtime.take_events();

        let transfer_result = runtime.execute_transfer(alice.clone(), bob.clone(), 250);
        assert!(transfer_result.is_ok());

        assert_eq!(runtime.account_balance(&alice), 750);
        assert_eq!(runtime.account_balance(&bob), 750);

        assert_eq!(runtime.account_nonce(&alice), 1);
        assert_eq!(runtime.account_nonce(&bob), 0);

        runtime.collect_events();
        let events = runtime.take_events();
        assert_eq!(events.len(), 3);

        assert_eq!(events[0], RuntimeEvent::System(system::Event::NewAccount { account: alice.clone() }));
        assert_eq!(events[1], RuntimeEvent::System(system::Event::ExtrinsicSuccess { account: alice.clone()}));
        assert_eq!(events[2], RuntimeEvent::Balances(Event::Transfer {from: alice.clone(), to: bob.clone(), amount: 250}));
    }

    #[test]
    fn transfer_with_insufficent_balance_fail() {
        let mut runtime = TestRuntime::new();
        let alice = "alice".to_string();
        let bob = "bob".to_string();
        runtime.genesis_config(vec![(alice.clone(), 100), (bob.clone(), 100)]);
        let transfer_result = runtime.execute_transfer(alice.clone(), bob.clone(), 250);

        assert!(transfer_result.is_err());
        assert_eq!(transfer_result, Err(Error::InsufficientBalance));
        assert_eq!(runtime.account_balance(&alice), 100);
        assert_eq!(runtime.account_balance(&bob), 100);
        assert_eq!(runtime.account_nonce(&alice), 0);
        assert_eq!(runtime.account_nonce(&bob), 0);
    }

    #[test]
    fn finalize_block_test() {
        let mut runtime = TestRuntime::new();
        let alice = "alice".to_string();
        let bob = "bob".to_string();
        runtime.genesis_config(vec![(alice.clone(), 100), (bob.clone(), 100)]);
        assert_eq!(runtime.current_block(), 0);
        runtime.finalize_block(1);
        assert_eq!(runtime.current_block(), 1);
    }
    #[test]
    fn transfer_with_zero_amount_fails() {
        let mut runtime = TestRuntime::new();
        let alice = "alice".to_string();
        let bob = "bob".to_string();
        runtime.genesis_config(vec![(alice.clone(), 1000)]);
        runtime.take_events(); // Limpa eventos do genesis

        let result = runtime.execute_transfer(alice.clone(), bob.clone(), 0);

        assert_eq!(result, Err(Error::ZeroAmount));
        assert_eq!(runtime.account_balance(&alice), 1000);
        assert_eq!(runtime.account_nonce(&alice), 0);
        assert!(runtime.take_events().is_empty());
    }









}