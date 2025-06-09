pub trait Movivel {
    fn mover(&self);
}

pub struct Car {}
pub struct Bike {}
pub struct Scooter {}

impl Movivel for Car {
    fn mover(&self) {
        println!("accelerating...");
    }
}

impl Movivel for Bike {
    fn mover(&self) {
        println!("cyclyng...");
    }
}

impl Movivel for Scooter {
    fn mover(&self) {
        println!("The scooter is sliding......");
    }
}


pub struct Driver<T: Movivel> {
    vehicle: T,
}

impl<T: Movivel> Driver<T> {
    fn to_drive(&self) {
        self.vehicle.mover();
    }
}


mod tests {
    use super::*;

    #[test]
    fn test_to_drive() {
        let car = Car {};
    let driver = Driver {
        vehicle: car
    }; 
    let bike = Bike {};
    let cyclist = Driver { vehicle: bike};
    let patinet = Scooter {};
    let scooterist = Driver { vehicle: patinet};

    driver.to_drive();
    cyclist.to_drive();
    scooterist.to_drive();

    }
}

