pub trait StorageLike<T> {
    fn get(&self) -> Option<T>;
    fn set(&mut self, val: T);
    fn clear(&mut self);
}

#[derive(Debug)]
pub struct RAMStorage<T> {
    value: Option<T>,
}

impl<T> RAMStorage<T> {
    pub fn new() -> Self {
        RAMStorage { value: None }
    }
}

impl<T: Copy> StorageLike<T> for RAMStorage<T> {
    fn get(&self) -> Option<T> {
        self.value
    }

    fn set(&mut self, val: T) {
        self.value = Some(val);
    }

    fn clear(&mut self) {
        self.value = None;
    }
}

pub struct ConstFallback<T> {
    fixed: T,
}

impl<T> ConstFallback<T> {
    pub fn new(value: T) -> Self {
        ConstFallback { fixed: value }
    }
}

impl<T: Copy> StorageLike<T> for ConstFallback<T> {
    fn get(&self) -> Option<T> {
        Some(self.fixed)
    }

    fn set(&mut self, _val: T) {
        ()
    }

    fn clear(&mut self) {
        ()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ram_storage_works() {
        let mut store = RAMStorage::<u32>::new();
        assert_eq!(store.get(), None);
        store.set(99);
        assert_eq!(store.get(), Some(99));
        store.clear();
        assert_eq!(store.get(), None);
    }

    #[test]
    fn const_fallback_test() {
        let mut store = ConstFallback::<u32>::new(100);
        assert_eq!(store.get(), Some(100));
        store.set(50);
        assert_eq!(store.get(), Some(100));
        store.clear();
        assert_eq!(store.get(), Some(100));
    }

    #[test]
    fn ram_tuple_works() {
        let mut store = RAMStorage::<(u32, &'static str)>::new();
        assert_eq!(store.get(), None);

        let tuple_start = (100, "one hundred");
        store.set(tuple_start);
        assert_eq!(store.get(), Some(tuple_start));

        let tuple_new = (200, "two hundred");
        store.set(tuple_new);
        assert_eq!(store.get(), Some(tuple_new));

        store.clear();
        assert_eq!(store.get(), None);
    }

    #[test]
    fn ram_tuple_destructured_works() {
        let mut store = RAMStorage::new();
        store.set((10u32, "ten"));
        let (n, s) = store.get().unwrap();
        assert_eq!(n, 10);
        assert_eq!(s, "ten");
    }
}
