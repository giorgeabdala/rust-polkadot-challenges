

pub trait Config {

}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StorageVersion {
    V1SimpleU32,
    V2U32WithFlag
}

pub struct PalletStorageSim<T: Config> {
    pub current_version: StorageVersion,

    storage_v1_value: Option<u32>,
    storage_v2_value: Option<(u32, bool)> ,
    _phantom: core::marker::PhantomData<T>
}


impl<T: Config> PalletStorageSim<T> {

    pub fn new() -> Self {
        Self {
            current_version: StorageVersion::V1SimpleU32,
            storage_v1_value: None,
            storage_v2_value: None,
            _phantom: Default::default(),
        }
    }

    pub fn set_initial_v1_value(&mut self, value: u32) {
        if self.current_version == StorageVersion::V1SimpleU32 {
            self.storage_v1_value = Some(value);
        }
    }

    pub fn get_current_v2_value(&self) -> Option<(u32, bool)> {
        match self.current_version {
            StorageVersion::V2U32WithFlag => { self.storage_v2_value }
            _ => None
        }
    }

    pub fn run_migration_if_needed(&mut self) -> u64 {
        let mut weight = 0;
       match self.current_version {
           StorageVersion::V1SimpleU32 => {
               if let Some(old_val) = self.storage_v1_value.take() {
                   self.storage_v2_value = Some((old_val, true));
                   weight = 2;
               } else { 
                   self.storage_v2_value = None;
                   weight = 1;
               }
               self.current_version = StorageVersion::V2U32WithFlag;
           }
           StorageVersion::V2U32WithFlag => {
               weight = 0;
           }
         
               
           }
        weight
    }
}

pub trait OnRuntimeUpgrade {
    fn on_runtime_upgrade(&mut self);
}

impl<T: Config> OnRuntimeUpgrade for PalletStorageSim<T>{
    fn on_runtime_upgrade(&mut self) {
        self.run_migration_if_needed();
    }
}



mod tests{
    use crate::advanced::challenge_03::{Config, PalletStorageSim, StorageVersion};

    pub struct TestConfig {}

    impl Config for TestConfig {}


    #[test]
    fn new_test() {
        let pallet: PalletStorageSim<TestConfig> = PalletStorageSim::new();
        assert_eq!(pallet.current_version, StorageVersion::V1SimpleU32);
        assert_eq!(pallet.storage_v1_value, None);
        assert_eq!(pallet.storage_v2_value, None);
    }

    #[test]
    fn set_initial_v1_value_test() {
        let mut pallet: PalletStorageSim<TestConfig> = PalletStorageSim::new();
        pallet.set_initial_v1_value(100);
        assert_eq!(pallet.storage_v1_value, Some(100));
        assert_eq!(pallet.storage_v2_value, None);
    }

    #[test]
    fn migration_with_value_existing() {
        let mut pallet: PalletStorageSim<TestConfig> = PalletStorageSim::new();
        pallet.set_initial_v1_value(100);
        let weight = pallet.run_migration_if_needed();
        assert_eq!(pallet.current_version, StorageVersion::V2U32WithFlag);
        assert_eq!(pallet.storage_v1_value, None);
        assert_eq!(pallet.storage_v2_value, Some((100, true)));
        assert_eq!(pallet.get_current_v2_value(), Some((100, true)));
        assert!(weight > 0);
    }

    #[test]
    fn migration_with_value_missing() {
        let mut pallet: PalletStorageSim<TestConfig> = PalletStorageSim::new();
        let weight = pallet.run_migration_if_needed();
        assert_eq!(pallet.current_version, StorageVersion::V2U32WithFlag);
        assert_eq!(pallet.storage_v1_value, None);
        assert_eq!(pallet.storage_v2_value, None);
        assert_eq!(pallet.get_current_v2_value(), None);
        assert!(weight > 0);
    }

    #[test]
    fn try_double_migration() {
        let mut pallet: PalletStorageSim<TestConfig> = PalletStorageSim::new();
        pallet.set_initial_v1_value(100);
        let _ = pallet.run_migration_if_needed();
        let weight = pallet.run_migration_if_needed();
        assert_eq!(pallet.current_version, StorageVersion::V2U32WithFlag);
        assert_eq!(pallet.storage_v1_value, None);
        assert_eq!(pallet.storage_v2_value, Some((100, true)));
        assert_eq!(weight, 0);
    }

    #[test]
    fn try_set_value_after_migration() {
        let mut pallet: PalletStorageSim<TestConfig> = PalletStorageSim::new();
        let _ = pallet.run_migration_if_needed();
        pallet.set_initial_v1_value(200);
        assert_eq!(pallet.storage_v1_value, None);
        assert_eq!(pallet.storage_v2_value, None);
    }






}

