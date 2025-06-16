## Challenge 4: Simple Custom Origin Pallet

**Difficulty Level:** Advanced
**Estimated Time:** 2 hours

### Objective Description

You will implement a pallet with custom origins that demonstrates how to create specialized permission systems beyond the standard `Root` and `Signed` origins. This challenge focuses on understanding how custom origins work in FRAME and how they can be used to implement fine-grained access control.

**Main Concepts Covered:**
1. **Custom Origins:** Creating specialized permission types
2. **Origin Filtering:** Controlling who can call specific functions
3. **Permission Systems:** Implementing role-based access control
4. **Origin Conversion:** Converting between different origin types
5. **Access Control:** Fine-grained permission management

### Detailed Structures to Implement:

#### **Custom Origin Definition:**
    ```rust
/// Custom origins for the pallet
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Origin {
    /// Administrative origin - highest level access
    Admin,
    /// Moderator origin - medium level access
    Moderator,
    /// Member origin - basic level access
    Member,
}

impl Origin {
    /// Check if origin has admin privileges
    pub fn is_admin(&self) -> bool {
        matches!(self, Origin::Admin)
    }
    
    /// Check if origin has at least moderator privileges
    pub fn is_moderator_or_above(&self) -> bool {
        matches!(self, Origin::Admin | Origin::Moderator)
    }
    
    /// Check if origin has at least member privileges
    pub fn is_member_or_above(&self) -> bool {
        matches!(self, Origin::Admin | Origin::Moderator | Origin::Member)
    }
}
```

#### **Pallet Configuration:**
    ```rust
pub trait Config {
    type RuntimeOrigin: From<Origin>;
    type AccountId: Clone + PartialEq + core::fmt::Debug;
}

pub struct Pallet<T: Config> {
    /// Maps accounts to their roles
    roles: std::collections::HashMap<T::AccountId, Origin>,
    /// Storage for managed data
    managed_data: std::collections::HashMap<String, String>,
    /// Event log
    events: Vec<Event<T::AccountId>>,
    _phantom: std::marker::PhantomData<T>,
}
```

#### **Events:**
    ```rust
    #[derive(Clone, Debug, PartialEq)]
pub enum Event<AccountId> {
    /// Role was assigned to an account
    RoleAssigned { account: AccountId, role: Origin },
    /// Role was revoked from an account
    RoleRevoked { account: AccountId },
    /// Data was created by an account
    DataCreated { key: String, creator: AccountId },
    /// Data was updated by an account
    DataUpdated { key: String, updater: AccountId },
    /// Data was deleted by an account
    DataDeleted { key: String, deleter: AccountId },
}
```

#### **Errors:**
    ```rust
    #[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// Origin does not have required permissions
    InsufficientPermissions,
    /// Account does not have any assigned role
    NoRoleAssigned,
    /// Data key already exists
    DataAlreadyExists,
    /// Data key not found
    DataNotFound,
    /// Invalid origin type
    InvalidOrigin,
}
```

### Origin Filtering Implementation:

#### **Origin Filters:**
    ```rust
impl<T: Config> Pallet<T> {
    /// Ensure origin is admin
    fn ensure_admin(origin: &T::RuntimeOrigin, account: &T::AccountId) -> Result<(), Error> {
        let role = Self::get_role(account).ok_or(Error::NoRoleAssigned)?;
        if role.is_admin() {
            Ok(())
        } else {
            Err(Error::InsufficientPermissions)
        }
    }
    
    /// Ensure origin is moderator or above
    fn ensure_moderator_or_above(origin: &T::RuntimeOrigin, account: &T::AccountId) -> Result<(), Error> {
        let role = Self::get_role(account).ok_or(Error::NoRoleAssigned)?;
        if role.is_moderator_or_above() {
            Ok(())
        } else {
            Err(Error::InsufficientPermissions)
        }
    }
    
    /// Ensure origin is member or above
    fn ensure_member_or_above(origin: &T::RuntimeOrigin, account: &T::AccountId) -> Result<(), Error> {
        let role = Self::get_role(account).ok_or(Error::NoRoleAssigned)?;
        if role.is_member_or_above() {
            Ok(())
        } else {
            Err(Error::InsufficientPermissions)
        }
    }
    
    /// Get role for an account
    fn get_role(account: &T::AccountId) -> Option<Origin> {
        // In a real implementation, this would query storage
        None // Placeholder
    }
}
```

