use std::collections::HashMap;

trait Storable {

    fn to_bytes(&self) -> Vec<u8> ;

    fn from_bytes(data: &[u8]) -> Result<Self, String> where Self: Sized ;

    fn storage_key(&self) -> String { "default".to_string() }

}

struct Storage<T> where T: Storable + Clone{
    items: Vec<T>,
    capacity: usize
}

impl<T: Storable + Clone> Storage<T> {
    fn new(capacity: usize) -> Self {
        Storage{items: Vec::new(), capacity}
    }

    fn store(&mut self, item: T) -> Result<usize, String>  {
        if self.items.len() >= self.capacity{return Err("Storage is Full".to_string())}
        self.items.push(item);
        Ok(self.items.iter().len() - 1)
    }

    fn retrieve(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    fn len(&self) -> usize {
       self.items.len()
    }

    fn is_full(&self) -> bool {
        self.items.len() >= self.capacity
    }

}

trait StorageMap {
    type Item: Storable;
    type Key;

    fn get(&self, key: &Self::Key) -> Option<&Self::Item>;
    fn insert(&mut self, key: Self::Key, value: Self::Item) -> Option<Self::Item>;
}

struct KeyValueStorage<K, V> {
    data: HashMap<K, V>
}

impl<K, V> KeyValueStorage<K, V> {
    fn new() -> Self{
        KeyValueStorage{data: HashMap::new()}
    }
}

impl<K, V> StorageMap for KeyValueStorage<K, V>
where
    K: Clone + std::hash::Hash + Eq,
    V: Storable + Clone

{
    type Item = V;
    type Key = K;

    fn get(&self, key: &Self::Key) -> Option<&Self::Item> {
        self.data.get(key)
    }

    fn insert(&mut self, key: Self::Key, value: Self::Item) -> Option<Self::Item> {
        self.data.insert(key, value)
    }
}

impl Storable for String {
    fn to_bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }

    fn from_bytes(data: &[u8]) -> Result<Self, String>
    where
        Self: Sized
    {
        String::from_utf8(data.to_vec()).map_err(|e| e.to_string())
    }

    fn storage_key(&self) -> String {
        self.to_string()
    }
}

mod tests {
    use crate::medium::challenge_02::{KeyValueStorage, Storage, StorageMap};

    #[test]

    fn storage_store_and_retrieve_test() {
        let mut storage: Storage<String> = Storage::new(10);
        let item = "item".to_string();
        let stored_result = storage.store(item.clone());
        assert!(stored_result.is_ok());
        let retrieved_opt = storage.retrieve(stored_result.unwrap());
        assert!(retrieved_opt.is_some());
        assert_eq!(retrieved_opt.unwrap(), &item);
    }

    #[test]
    fn storage_len_test() {
        let mut storage: Storage<String> = Storage::new(10);
        let [item, item_two] = ["item".to_string(), "item_two".to_string()];
        let _ = storage.store(item.clone());
        assert_eq!(storage.len(), 1);
        let _ = storage.store(item_two.clone());
        assert_eq!(storage.len(), 2);
    }

    #[test]
    fn is_full_test() {
        let mut storage: Storage<String> = Storage::new(2);
        let [item, item_two] = ["item".to_string(), "item_two".to_string()];
        let _ = storage.store(item.clone());
        assert!(!storage.is_full());
        let _ = storage.store(item_two.clone());
        assert!(storage.is_full());
    }


    #[test]
    fn key_storage_insert_and_get_test () {
        let mut key_value_storage: KeyValueStorage<i32, String> = KeyValueStorage::new();
        let value = "Value".to_string();
        let _ = key_value_storage.insert(1, value.clone());
        let value_found_opt = key_value_storage.get(&1);
        assert!(value_found_opt.is_some());
        assert_eq!(*value_found_opt.unwrap(), value);
    }


}