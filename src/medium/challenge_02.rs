
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
    for (index, &ids) in ids.iter().take(MAX_VALIDATORS).enumerate() {
        validators[index] = (ids, Some(ValidatorStatus::Active));
    }
    validators
}

fn set_validator_status(validators: &mut [(ValidatorId, Option<ValidatorStatus>);MAX_VALIDATORS]) {
    todo!()
}

fn get_validator_status(validators: &[(ValidatorId, Option<ValidatorStatus>); MAX_VALIDATORS], id: ValidatorId)
    -> Option<(ValidatorId, Option<ValidatorStatus>)> {
    todo!()
}

mod tests {
    use crate::desafio2::{initialize_validators, ValidatorId, ValidatorStatus, MAX_VALIDATORS};

    #[test]
    fn initialize_validators_test() {
        let mut ids = [0;MAX_VALIDATORS];
        for i in 0..ids.len() {ids[i] = i as ValidatorId}
        let validators = initialize_validators(&ids);
        for i in 0..ids.len() {
            assert_eq!(validators[i].1.unwrap(), ValidatorStatus::Active)
        }

       let mut less_ids = [0;MAX_VALIDATORS-1];
        for i in 0..less_ids.len() {less_ids[i] = i as ValidatorId}
        assert_eq!(MAX_VALIDATORS-1, less_ids.len());

        let  mut more_ids = [0;MAX_VALIDATORS];
        for i in 0..ids.len() {ids[i] = i as ValidatorId}
        let validators = initialize_validators(&ids);
        assert_eq!(MAX_VALIDATORS, validators.len());
    }





}




