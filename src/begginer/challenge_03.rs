pub trait Attackable {
    fn attack(&self);
}

pub struct Warrior {}
pub struct Mage {}
pub struct Archer {}


impl Attackable for Warrior {
    fn attack(&self) {
        println!("Warrior attacks with their sword!");
    }
}

impl Attackable for Mage {
    fn attack(&self) {
        println!("Mage casts a fireball!");
    }
}

impl Attackable for Archer {
    fn attack(&self) {
        println!("Archer shoots a precise arrow!");
    }
}


mod tests {
    use super::*;
   
    #[test]
    fn attack_test() {
        let characters: Vec<Box<dyn Attackable>> = vec![
            Box::new(Warrior{}),
            Box::new(Mage{}),
            Box::new(Archer{})
        ];

        for character in &characters {
            character.attack();
        }

}



}
