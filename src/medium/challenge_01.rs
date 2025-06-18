use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(Clone, PartialEq)]
struct User {
    id: u32,
    username: String,
    email: String,
    roles: Vec<String>
}

impl User {
    fn new(id: u32, username: String) -> Self {
        User{id, username, email: String::new(), roles: vec![]}
    }

}

struct UserManager {
    users: HashMap<u32, User>,
    username_index: BTreeMap<String, u32>,
    active_sessions: HashSet<u32>
}


impl UserManager {

    fn new() -> Self {
        UserManager {
            users: HashMap::new(),
            username_index: BTreeMap::new(),
            active_sessions: HashSet::new()
        }
    }

    fn add_user(&mut self, user: User) -> Result<(), String> {
        if self.users.contains_key(&user.id) {
            return Err(format!("user with ID {} already exists", user.id));
        }

        if self.username_index.contains_key(&user.username) {
            return Err(format!("User name '{}' is already in use", user.username));
        }

        self.users.insert(user.id, user.clone());
        self.username_index.insert(user.username, user.id);
        Ok(())
    }

    fn get_user(&self, id: u32) -> Option<&User> {
        self.users.get(&id)
    }

    fn find_by_username(&self, username: &str) -> Option<&User> {
        todo!()
    }

    fn get_users_by_role(&self, role: &str) -> Vec<&User> {
        todo!()
    }

    fn start_session(&mut self, user_id: u32) -> bool {
        todo!()
    }

    fn end_session(&mut self, user_id: u32) -> bool {
        todo!()
    }

    fn get_active_users(&self) -> Vec<&User> {
        todo!()
    }

    fn get_sorted_usernames(&self) -> Vec<&String> {
        todo!()
    }

}

#[cfg(test)]
mod tests {
    use crate::medium::challenge_01::{User, UserManager};

    #[test]
    fn add_and_get_user() {
        let user = User::new(1, "first user".to_string());
        let mut manager = UserManager::new();
        let result = manager.add_user(user.clone());
        assert_eq!(result, Ok(()));

        let user_found_opt = manager.get_user(1);
        assert!(user_found_opt.is_some());
        let user_found = user_found_opt.unwrap();
        assert_eq!(user_found.id, user.id);
        assert_eq!(user_found.username, user.username);
        assert_eq!(user_found.email, user.email);
        assert_eq!(user_found.roles, user.roles);
    }

    #[test]
    fn test_add_duplicate_id() {
        let user = User::new(1, "first user".to_string());
        let user2  = User::new(1, "second user".to_string());
        let mut manager = UserManager::new();
        let _ = manager.add_user(user.clone());
        let result = manager.add_user(user2.clone());
        assert!(result.is_err() );
    }

    #[test]
    fn test_add_duplicate_username() {
        let user = User::new(1, "first user".to_string());
        let user2  = User::new(2, "first user".to_string());
        let mut manager = UserManager::new();
        let _ = manager.add_user(user.clone());
        let result = manager.add_user(user2.clone());
        assert!(result.is_err() );
    }

    #[test]
    fn get_user_nonexistent_test() {
        let user = User::new(1, "first user".to_string());
        let mut manager = UserManager::new();
        let result = manager.add_user(user.clone());
        assert_eq!(result, Ok(()));

        let user_found_opt = manager.get_user(2);
        assert!(user_found_opt.is_none())
    }

    #[test]
    fn find_by_username_test() {
        let user = User::new(1, "first user".to_string());
        let mut manager = UserManager::new();
        let _ = manager.add_user(user.clone());
        let user_opt = manager.find_by_username(&user.clone().username);
        assert!(user_opt.is_some())
    }

    #[test]
    fn find_by_username_nonexistent_test() {
        let user = User::new(1, "first user".to_string());
        let mut manager = UserManager::new();
        let _ = manager.add_user(user.clone());
        let user_opt = manager.find_by_username("fail");
        assert!(user_opt.is_none());
    }

