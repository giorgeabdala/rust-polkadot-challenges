#[derive(Debug, Clone, PartialEq)]
struct User {
    name: String,
    age: u32,
}

#[derive(Debug, Clone, PartialEq)]
enum Event {
    UserCreated { name: String, age: u32 },
    UserUpdated { name: String, new_age: u32 },
}

// Declarative macro: code generation at compile time
macro_rules! create_getter {
    ($field:ident, $field_type:ty) => { // Pattern matching on tokens
        fn $field(&self) -> &$field_type {
            &self.$field // Token substitution
        }
    };
}

// Macro with repetition: handles variable number of arguments
#[cfg(test)]
macro_rules! event {
    ($variant:ident { $($field:ident: $value:expr),* }) => { // * = zero or more
        Event::$variant {
            $($field: $value,)* // Expand each field:value pair
        }
    };
}

impl User {
    fn new(name: String, age: u32) -> Self {
        User { name, age }
    }

    // Using the getter macro to generate methods
    create_getter!(name, String);
    create_getter!(age, u32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getter_macro() {
        // Create User with test data
        let user = User::new("Alice".to_string(), 25);

        // Use generated getter methods
        let name = user.name();
        let age = user.age();

        // Verify they return correct references
        assert_eq!(name, "Alice");
        assert_eq!(*age, 25);
        
        // Verify these are actually references
        assert_eq!(name, &user.name);
        assert_eq!(age, &user.age);
    }

    #[test]
    fn test_event_macro() {
        // Create events using event! macro
        let created_event = event!(UserCreated { 
            name: "Alice".to_string(), 
            age: 25 
        });
        
        let updated_event = event!(UserUpdated { 
            name: "Bob".to_string(), 
            new_age: 30 
        });

        // Verify they match expected Event enum variants
        let expected_created = Event::UserCreated {
            name: "Alice".to_string(),
            age: 25,
        };
        
        let expected_updated = Event::UserUpdated {
            name: "Bob".to_string(),
            new_age: 30,
        };

        assert_eq!(created_event, expected_created);
        assert_eq!(updated_event, expected_updated);
    }

    #[test]
    fn test_macro_generated_code_functionality() {
        let user = User::new("Charlie".to_string(), 35);
        
        // Test that the getters work as expected
        assert_eq!(user.name(), "Charlie");
        assert_eq!(*user.age(), 35);
        
        // Test that we can use the getters in expressions
        let name_len = user.name().len();
        assert_eq!(name_len, 7);
        
        let is_adult = *user.age() >= 18;
        assert!(is_adult);
    }

    #[test]
    fn test_event_macro_with_different_values() {
        // Test with different data types and values
        let event1 = event!(UserCreated { 
            name: "Test User".to_string(), 
            age: 0 
        });
        
        let event2 = event!(UserUpdated { 
            name: "Updated User".to_string(), 
            new_age: 100 
        });

        match event1 {
            Event::UserCreated { name, age } => {
                assert_eq!(name, "Test User");
                assert_eq!(age, 0);
            }
            _ => panic!("Wrong event type"),
        }

        match event2 {
            Event::UserUpdated { name, new_age } => {
                assert_eq!(name, "Updated User");
                assert_eq!(new_age, 100);
            }
            _ => panic!("Wrong event type"),
        }
    }

    #[test]
    fn test_macro_patterns() {
        // Test that macros can handle expressions
        let base_age = 20;
        let calculated_age = base_age + 5;
        
        let event = event!(UserCreated { 
            name: format!("User_{}", 42), 
            age: calculated_age 
        });

        match event {
            Event::UserCreated { name, age } => {
                assert_eq!(name, "User_42");
                assert_eq!(age, 25);
            }
            _ => panic!("Wrong event type"),
        }
    }
}