#![allow(dead_code)]

#[derive(Clone, PartialEq, Debug)]
struct Contact {
    name: String,
    email: Option<String>,
    phone: Option<String>
}

struct ContactBook {
    contacts: Vec<Contact>
}


impl Contact {

    fn new(name: String) -> Self {
        Contact{name, email:None, phone: None}
    }

    fn set_email(&mut self, email: String) {
        self.email = Some(email);
    }

    fn set_phone(&mut self, phone: String) {
        self.phone = Some(phone);
    }

}

impl ContactBook {

    fn new() -> Self {
        ContactBook{contacts: Vec::new()}
    }

    fn add_contact(&mut self, contact: Contact) {
        self.contacts.push(contact);
    }

    fn find_contact(&self, name: &str) -> Option<&Contact> {
        self.contacts.iter().find(|contact| contact.name == name)
    }

    fn get_email(&self, name: &str) -> Option<&String> {
        self.contacts
            .iter()
            .find(|contact| contact.name == name)
            .and_then(|contact| contact.email.as_ref())
    }
}

#[cfg(test)]

mod tests{
    use crate::beginner::challenge_04::{Contact, ContactBook};

    #[test]
    fn new_contact_test() {
        let name = "Alice".to_string();
        let contact = Contact::new(name.clone());
        assert_eq!(contact.name, name);
        assert_eq!(contact.email, None);
        assert_eq!(contact.phone, None);
    }

    #[test]
    fn contact_set_email_test() {
        let name = "Alice".to_string();
        let mut contact = Contact::new(name.clone());
        let email = "giorgeabdala@polkadotdeveloper.com".to_string();
        contact.set_email(email.clone());
        assert_eq!(contact.name, name);
        assert_eq!(contact.email, Some(email));
        assert_eq!(contact.phone, None);
    }

    #[test]
    fn contact_set_phone_test() {
        let name = "Alice".to_string();
        let mut contact = Contact::new(name.clone());
        let phone = "5541999999999".to_string();
        contact.set_phone(phone.clone());
        assert_eq!(contact.name, name);
        assert_eq!(contact.email, None);
        assert_eq!(contact.phone, Some(phone));
    }

    #[test]
    fn add_contact_book_test() {
        let name = "Alice".to_string();
        let contact = Contact::new(name.clone());
        let mut book = ContactBook::new();
        book.add_contact(contact.clone());
        assert_eq!(book.contacts.len(), 1);
        assert_eq!(book.contacts.first(), Some(&contact));
    }

    #[test]
    fn find_contact_test() {
        let name = "Alice".to_string();
        let mut contact = Contact::new(name.clone());
        contact.set_email("giorgeabdala@polkadotdeveloper.com".to_string());
        let contact2 = Contact::new("Contact 2".to_string());
        let mut book = ContactBook::new();
        book.add_contact(contact.clone());
        book.add_contact(contact2);

        let contact_found = book.find_contact(&name).expect("Contact not found");
        assert_eq!(contact, contact_found.clone());
    }

    #[test]
    fn get_email_test() {
        let name = "Alice".to_string();
        let email = "giorgeabdala@polkadotdeveloper.com".to_string();
        let mut contact = Contact::new(name.clone());
        contact.set_email(email.clone());
        let contact2 = Contact::new("Contact 2".to_string());
        let mut book = ContactBook::new();
        book.add_contact(contact.clone());
        book.add_contact(contact2);

        let fallback = "email not found".to_string();
        let email_found = book.get_email(&name).unwrap_or(&fallback);
        assert_eq!(email_found.clone(), email);
    }

    #[test]
    fn get_email_for_contact_without_email_test() {
        let mut book = ContactBook::new();
        let contact_no_email_name = "Bob".to_string();
        let contact_no_email = Contact::new(contact_no_email_name.clone());
        book.add_contact(contact_no_email);

        let fallback_email = "no_email_on_file".to_string();
        let found_email = book.get_email(&contact_no_email_name).unwrap_or(&fallback_email);
        assert_eq!(*found_email, fallback_email); // ou found_email.clone()
    }

    #[test]
    fn find_non_existent_contact_test() {
        let book = ContactBook::new(); // Empty book
        assert_eq!(book.find_contact("NonExistent"), None);

        let mut book_with_contact = ContactBook::new();
        book_with_contact.add_contact(Contact::new("Alice".to_string()));
        assert_eq!(book_with_contact.find_contact("NonExistent"), None);
    }

}
