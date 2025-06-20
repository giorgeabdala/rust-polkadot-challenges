#![allow(dead_code)]
use std::collections::HashMap;
use std::marker::PhantomData;
use std::task::Context;

#[derive(Debug, PartialEq)]
pub struct Weight {
    pub ref_time: u64,
    pub proof_size: u64,
}

impl Weight {
    pub fn from_parts(ref_time: u64, proof_size: u64) -> Self {
        Self {ref_time, proof_size}
    }

    pub fn zero() -> Self {
        Self::from_parts(0,0)
    }
}

pub trait WeightInfo {
    fn create_item() -> Weight;
    fn update_item() -> Weight;
    fn delete_item() -> Weight;
    fn batch_operation(n: u32) -> Weight;

}

pub struct BenchmarkWeights;

impl WeightInfo for BenchmarkWeights {
    fn create_item() -> Weight {
        Weight::from_parts(25_000, 1024)
    }

    fn update_item() -> Weight {
        Weight::from_parts(20_000, 512)
    }

    fn delete_item() -> Weight {
        Weight::from_parts(15_000, 256)
    }

    fn batch_operation(n: u32) -> Weight {
        Weight::from_parts(10_000_u64.saturating_add(5_000_u64.saturating_mul(n as u64)),
            256_u64.saturating_add(128_u64.saturating_mul(n as u64))
        )
    }

}

pub trait Config {
    type WeightInfo: WeightInfo;
}

pub struct Pallet<T: Config> {
    items: HashMap<u32, String>,
    next_id: u32,
    _phantom: PhantomData<T>
}

impl <T: Config> Pallet<T> {

    pub fn create_item(&mut self, content: String) -> Result<u32, &'static str> {
        let _weight = T:: WeightInfo::create_item();

        let id = self.next_id;
        self.items.insert(id, content);
        self.next_id = self.next_id.saturating_add(1);
        Ok(id)
    }

    pub fn update_item(&mut self, id: u32, new_content: String) -> Result<(), &'static str> {
        let _weight = T::WeightInfo::update_item();

        self.items.get_mut(&id).ok_or("Item not found")?;
        self.items.insert(id, new_content);
        Ok(())
    }

    pub fn batch_delete(&mut self, ids: Vec<u32>) -> Result<u32, &'static str> {
        let count = ids.len() as u32;
        let _weight =T::WeightInfo::batch_operation(count);

        let mut deleted = 0;
            for id in ids {
                if self.items.remove(&id).is_some() {
                    deleted +=1;
                }
            }
        Ok(deleted)
    }
}

pub struct FeeCalculator {
    pub ref_time_fee: u64,
    pub proof_size_fee: u64
}

impl FeeCalculator {
    pub fn new() -> Self {
        Self {
            ref_time_fee: 1,
            proof_size_fee: 2,
        }
    }

    pub fn calculate_fee(&self, weight: Weight) -> u64 {
        let ref_time_cost = weight.ref_time.saturating_mul(self.ref_time_fee);
        let proof_size_cost = weight.proof_size.saturating_mul(self.proof_size_fee);
        ref_time_cost.saturating_add(proof_size_cost)
    }
}

pub struct WeightMeter {
    consumed: Weight,
    limit: Weight
}

impl WeightMeter {
    pub fn new(limit: Weight) -> Self {
        Self {
            consumed: Weight::zero(),
            limit,
        }
    }

    pub fn consume(&mut self, weight: Weight) -> Result<(), &'static str> {
        let new_ref_time = self.consumed.ref_time.saturating_add(weight.ref_time);
        let new_proof_size = self.consumed.proof_size.saturating_add(weight.proof_size);

        if new_ref_time > self.limit.ref_time || new_proof_size > self.limit.proof_size {
            return Err("Weight limit exceeded");
        }

        self.consumed = Weight::from_parts(new_ref_time, new_proof_size);
        Ok(())
    }

    pub fn remaining(&self) -> Weight {
        Weight::from_parts(
            self.limit.ref_time.saturating_sub(self.consumed.ref_time),
            self.limit.proof_size.saturating_sub(self.consumed.proof_size)
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::marker::PhantomData;
    use crate::advanced::challenge_02::{BenchmarkWeights, Config, Pallet, Weight, WeightInfo};



    pub struct TestConfig{}
    impl Config for TestConfig {
        type WeightInfo = BenchmarkWeights;
    }

    fn test_benchmark_weights_create_item_returns_correct_weight() {
        let expected_weight = Weight::from_parts(25_000, 1024);
        let actual_weigth = BenchmarkWeights::create_item();
        assert_eq!(
            actual_weigth,
            expected_weight,
            "The weight for create_item() should be ref_time=25_000, proof_size=1024"
        );
    }



    #[test]
    fn create_item_test() {
        let mut pallet = Pallet::<TestConfig> {
            items: HashMap::new(),
            _phantom: PhantomData,
            next_id: 0
        };

        let item_content = String::from("My First item");
        match pallet.create_item(item_content.clone()) {
            Ok(id) => {
                assert_eq!(id, 0);
                assert_eq!(1, pallet.next_id);
                assert_eq!(pallet.items.get(&0), Some(&item_content));
            }
            Err(e) => panic!("Failed to create item")
        }

        let item_content2 = String::from("My Second item");
        match pallet.create_item(item_content2.clone()) {
            Ok(id) => {
                assert_eq!(id, 1);
                assert_eq!(2, pallet.next_id);
                assert_eq!(pallet.items.get(&1), Some(&item_content2));
            }
            Err(e) => panic!("Failed to create item")
        }


    }

}