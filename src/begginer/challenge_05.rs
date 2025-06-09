
trait Animal {
    fn speak(&self);
}

struct Dog;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof woof");
    }
}

struct Cat;

impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!! I am a cat");
    }
}

struct Handler<T: Animal> {
    animal: T,
}

impl<T: Animal> Handler<T> {
    fn make_animal_speak(&self) {
        self.animal.speak();
    }
}

mod tests {
    use super::*;

    #[test]
    fn test() {
        let my_dog = Dog {};
        let dog_handler = Handler {
            animal: my_dog,
        };
        dog_handler.make_animal_speak();
        let cat = Cat {};
        let cat_handler = Handler { animal: cat };
        cat_handler.make_animal_speak();
    }
}