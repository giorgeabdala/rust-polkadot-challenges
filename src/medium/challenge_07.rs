use std::collections::HashMap;
use std::hash::Hash;
use std::borrow::Borrow; // Might be useful

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct TaskId(u32);

impl TaskId {
    fn new(id: u32) -> Self {
        TaskId(id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}


#[derive(Debug, Clone, PartialEq)] // Eq removed as String is not trivially Eq without manual impl
pub struct Task {
    pub id: TaskId,
    pub description: String,
    pub status: TaskStatus,
}

impl Task {
    fn new(id: TaskId, description: String) -> Self {
        Task {
            id,
            description,
            status: TaskStatus::Pending,
        }
    }
}

pub trait KeyValueStorage<K: Eq + Hash, V> {
    fn get(&self, key: &K) -> Option<&V>;
    fn get_mut(&mut self, key: &K) -> Option<&mut V>;
    fn set(&mut self, key: K, value: V) -> Result<(), String>; // Error as String
    fn remove(&mut self, key: &K) -> Option<V>; // Key as reference for removal
    fn get_all_values(&self) -> Vec<&V>; // To list all tasks
    // fn get_all_entries(&self) -> Vec<(&K, &V)>; // Optional, if you need keys and values
}

pub struct InMemoryStorage<K: Eq + Hash + Clone, V> { // K needs to be Clone for some operations
    data: HashMap<K, V>,
}

impl<K: Eq + Hash + Clone, V> InMemoryStorage<K, V> {
    pub fn new() -> Self {
        InMemoryStorage {
            data: HashMap::new(),
        }
    }
}

impl<K: Eq + Hash + Clone, V> Default for InMemoryStorage<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Eq + Hash + Clone, V> KeyValueStorage<K, V> for InMemoryStorage<K,V> {
    fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.data.get_mut(key)
    }

    fn set(&mut self, key: K, value: V) -> Result<(), String> {
        self.data.insert(key, value);
        Ok(())
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key)
    }

    fn get_all_values(&self) -> Vec<&V> {
        self.data.values().collect()
    }
}


pub struct TaskManager<S: KeyValueStorage<TaskId, Task>> {
    storage: S,
    next_id: u32,
}

impl<S:KeyValueStorage<TaskId, Task>> TaskManager<S> {
    fn add_task(&mut self, description: String) -> TaskId {
        let task_id = TaskId::new(self.next_id);
        let task = Task::new(task_id, description);
        self.storage.set(task_id, task).expect("Setting task in InMemoryStorage should not fail");
        self.next_id += 1;
        task_id
    }

    fn get_task<'a> (&'a self, id:&TaskId) -> Option<&'a Task> {
        self.storage.get(id)
    }


    fn update_task_status(&mut self, id: &TaskId, new_status: TaskStatus) -> Result<(), String> {
        let task_opt = self.storage.get_mut(id);
        match task_opt {
            Some(task) => {
                task.status = new_status;
                Ok(())
            }
            None =>  Err("Task not found".to_string())
        }
    }

    fn list_tasks_by_filter_closure<'storage_lifetime, P>(&'storage_lifetime self, predicate: P)
        -> Vec<&'storage_lifetime Task>  where P: Fn(&&Task) -> bool, S: KeyValueStorage<TaskId, Task> + 'storage_lifetime {
       let tasks = self.storage.get_all_values();
        tasks.iter()
            .filter(|task_ref_ref| predicate(task_ref_ref))
            .map(|task_ref_ref| *task_ref_ref).collect()
    }


    fn remove_task(&mut self, id: &TaskId) -> Result<Task, String> {
       self.storage.remove(id).ok_or_else(|| "Task not found".to_string())
    }
}

