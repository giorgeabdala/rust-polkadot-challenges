## Challenge 5: Custom Origin with Permission System

**Difficulty Level:** Advanced
**Estimated Time:** 1.5 hours

### Objective Description

You will implement a custom origin system with a hierarchical permission structure. This challenge focuses on understanding how Substrate manages call origins and how to create custom authorization mechanisms for different types of operations.

**Main Concepts Covered:**
1. **Custom Origins:** Creating specialized origin types beyond Root/Signed/None
2. **Permission Hierarchy:** Implementing role-based access control
3. **Origin Filtering:** Controlling which origins can call specific functions
4. **Role Management:** Dynamic role assignment and validation
5. **Access Control:** Ensuring proper authorization for sensitive operations

### Detailed Structures to Implement:

#### **Custom Origin Definition:**
```rust
/// Custom origin types for the permission system
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CustomOrigin {
    /// Administrator with full permissions
    Admin,
    /// Regular member with limited permissions
    Member,
}

impl CustomOrigin {
    /// Check if origin has admin privileges
    pub fn is_admin(&self) -> bool {
        matches!(self, CustomOrigin::Admin)
    }
    
    /// Check if origin has at least member privileges
    pub fn is_member_or_above(&self) -> bool {
        matches!(self, CustomOrigin::Admin | CustomOrigin::Member)
    }
}
```

#### **Origin Wrapper:**
```rust
/// Wrapper for different origin types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Origin {
    /// Standard signed origin
    Signed(u32), // account_id
    /// Custom permission-based origin
    Custom(CustomOrigin),
    /// Root origin (system level)
    Root,
}

impl Origin {
    /// Extract account ID from signed origin
    pub fn as_signed(&self) -> Option<u32> {
        match self {
            Origin::Signed(account_id) => Some(*account_id),
            _ => None,
        }
    }
    
    /// Check if origin is root
    pub fn is_root(&self) -> bool {
        matches!(self, Origin::Root)
    }
    
    /// Get custom origin if present
    pub fn as_custom(&self) -> Option<&CustomOrigin> {
        match self {
            Origin::Custom(custom) => Some(custom),
            _ => None,
        }
    }
}
```

#### **Role Management System:**
```rust
use std::collections::HashMap;

/// Manages user roles and permissions
pub struct RoleManager {
    /// Maps account IDs to their roles
    roles: HashMap<u32, CustomOrigin>,
    /// System administrator account
    admin_account: u32,
}

impl RoleManager {
    pub fn new(admin_account: u32) -> Self {
        let mut roles = HashMap::new();
        roles.insert(admin_account, CustomOrigin::Admin);
        
        Self {
            roles,
            admin_account,
        }
    }
    
    /// Assign role to an account
    pub fn assign_role(&mut self, account_id: u32, role: CustomOrigin) -> Result<(), &'static str> {
        self.roles.insert(account_id, role);
        Ok(())
    }
    
    /// Get role for an account
    pub fn get_role(&self, account_id: u32) -> Option<&CustomOrigin> {
        self.roles.get(&account_id)
    }
    
    /// Remove role from an account
    pub fn remove_role(&mut self, account_id: u32) -> Result<(), &'static str> {
        if account_id == self.admin_account {
            return Err("Cannot remove admin role from system administrator");
        }
        
        self.roles.remove(&account_id);
        Ok(())
    }
    
    /// List all accounts with specific role
    pub fn accounts_with_role(&self, role: &CustomOrigin) -> Vec<u32> {
        self.roles
            .iter()
            .filter(|(_, r)| *r == role)
            .map(|(account_id, _)| *account_id)
            .collect()
    }
}
```

#### **Origin Filter System:**
```rust
/// Validates origins against permission requirements
pub struct OriginFilter {
    role_manager: RoleManager,
}

impl OriginFilter {
    pub fn new(role_manager: RoleManager) -> Self {
        Self { role_manager }
    }
    
    /// Convert signed origin to custom origin based on roles
    pub fn signed_to_custom(&self, account_id: u32) -> Option<CustomOrigin> {
        self.role_manager.get_role(account_id).cloned()
    }
    
    /// Ensure origin has admin privileges
    pub fn ensure_admin(&self, origin: &Origin) -> Result<(), &'static str> {
        match origin {
            Origin::Root => Ok(()),
            Origin::Custom(CustomOrigin::Admin) => Ok(()),
            Origin::Signed(account_id) => {
                match self.role_manager.get_role(*account_id) {
                    Some(CustomOrigin::Admin) => Ok(()),
                    _ => Err("Admin privileges required"),
                }
            },
            _ => Err("Admin privileges required"),
        }
    }
    
    /// Ensure origin has at least member privileges
    pub fn ensure_member(&self, origin: &Origin) -> Result<(), &'static str> {
        match origin {
            Origin::Root => Ok(()),
            Origin::Custom(custom) if custom.is_member_or_above() => Ok(()),
            Origin::Signed(account_id) => {
                match self.role_manager.get_role(*account_id) {
                    Some(role) if role.is_member_or_above() => Ok(()),
                    _ => Err("Member privileges required"),
                }
            },
            _ => Err("Member privileges required"),
        }
    }
    
    /// Get mutable reference to role manager for admin operations
    pub fn role_manager_mut(&mut self) -> &mut RoleManager {
        &mut self.role_manager
    }
    
    /// Get reference to role manager for queries
    pub fn role_manager(&self) -> &RoleManager {
        &self.role_manager
    }
}
```

