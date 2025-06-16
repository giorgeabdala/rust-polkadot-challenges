

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
        todo!()
    }

    fn set_email(&mut self, email: String) {
        todo!()
    }

    fn set_phone(&mut self, phone: String) {
        todo!()
    }
    
}

impl ContactBook {

    fn new() -> Self {
        todo!()
    }

    fn add_contact(&mut self, contact: Contact) {
        todo!()
    }

    fn find_contact(&self, name: &str) -> Option<&Contact> {
        todo!()
    }

    fn get_email(&self, name: &str) -> Option<&String> {
        todo!()
    }
    
}

