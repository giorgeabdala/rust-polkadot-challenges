use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Weight {
    pub ref_time: u64,
    pub proof_size: u64,
}

impl Weight {

    pub fn from_parts(ref_time: u64, proof_size: u64) -> Self {
        Self{ref_time, proof_size}
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

/// Resultados de benchmark simulados para diferentes operações
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
        Weight::from_parts(
            10_000_u64.saturating_add(5_000_u64.saturating_mul(n as u64)),
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
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            next_id: 0,
            _phantom: PhantomData
        }
    }
    pub fn create_item(
        &mut self,
        content: String,
        weight_meter: &mut WeightMeter
    ) -> Result<u32, &'static str> {
        // Simulate weight consumption by getting it from the config
        let to_consume = T::WeightInfo::create_item();
        weight_meter.consume(to_consume)?;

        let id = self.next_id;
        self.items.insert(id, content);
        self.next_id = self.next_id.saturating_add(1);
        Ok(id)
    }

      pub fn update_item(
        &mut self,
        id: u32,
        new_content: String,
        weight_meter: &mut WeightMeter
    ) -> Result<(), &'static str> {
          let to_consume = T::WeightInfo::update_item();
          weight_meter.consume(to_consume)?;
          self.items.get_mut(&id).ok_or("Item not found")?;
          self.items.insert(id, new_content);
          Ok(())

    }
    pub fn delete_item(&mut self, id: u32, weight_meter: &mut WeightMeter) -> Result<(), &'static str> {
        let to_consume = T::WeightInfo::delete_item();
        weight_meter.consume(to_consume)?;
        self.items.remove(&id)
            .ok_or("Item not found")?;
        Ok(())
    }

    pub fn batch_delete(&mut self, ids: Vec<u32>, weight_meter: &mut WeightMeter) -> Result<u32, &'static str> {
        let count = ids.len() as u32;
        let to_consume = T::WeightInfo::batch_operation(count);
        weight_meter.consume(to_consume)?;
        let mut deleted_count = 0;
        for id in ids {
            if self.items.remove(&id).is_some() {
                deleted_count += 1;
            }
        }
        Ok(deleted_count)
    }
}

pub struct FeeCalculator {
    pub ref_time_fee: u64,
    pub proof_size_fee: u64,
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
    limit: Weight,
}

impl WeightMeter {
    pub fn new(limit: Weight) -> Self {
        Self {
            consumed: Weight::zero(),
            limit,
        }
    }
    pub fn consume(&mut self, weight_to_consume: Weight) -> Result<(), &'static str> {
        let new_ref_time = self.consumed.ref_time.saturating_add(weight_to_consume.ref_time);
        let new_proof_size = self.consumed.proof_size.saturating_add(weight_to_consume.proof_size);

        if new_ref_time > self.limit.ref_time || new_proof_size > self.limit.proof_size {
            return Err("Weight limit exceeded");
        }

        self.consumed = Weight::from_parts(new_ref_time, new_proof_size);
        Ok(())
    }
    pub fn remaining(&self) -> Weight {
        Weight::from_parts(
            self.limit.ref_time.saturating_sub(self.consumed.ref_time),
            self.limit.proof_size.saturating_sub(self.consumed.proof_size),
        )
    }
    pub fn consumed(&self) -> Weight {
        self.consumed
    }
}

mod tests {
    use crate::advanced::challenge_02::{BenchmarkWeights, Config, FeeCalculator, Pallet, Weight, WeightInfo, WeightMeter};

    pub struct TestConfig{}
    impl Config for TestConfig {
        type WeightInfo = BenchmarkWeights;
    }


    fn calculator_expected_fee(expected_weight: Weight) -> u64 {
        let calculator = FeeCalculator::new();
        let expected_ref_time = expected_weight.ref_time.saturating_mul(calculator.ref_time_fee);
        let expected_proof_size = expected_weight.proof_size.saturating_mul(calculator.proof_size_fee);
        expected_ref_time.saturating_add(expected_proof_size)
    }


    #[test]
    fn benchmark_weight_and_calculatorfee_for_create_item() {
        let expected_weight = Weight::from_parts(25_000, 1024);
        let actual_weigt = BenchmarkWeights::create_item();
        assert_eq!(actual_weigt, expected_weight);
        let expected_fee =calculator_expected_fee(expected_weight);
        let calculator = FeeCalculator::new();
        let fee = calculator.calculate_fee(actual_weigt);
        assert_eq!(fee, expected_fee);
    }

    #[test]
    fn benchmark_weigh_and_calculatorfee_for_update_item() {
        let expected_weight = Weight::from_parts(20_000, 512);
        let actual_weigt = BenchmarkWeights::update_item();
        assert_eq!(actual_weigt, expected_weight);
        let expected_fee =calculator_expected_fee(expected_weight);
        let calculator = FeeCalculator::new();
        let fee = calculator.calculate_fee(actual_weigt);
        assert_eq!(fee, expected_fee);
    }