#### **Permission-Based Pallet:**
```rust
/// Simple pallet with permission-based operations
pub struct PermissionPallet {
    /// Counter that can be modified by members
    counter: u32,
    /// Admin-only setting
    admin_setting: bool,
    /// Origin filter for permission checks
    origin_filter: OriginFilter,
}

impl PermissionPallet {
    pub fn new(admin_account: u32) -> Self {
        let role_manager = RoleManager::new(admin_account);
        let origin_filter = OriginFilter::new(role_manager);
        
        Self {
            counter: 0,
            admin_setting: false,
            origin_filter,
        }
    }
    
    /// Increment counter (requires member privileges)
    pub fn increment_counter(&mut self, origin: Origin) -> Result<u32, &'static str> {
        self.origin_filter.ensure_member(&origin)?;
        
        self.counter = self.counter.saturating_add(1);
        Ok(self.counter)
    }
    
    /// Reset counter (requires admin privileges)
    pub fn reset_counter(&mut self, origin: Origin) -> Result<(), &'static str> {
        self.origin_filter.ensure_admin(&origin)?;
        
        self.counter = 0;
        Ok(())
    }
    
    /// Toggle admin setting (requires admin privileges)
    pub fn toggle_admin_setting(&mut self, origin: Origin) -> Result<bool, &'static str> {
        self.origin_filter.ensure_admin(&origin)?;
        
        self.admin_setting = !self.admin_setting;
        Ok(self.admin_setting)
    }
    
    /// Assign role to account (requires admin privileges)
    pub fn assign_role(
        &mut self,
        origin: Origin,
        target_account: u32,
        role: CustomOrigin,
    ) -> Result<(), &'static str> {
        self.origin_filter.ensure_admin(&origin)?;
        
        self.origin_filter
            .role_manager_mut()
            .assign_role(target_account, role)
    }
    
    /// Remove role from account (requires admin privileges)
    pub fn remove_role(
        &mut self,
        origin: Origin,
        target_account: u32,
    ) -> Result<(), &'static str> {
        self.origin_filter.ensure_admin(&origin)?;
        
        self.origin_filter
            .role_manager_mut()
            .remove_role(target_account)
    }
    
    /// Get counter value (public read)
    pub fn get_counter(&self) -> u32 {
        self.counter
    }
    
    /// Get admin setting (public read)
    pub fn get_admin_setting(&self) -> bool {
        self.admin_setting
    }
    
    /// Get user role (public read)
    pub fn get_user_role(&self, account_id: u32) -> Option<&CustomOrigin> {
        self.origin_filter.role_manager().get_role(account_id)
    }
    
    /// List all admins (public read)
    pub fn list_admins(&self) -> Vec<u32> {
        self.origin_filter
            .role_manager()
            .accounts_with_role(&CustomOrigin::Admin)
    }
    
    /// List all members (public read)
    pub fn list_members(&self) -> Vec<u32> {
        self.origin_filter
            .role_manager()
            .accounts_with_role(&CustomOrigin::Member)
    }
}
```

#### **Origin Construction Helpers:**
```rust
/// Helper functions for creating origins
pub struct OriginBuilder;

impl OriginBuilder {
    /// Create a signed origin
    pub fn signed(account_id: u32) -> Origin {
        Origin::Signed(account_id)
    }
    
    /// Create a root origin
    pub fn root() -> Origin {
        Origin::Root
    }
    
    /// Create a custom admin origin
    pub fn admin() -> Origin {
        Origin::Custom(CustomOrigin::Admin)
    }
    
    /// Create a custom member origin
    pub fn member() -> Origin {
        Origin::Custom(CustomOrigin::Member)
    }
}
```

### Tests

Create comprehensive tests covering:

1. **Custom Origin Types:**
   - Test origin creation and comparison
   - Verify origin type checking methods
   - Test origin conversion functions

2. **Role Management:**
   - Test role assignment and removal
   - Verify role queries and listings
   - Test admin protection mechanisms

3. **Permission Filtering:**
   - Test admin-only operations
   - Test member-level operations
   - Verify permission denial for unauthorized origins

4. **Pallet Integration:**
   - Test all pallet operations with different origins
   - Verify proper error handling for unauthorized access
   - Test role management through pallet interface

5. **Edge Cases:**
   - Test operations with invalid origins
   - Test role changes and their effects
   - Test system administrator protection

### Expected Output

A complete custom origin system that:
- Defines hierarchical permission levels
- Implements role-based access control
- Provides origin filtering and validation
- Demonstrates proper authorization patterns
- Shows understanding of Substrate's origin system

### Theoretical Context

**Origins in Substrate:**
- **Purpose:** Identify and authorize the source of extrinsic calls
- **Types:** Root (system), Signed (users), None (unsigned), Custom (specialized)
- **Filtering:** Origins are checked against function requirements
- **Hierarchy:** Custom origins can implement complex permission structures
- **Security:** Proper origin checking prevents unauthorized operations

**Custom Origins:**
- Allow for specialized authorization beyond basic signed/root
- Enable role-based access control systems
- Support complex permission hierarchies
- Integrate with pallet-specific authorization logic
- Provide flexibility for governance and administrative functions

**Best Practices:**
- Always validate origins before executing sensitive operations
- Implement clear permission hierarchies
- Protect system-critical roles (like admin accounts)
- Provide query functions for transparency
- Handle authorization errors gracefully

This system demonstrates how to implement sophisticated authorization mechanisms using Substrate's origin system.
