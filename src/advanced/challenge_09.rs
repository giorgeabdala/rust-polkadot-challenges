

pub trait Config {
    type AccountId: Clone + PartialEq + core::fmt::Debug;
    type BlockNumber: Copy + PartialOrd + core::ops::Add<Output = Self::BlockNumber> ;
    type TaskLifetime: Get<Self::BlockNumber>; 
}
pub trait Get<V> {
    fn get() -> V;
}

pub struct Task<AccountId, BlockNumber> {
    pub id: u32 , 
    pub creator: AccountId,
    pub created_at: BlockNumber
}


#[derive(Clone, Debug, PartialEq)]
pub enum Event<T: Config> {
    TaskCreated { task_id: u32, creator: T::AccountId },
    TaskExpired { task_id: u32 },
    RuntimeUpgraded { old_version: u32, new_version: u32 },
}


#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    BadOrigin,
    MaxTasksReached,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Origin<AccountId> {
    Signed(AccountId),
    Root,
}

use std::collections::HashMap;

pub struct Pallet<T: Config> {
    tasks: HashMap<u32, Task<T::AccountId, T::BlockNumber>>,
    next_task_id: u32,
    runtime_version: u32,
    emitted_events: Vec<Event<T>>,
    _phantom: core::marker::PhantomData<T>, 
}


impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            next_task_id: 1,
            runtime_version: 1,
            emitted_events: Vec::new(),
            _phantom: core::marker::PhantomData,
        }
    }
    
    pub fn create_task(&mut self, origin: Origin<T::AccountId>, current_block: T::BlockNumber) -> Result<(), Error> {
        let next_id = self.next_task_id;
        let account_id = Self::ensure_signed(origin)?;
        if self.tasks.len() >= 50 {return Err(Error::MaxTasksReached)}
        let task = Task{id: next_id, creator: account_id.clone(), created_at: current_block};
        self.tasks.insert(next_id, task);
        self.deposit_event(Event::TaskCreated {task_id: next_id, creator: account_id });
        self.next_task_id+=1;
        Ok(())
     }
    
    pub fn on_initialize(&mut self, block_number: T::BlockNumber) -> u64 {
        10_000
    }

    pub fn on_finalize(&mut self, block_number: T::BlockNumber) -> u64 {
        let task_lifetime = T::TaskLifetime::get();
        let initial_task_count = self.tasks.len();
        let mut weight = 10_000; 

        let expired_task_ids: Vec<u32> = self.tasks
            .iter()
            .filter_map(|(task_id, task)| {
                if task.created_at + task_lifetime <= block_number {
                    Some(*task_id)
                } else {
                    None
                }
            })
            .collect();

        for task_id in expired_task_ids {
            self.tasks.remove(&task_id);
            self.deposit_event(Event::TaskExpired { task_id });
        }

        let tasks_removed = initial_task_count - self.tasks.len();
        if tasks_removed > 0 {
            weight = 15_000; 
        }

        weight
    }


    pub fn on_runtime_upgrade(&mut self) -> u64 {
        let old_version = self.runtime_version;
        self.runtime_version += 1;
        self.deposit_event(Event::RuntimeUpgraded {old_version, new_version: self.runtime_version});
        50_000
    }



    fn deposit_event(&mut self, event: Event<T>) {
        self.emitted_events.push(event);
    }

    pub fn take_events(&mut self) -> Vec<Event<T>> {
        std::mem::take(&mut self.emitted_events)
    }

    fn ensure_signed(origin: Origin<T::AccountId>) -> Result<T::AccountId, Error> {
        match origin {
            Origin::Signed(account) => Ok(account),
            _ => Err(Error::BadOrigin),
        }
    }

    pub fn get_task(&self, task_id: u32) -> Option<&Task<T::AccountId, T::BlockNumber>> {
        self.tasks.get(&task_id)
    }

    pub fn get_active_tasks_count(&self) -> u32 {
        self.tasks.len() as u32
    }

    pub fn get_runtime_version(&self) -> u32 {
        self.runtime_version
    }
    
}

#[cfg(test)]

mod tests {
    use crate::advanced::challenge_09::{Config, Get};
    use super::*;

    #[derive(Debug, PartialEq)]
    struct TestConfig;
    struct TestTaskLifetime;

    impl Config for TestConfig {
        type AccountId = u32;
        type BlockNumber = u64;
        type TaskLifetime = TestTaskLifetime;
    }
    impl Get<u64> for TestTaskLifetime {
        fn get() -> u64{5}
    }

    #[test]
    fn on_finalize_test() {
        let mut pallet = Pallet::<TestConfig>::new();
        let result = pallet.create_task(Origin::Signed(1), 1);
        assert!(result.is_ok());
        assert_eq!(pallet.on_initialize(1), 10_000);
        assert_eq!(pallet.tasks.len(), 1);

        assert_eq!(pallet.on_finalize(2), 10_000);

        assert_eq!(pallet.on_finalize(7), 15_000);
        assert_eq!(pallet.tasks.len(), 0);

        let events = pallet.take_events();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0], Event::TaskCreated {task_id: 1, creator: 1});
        assert_eq!(events[1], Event::TaskExpired {task_id: 1});
    }

    #[test]
    fn on_runtime_upgrade_test() {
        let mut pallet = Pallet::<TestConfig>::new();
        assert_eq!(pallet.runtime_version, 1);
        assert_eq!(pallet.on_runtime_upgrade(), 50_000);
        assert_eq!(pallet.runtime_version, 2);

        let events = pallet.take_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0], Event::RuntimeUpgraded {old_version: 1,new_version: 2})
    }

    #[test]
    fn create_task_fail_bad_origin_fail() {
        let mut pallet = Pallet::<TestConfig>::new();
        let result = pallet.create_task(Origin::Root, 1);
        assert!(result.is_err());
        assert_eq!(result, Err(Error::BadOrigin));
    }

    #[test]
    fn create_task_max_tasks_fail() {
        let mut pallet = Pallet::<TestConfig>::new();

        for i in 1..51 {
            let result = pallet.create_task(Origin::Signed(1), 1);
            assert!(result.is_ok());
        }
        let result = pallet.create_task(Origin::Signed(1), 1);
        assert!(result.is_err());
    }

    #[test]
    fn create_task_succeeds() {
        let mut pallet = Pallet::<TestConfig>::new();
        let creator_account = 1u32;
        let creation_block = 10u64;
        assert_eq!(pallet.create_task(Origin::Signed(creator_account), creation_block), Ok(()));
        assert_eq!(pallet.get_active_tasks_count(), 1);
        let task = pallet.get_task(1).expect("Task should exist");
        assert_eq!(task.id, 1);
        assert_eq!(task.creator, creator_account);
        assert_eq!(task.created_at, creation_block);
        let events = pallet.take_events();
        assert_eq!(events, vec![
            Event::TaskCreated { task_id: 1, creator: creator_account }
        ]);
    }
}