    #[test]
    fn benchmark_weigh_for_delete_item() {
        let expected_weight = Weight::from_parts(15_000, 256);
        let actual_weigt = BenchmarkWeights::delete_item();
        assert_eq!(actual_weigt, expected_weight);
        let expected_fee =calculator_expected_fee(expected_weight);
        let calculator = FeeCalculator::new();
        let fee = calculator.calculate_fee(actual_weigt);
        assert_eq!(fee, expected_fee);
    }

    #[test]
    fn benchmark_weigh_for_batch_operations() {
        let quantity_batch = 5;
        let expected_weight = Weight::from_parts(
            10_000 + (5_000 * quantity_batch as u64),
            256 + (128 * quantity_batch as u64)
        );
        let actual_weigt = BenchmarkWeights::batch_operation(quantity_batch as u32);
        assert_eq!(actual_weigt, expected_weight);
        let expected_fee =calculator_expected_fee(expected_weight);
        let calculator = FeeCalculator::new();
        let fee = calculator.calculate_fee(actual_weigt);
        assert_eq!(fee, expected_fee);
    }

    #[test]
    fn weightmeter_test() {
        let limit_weight = Weight {ref_time: 10_000, proof_size: 512};
        let consumed_weight = Weight {ref_time: 9_000, proof_size: 256};
        let mut meter = WeightMeter::new(limit_weight);
        let result = meter.consume(consumed_weight);
        assert!(result.is_ok());
        let remained_weight = Weight{ref_time: 1_000, proof_size:256};
        assert_eq!(remained_weight, meter.remaining());
        assert!(meter.consume(remained_weight).is_ok());
        assert!(meter.consume(remained_weight).is_err());
    }

    #[test]
    fn pallet_create_item_consumes_correct_weight_and_succeeds() {
        let mut pallet = Pallet::<TestConfig>::new();
        let limit = Weight::from_parts(100_000, 2048);
        let mut weight_meter = WeightMeter::new(limit);
        let content = String::from("Test Item 1");
        let expected_weight = BenchmarkWeights::create_item();

        let id_result = pallet.create_item(content.clone(), &mut weight_meter);
        assert!(id_result.is_ok());
        let id = id_result.unwrap();
        assert_eq!(id, 0);
        assert_eq!(pallet.items.get(&0), Some(&content));
        let final_consumed_by_meter = weight_meter.consumed();
        assert_eq!(final_consumed_by_meter.ref_time, expected_weight.ref_time);
        assert_eq!(final_consumed_by_meter.proof_size, expected_weight.proof_size);
    }

    #[test]
    fn pallet_update_item_consumes_weight_and_succeeds() {
        let mut pallet = Pallet::<TestConfig>::new();
        let mut wm_setup = WeightMeter::new(Weight::from_parts(100_000, 2048));
        let item_id = pallet.create_item("Original".to_string(), &mut wm_setup).unwrap();

        let mut wm_update = WeightMeter::new(Weight::from_parts(100_000, 2048));
        let new_content = "Updated Content".to_string();
        let expected_weight_for_update = BenchmarkWeights::update_item();

        let result = pallet.update_item(item_id, new_content.clone(), &mut wm_update);
        assert!(result.is_ok());
        assert_eq!(pallet.items.get(&item_id), Some(&new_content));
        assert_eq!(wm_update.consumed(), expected_weight_for_update);
    }

    #[test]
    fn pallet_delete_item_consumes_weight_and_succeeds() {
        let mut pallet = Pallet::<TestConfig>::new();
        let mut weight_meter_setup = WeightMeter::new(Weight::from_parts(100_000, 2048)); // Para o setup
        let content = String::from("To Be Deleted");
        let item_id = pallet.create_item(content, &mut weight_meter_setup).unwrap();

        let mut weight_meter_delete = WeightMeter::new(Weight::from_parts(100_000, 2048));
        let expected_weight_for_delete = BenchmarkWeights::delete_item();

        let result = pallet.delete_item(item_id, &mut weight_meter_delete);
        assert!(result.is_ok(), "delete_item failed unexpectedly");
        assert!(pallet.items.get(&item_id).is_none(), "Item should have been deleted");
        assert_eq!(weight_meter_delete.consumed(), expected_weight_for_delete);
    }

    #[test]
    fn pallet_create_item_fails_if_weight_limit_exceeded() {
        let mut pallet = Pallet::<TestConfig>::new();
        let limit = Weight::from_parts(10, 10);
        let mut weight_meter = WeightMeter::new(limit);
        let content = String::from("Test Item Will Fail");
        let result = pallet.create_item(content.clone(), &mut weight_meter);
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Weight limit exceeded"));
        assert!(pallet.items.get(&0).is_none());
        assert_eq!(weight_meter.consumed(), Weight::zero());
    }
    
}