#[cfg(test)]
 mod tests {
    use super::*;

    #[test]
     fn add_task_test() {
        let storage = InMemoryStorage::new();
        let mut task_manager = TaskManager{storage, next_id: 0 };
        let description = "First Task".to_string();
        let task_id_one = task_manager.add_task(description.clone());
        assert_eq!(task_id_one, TaskId::new(0));
        assert_eq!(task_manager.next_id, 1, "next_id should be incremented");
     }

    #[test]
    fn get_task_test() {
        let storage = InMemoryStorage::new();
        let mut task_manager = TaskManager{storage, next_id: 0 };
        let description = "First Task".to_string();
        let task_id_one = task_manager.add_task(description.clone());
        assert_eq!(task_id_one, TaskId::new(0));

        //checking task on
        let task_one_opt = task_manager.get_task(&task_id_one);
        assert!(task_one_opt.is_some(), "Task one shoud be Found");
        let task_one = task_one_opt.unwrap();
        assert_eq!(task_one.id, TaskId::new(0));
        assert_eq!(task_one.description, description);
        assert_eq!(task_one.status, TaskStatus::Pending);

        //add task two
        let description_two = "Second Task".to_string();
        let task_id_two = task_manager.add_task(description_two.clone());
        assert_eq!(task_id_two, TaskId(1));

        //cheking task two
        let task_two_opt = task_manager.get_task(&task_id_two);
        assert!(task_two_opt.is_some(), "Task tow shoul be Found");
        let task_two = task_two_opt.unwrap();
        assert_eq!(task_two.id, TaskId::new(1));
        assert_eq!(task_two.description, description_two);
        assert_eq!(task_two.status, TaskStatus::Pending);

        //still get task two
        let task_one_again_opt = task_manager.get_task(&task_id_one);
        assert!(task_one_again_opt.is_some(), "Task one should be Found again");
        assert_eq!(task_one_again_opt.unwrap().id, task_id_one);

    }

     #[test]
     fn update_status_test() {
        let storage = InMemoryStorage::new();
         let mut task_manager = TaskManager{storage, next_id: 0};

         let description = "First Task".to_string();
         let task_id = task_manager.add_task(description);
         let task = task_manager.get_task(&task_id).expect("Task should exist");
         assert_eq!(task.status, TaskStatus::Pending, "Initial status should be Pending");

         let update_result = task_manager.update_task_status(&task_id, TaskStatus::InProgress);
         assert!(update_result.is_ok(), "Updating task status should succeed");

         let task = task_manager.get_task(&task_id).expect("Task not Found");
         assert_eq!(task.status, TaskStatus::InProgress, "Status should be updated to InProgress");
     }

    #[test]
    fn update_status_nonexistent_task_fail() {
        let storage = InMemoryStorage::new();
        let mut task_manager = TaskManager{storage, next_id: 0};
        let description = "First Task".to_string();
        let task_id = task_manager.add_task(description);
        let task = task_manager.get_task(&task_id).expect("Task should exist");
        let update_error_result = task_manager.update_task_status(&TaskId(10), TaskStatus::InProgress);
        assert!(update_error_result.is_err(), "Updating task status should fail");
    }
     #[test]
     fn test_list_tasks_with_closure_filter() {
         let storage = InMemoryStorage::new();
         let mut task_manager = TaskManager{storage, next_id:0};

         let description_one = "Task 1".to_string();
         let description_two = "Task 2".to_string();
         let description_three = "Task 3".to_string();

         let task_id_one = task_manager.add_task(description_one.clone());
         task_manager.add_task(description_two.clone());
         task_manager.add_task(description_three.clone());

         task_manager.update_task_status(&TaskId::new(1),
                                         TaskStatus::InProgress).expect("Update status fail");
         task_manager.update_task_status(&TaskId::new(2),
                                         TaskStatus::Completed).expect("Update status fail");

         let pending_tasks = task_manager.list_tasks_by_filter_closure(
             |task: &&Task| task.status == TaskStatus::Pending
         );

         assert_eq!(pending_tasks.len(), 1);
         assert_eq!(pending_tasks[0].id, task_id_one);

     }



     #[test]
     fn remove_task_test() {
         let storage = InMemoryStorage::new();
         let mut task_manager = TaskManager{storage, next_id: 0};
         let description = "First Task".to_string();
         let task_id = task_manager.add_task(description);
         let task = task_manager.get_task(&task_id).expect("Task not found");
         assert_eq!(task.id, task_id);

         let removed = task_manager.remove_task(&task_id).expect("Failed to remove task");
        assert_eq!(removed.id, task_id);
         let task_opt = task_manager.get_task(&task_id);
         assert!(task_opt.is_none());
     }

    #[test]
    fn remove_nonexistent_task_test() {
        let storage = InMemoryStorage::new();
        let mut task_manager = TaskManager { storage, next_id: 0 };

        let non_existent_id = TaskId::new(999);
        let result = task_manager.remove_task(&non_existent_id);
        assert!(result.is_err(), "Expected an error when removing a non-existent task");

        let description = "Test Task".to_string();
        let task_id = task_manager.add_task(description);
        let _removed_task = task_manager.remove_task(&task_id).expect("Failed to remove task");

        let result_again = task_manager.remove_task(&task_id);
        assert!(result_again.is_err(), "Expected an error when removing the task a second time");
    }

     #[test]
     fn test_lifetimes_on_get_task() {
        let storage = InMemoryStorage::new();
         let mut task_manager = TaskManager{storage, next_id: 0};
         let description = "Task for lifetime test".to_string();
         let task_id1 = task_manager.add_task(description.clone());
         task_manager.add_task("Another task".to_string());

         let task_ref_option = task_manager.get_task(&task_id1);
         assert!(task_ref_option.is_some(), "Task should be found");
         let task_ref = task_ref_option.unwrap();

         assert_eq!(task_ref.description, description);
         println!("Accessed task description: {}", task_ref.description);

         let all_tasks_refs = task_manager.list_tasks_by_filter_closure(|_| true);
         assert_eq!(all_tasks_refs.len(), 2, "Should list two tasks");

         for task in all_tasks_refs {
             println!("Listed task (lifetime ok): {}", task.description);
         }

         // The fact that this test compiles and runs demonstrates that the references
         // returned by `get_task` and `list_tasks_by_filter_closure` can be
         // used safely while `task_manager` is in scope.
         // If we tried to make these references outlive `task_manager`,
         // the Rust compiler would generate an error, ensuring memory safety.


     }
 }