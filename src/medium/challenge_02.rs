
const MAX_VALIDATORS: usize = 5;
type ValidatorId = u32;

#[derive(Copy, Clone, Debug, PartialEq)]
enum ValidatorStatus {
    Active,
    Inactive,
}

type ValidatorSlot = (ValidatorId, Option<ValidatorStatus>);

fn initialize_validators(ids: &[ValidatorId]) -> [ValidatorSlot; MAX_VALIDATORS] {
    let mut validators = [(0, None);MAX_VALIDATORS];
    for (index, &current_id) in ids.iter().take(MAX_VALIDATORS).enumerate() {
        validators[index] = (current_id, None);
    }
    validators
}

fn set_validator_status(validators: &mut [ValidatorSlot;MAX_VALIDATORS], id: ValidatorId, status: ValidatorStatus) -> bool {
    for slot in validators.iter_mut() {
        if  slot.0 == id {
            slot.1 = Some(status);
            return true;
        }
    }
    false
}

fn get_validator_status(validators: &[ValidatorSlot; MAX_VALIDATORS], id: ValidatorId)
    -> Option<ValidatorSlot> {
    for validator_tuple in validators.iter(){
        if validator_tuple.0 == id {
            return Some(*validator_tuple);
        }
    }
    None
}

mod tests {
    use super::*;

    fn create_validators() -> [ValidatorSlot; MAX_VALIDATORS] {
        let mut ids = [0;MAX_VALIDATORS];
        for i in 0..ids.len() {ids[i] = i as ValidatorId}
        initialize_validators(&ids)
    }

    #[test]
    fn initialize_validators_test() {
        let validators = create_validators();
        for i in 0..validators.len() {
            assert_eq!(validators[i].1, None);
        }

        let less_ids_data = [10, 20]; // Exemplo com 2 IDs
        let validators_less = initialize_validators(&less_ids_data);
        assert_eq!(validators_less[0], (10, None));
        assert_eq!(validators_less[1], (20, None));
        for i in less_ids_data.len()..MAX_VALIDATORS {
            assert_eq!(validators_less[i], (0, None), "Slot {} should be (0, None)", i);
        }

        let more_ids_data = [1, 2, 3, 4, 5, 6, 7]; // Mais que MAX_VALIDATORS
        let validators_more = initialize_validators(&more_ids_data);
        for i in 0..MAX_VALIDATORS {
            assert_eq!(validators_more[i], (more_ids_data[i], None), "Validator {} does not match", more_ids_data[i]);
        }
    }

    #[test]
    fn set_validator_status_test() {
        let mut validators = create_validators();
        set_validator_status(& mut validators, 1, ValidatorStatus::Inactive);
        assert_eq!(validators[1].1.unwrap(), ValidatorStatus::Inactive);

    }

    #[test]
    fn set_validator_status_non_existing_id() {
        let mut validators = create_validators();
        let result = set_validator_status(&mut validators, 99, ValidatorStatus::Active);
        assert!(!result);
    }

    #[test]
    fn get_validator_status_test() {
        let mut validators = create_validators();
        set_validator_status(&mut validators, 1, ValidatorStatus::Active);
        let status = get_validator_status(&validators, 1);
        assert_eq!(status.unwrap().1.unwrap(), ValidatorStatus::Active);
    }

    #[test]
    fn get_validator_status_non_existing_id() {
        let validators = create_validators();
        let status = get_validator_status(&validators, 99);
        assert!(status.is_none());
    }





}




