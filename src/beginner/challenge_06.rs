trait Animal {
    fn speak(&self);
}

struct Dog {}

struct Cat {}

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof woof");
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!! I am a cat");
    }
}

struct Handler<T> {
    animal: T,
}

impl<T> Handler<T> 
where
    T: Animal,
{
    fn new(animal: T) -> Self {
        Handler { animal }
    }

    fn make_animal_speak(&self) {
        self.animal.speak();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animal_speak() {
        let dog = Dog {};
        let cat = Cat {};

        // Test direct trait usage
        dog.speak(); // Should print "Woof woof"
        cat.speak(); // Should print "Meow!! I am a cat"
    }

    #[test]
    fn test_generic_handler() {
        let dog = Dog {};
        let cat = Cat {};

        // Test generic handler
        let dog_handler = Handler::new(dog);
        dog_handler.make_animal_speak();

        let cat_handler = Handler::new(cat);
        cat_handler.make_animal_speak();
    }

    #[test]
    fn test_trait_bounds() {
        // Test that trait bounds work correctly
        let dog = Dog {};
        let handler = Handler::new(dog);
        
        // This should compile because Dog implements Animal
        handler.make_animal_speak();
    }

    #[test]
    fn test_multiple_handlers() {
        let dog1 = Dog {};
        let dog2 = Dog {};
        let cat = Cat {};

        let handlers = vec![
            Handler::new(dog1),
            Handler::new(dog2),
        ];

        for handler in handlers {
            handler.make_animal_speak();
        }

        let cat_handler = Handler::new(cat);
        cat_handler.make_animal_speak();
    }
}