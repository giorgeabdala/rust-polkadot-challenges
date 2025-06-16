# Challenge 4: Option<T> - Handling Absence

**Estimated Time:** 15 minutes  
**Difficulty:** Beginner  
**Topics:** Option<T>, Null Safety, Pattern Matching

## Learning Objectives

By completing this challenge, you will understand:
- How `Option<T>` prevents null pointer errors
- Pattern matching with `Some` and `None`
- Basic Option methods (`unwrap_or`, `map`)
- When to use `Option<T>` in your APIs

## Background

Rust doesn't have null values. Instead, it uses `Option<T>` to represent values that might or might not exist:
- `Some(T)` - contains a value of type T
- `None` - represents the absence of a value

This eliminates null pointer exceptions at compile time.

## Challenge

Create a simple contact book that demonstrates working with `Option<T>`.

### Requirements

1. **Create a `Contact` struct** with:
   - `name: String`
   - `email: Option<String>`
   - `phone: Option<String>`

2. **Create a `ContactBook` struct** with:
   - `contacts: Vec<Contact>`

3. **Implement methods:**
   - `Contact::new(name: String) -> Self`
   - `Contact::set_email(&mut self, email: String)`
   - `Contact::set_phone(&mut self, phone: String)`
   - `ContactBook::new() -> Self`
   - `ContactBook::add_contact(&mut self, contact: Contact)`
   - `ContactBook::find_contact(&self, name: &str) -> Option<&Contact>`
   - `ContactBook::get_email(&self, name: &str) -> Option<&String>`

### Expected Behavior

```rust
let mut book = ContactBook::new();

let mut contact = Contact::new("Alice".to_string());
contact.set_email("alice@example.com".to_string());
// phone remains None

book.add_contact(contact);

// Safe lookup - returns Option<&Contact>
match book.find_contact("Alice") {
    Some(contact) => println!("Found: {}", contact.name),
    None => println!("Contact not found"),
}

// Using unwrap_or for defaults
let email = book.get_email("Alice").unwrap_or(&"No email".to_string());
```

## Testing

Write tests that demonstrate:
- Creating contacts with and without optional fields
- Finding contacts that exist and don't exist
- Handling `None` values safely
- Using `unwrap_or` for default values

Focus on understanding when you get `Some` vs `None` and how to handle both cases.

## Tips

- Use `Option<T>` when a value might legitimately be absent
- Use `unwrap_or()` to provide default values
- Use pattern matching to handle both `Some` and `None` cases
- Avoid `unwrap()` unless you're certain the value exists

## Key Learning Points

- **Null Safety**: No null pointer exceptions possible
- **Explicit Handling**: Must handle both `Some` and `None` cases
- **Safe Defaults**: Using `unwrap_or` for fallback values 