    #[test]
    fn get_users_by_role_test() {
        let mut user = User::new(1, "first user".to_string());
        let mut user_two = User::new(2, "second user".to_string());
        let role = "role 1".to_string();
        user.roles.push(role.clone());
        user_two.roles.push(role.clone());
        let mut manager = UserManager::new();
        let _ = manager.add_user(user.clone());
        let _ = manager.add_user(user_two);
        let _ = manager.add_user(User::new(2, "second user".to_string()));

        let mut users = manager.get_users_by_role(&role.clone());
        assert_eq!(users.len(), 2);
        let user_found = users.pop().unwrap();
        assert_eq!(user.id, user_found.id)
    }

    #[test]
    fn get_user_nonexistent_role_test() {
        let mut user = User::new(1, "first user".to_string());
        let role = "role 1".to_string();
        user.roles.push(role.clone());
        let mut manager = UserManager::new();
        let _ = manager.add_user(user.clone());
        let mut users = manager.get_users_by_role("role non existent");
        assert_eq!(users.len(), 0);
    }

    #[test]
    fn start_session_test() {
        let user = User::new(1, "user".to_string());
        let mut manager = UserManager::new();
        let _ = manager.add_user(user.clone());
        let started = manager.start_session(user.id);
        assert!(started);
        assert!(manager.active_sessions.contains(&user.id));
    }

    #[test]
    fn start_session_user_nonexistent_test() {
        let user = User::new(1, "user".to_string());
        let mut manager = UserManager::new();
        let _ = manager.add_user(user.clone());
        let started = manager.start_session(2);
        assert!(!started);
        assert!(!manager.active_sessions.contains(&user.id));
    }

    #[test]
    fn start_session_returns_false_if_already_active() {
        let user = User::new(1, "user".to_string());
        let mut manager = UserManager::new();
        let _ = manager.add_user(user.clone());
        let started = manager.start_session(1);
        assert!(started);
        let started = manager.start_session(1);
        assert!(!manager.active_sessions.contains(&user.id));
    }

    #[test]
    fn end_session_test() {
        let user = User::new(1, "user".to_string());
        let mut manager = UserManager::new();
        let _ = manager.add_user(user.clone());
        let started = manager.end_session(user.id);
        assert!(started);
    }

    #[test]
    fn en_session_return_false_for_nonexistent_user() {
        let user = User::new(1, "user".to_string());
        let mut manager = UserManager::new();
        let _ = manager.add_user(user.clone());
        let started = manager.end_session(2);
        assert!(!started);
    }

    #[test]
    fn get_active_users() {
        let user = User::new(1, "user".to_string());
        let user_two = User::new(2, "user two".to_string());
        let user_three = User::new(3, "user three".to_string());
        let mut manager = UserManager::new();
        let _ = manager.add_user(user.clone());
        let _ = manager.add_user(user_two.clone());
        let _ = manager.add_user(user_three.clone());
        let _ = manager.start_session(1);
        let _ = manager.start_session(2);

        let users = manager.get_active_users();
        assert_eq!(users.len(), 2);
        assert!(users.contains(&&user));
        assert!(users.contains(&&user_two));
    }

    #[test]
    fn get_sorted_usernames()  {
        let user = User::new(1, "user".to_string());
        let user_two = User::new(2, "user two".to_string());
        let user_three = User::new(3, "user three".to_string());
        let mut manager = UserManager::new();
        let _ = manager.add_user(user.clone());
        let _ = manager.add_user(user_two.clone());
        let _ = manager.add_user(user_three.clone());

        let mut usernames = manager.get_sorted_usernames();
        assert_eq!(usernames.len(), 3);
        assert_eq!(*usernames.pop().unwrap(), user.username);
        assert_eq!(*usernames.pop().unwrap(), user_two.username);
        assert_eq!(*usernames.pop().unwrap(), user_three.username);
    }













}