#### **Dispatchable Functions with Custom Origins:**
    ```rust
impl<T: Config> Pallet<T> {
    /// Assign a role to an account (Admin only)
    pub fn assign_role(
        &mut self,
        origin: T::RuntimeOrigin,
        caller: T::AccountId,
        target: T::AccountId,
        role: Origin,
    ) -> Result<(), Error> {
        Self::ensure_admin(&origin, &caller)?;
        
        self.roles.insert(target.clone(), role.clone());
        self.events.push(Event::RoleAssigned { 
            account: target, 
            role 
        });
        
        Ok(())
    }
    
    /// Revoke role from an account (Admin only)
    pub fn revoke_role(
        &mut self,
        origin: T::RuntimeOrigin,
        caller: T::AccountId,
        target: T::AccountId,
    ) -> Result<(), Error> {
        Self::ensure_admin(&origin, &caller)?;
        
        self.roles.remove(&target);
        self.events.push(Event::RoleRevoked { 
            account: target 
        });
        
        Ok(())
    }
    
    /// Create data (Member or above)
    pub fn create_data(
        &mut self,
        origin: T::RuntimeOrigin,
        caller: T::AccountId,
        key: String,
        value: String,
    ) -> Result<(), Error> {
        Self::ensure_member_or_above(&origin, &caller)?;
        
        if self.managed_data.contains_key(&key) {
            return Err(Error::DataAlreadyExists);
        }
        
        self.managed_data.insert(key.clone(), value);
        self.events.push(Event::DataCreated { 
            key, 
            creator: caller 
        });
        
        Ok(())
    }
    
    /// Update data (Moderator or above)
    pub fn update_data(
        &mut self,
        origin: T::RuntimeOrigin,
        caller: T::AccountId,
        key: String,
        new_value: String,
    ) -> Result<(), Error> {
        Self::ensure_moderator_or_above(&origin, &caller)?;
        
        if !self.managed_data.contains_key(&key) {
            return Err(Error::DataNotFound);
        }
        
        self.managed_data.insert(key.clone(), new_value);
        self.events.push(Event::DataUpdated { 
            key, 
            updater: caller 
        });
        
        Ok(())
    }
    
    /// Delete data (Admin only)
    pub fn delete_data(
        &mut self,
        origin: T::RuntimeOrigin,
        caller: T::AccountId,
        key: String,
    ) -> Result<(), Error> {
        Self::ensure_admin(&origin, &caller)?;
        
        if !self.managed_data.contains_key(&key) {
            return Err(Error::DataNotFound);
        }
        
        self.managed_data.remove(&key);
        self.events.push(Event::DataDeleted { 
            key, 
            deleter: caller 
        });
        
        Ok(())
    }
}
```

### Helper Functions:

#### **Utility Methods:**
        ```rust
impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            roles: std::collections::HashMap::new(),
            managed_data: std::collections::HashMap::new(),
            events: Vec::new(),
            _phantom: std::marker::PhantomData,
        }
    }
    
    /// Get all accounts with a specific role
    pub fn get_accounts_with_role(&self, role: &Origin) -> Vec<T::AccountId> {
        self.roles
            .iter()
            .filter(|(_, r)| *r == role)
            .map(|(account, _)| account.clone())
            .collect()
    }
    
    /// Get data by key
    pub fn get_data(&self, key: &str) -> Option<String> {
        self.managed_data.get(key).cloned()
    }
    
    /// Get all data keys
    pub fn get_all_keys(&self) -> Vec<String> {
        self.managed_data.keys().cloned().collect()
    }
    
    /// Take events for testing
    pub fn take_events(&mut self) -> Vec<Event<T::AccountId>> {
        std::mem::take(&mut self.events)
    }
}
```

### Tests

Create comprehensive tests covering:

1. **Role Management:**
   - Test role assignment by admin
   - Test role revocation by admin
   - Test permission denial for non-admin role operations

2. **Permission Levels:**
   - Test member-level operations (create data)
   - Test moderator-level operations (update data)
   - Test admin-level operations (delete data)

3. **Access Control:**
   - Test permission escalation prevention
   - Test proper error handling for insufficient permissions
   - Test role-based function access

4. **Data Management:**
   - Test data creation, update, and deletion
   - Test error handling for duplicate/missing data
   - Test proper event emission

### Expected Output

A complete custom origin system that:
- Implements hierarchical permission levels
- Properly filters function access based on roles
- Demonstrates fine-grained access control
- Shows understanding of FRAME origin system
- Handles errors gracefully

### Theoretical Context

**Custom Origins in FRAME:**
- **Purpose:** Enable fine-grained access control beyond basic Root/Signed
- **Implementation:** Custom enum types that implement origin traits
- **Integration:** Works with FRAME's origin filtering system
- **Use Cases:** Role-based access, governance systems, specialized permissions

**Origin Filtering:**
- Functions can specify required origin types
- Runtime validates origins before dispatch
- Enables complex permission hierarchies
- Prevents unauthorized access to sensitive functions

This system is essential for building sophisticated governance and permission systems in Substrate-based blockchains.
