#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CustomOrigin {
    Admin,
    Member,
}

impl CustomOrigin {
    pub fn is_admin(&self) -> bool {
        matches!(self, CustomOrigin::Admin)
    }
    pub fn is_member_or_above(&self) -> bool {
        matches!(self, CustomOrigin::Admin | CustomOrigin::Member)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Origin {
    Signed(u32), // account_id
    Custom(CustomOrigin),
    Root,
}

impl Origin {
     pub fn as_signed(&self) -> Option<u32> {
        match self {
            Origin::Signed(account_id) => Some(*account_id),
            _ => None,
        }
    }

    /// Verifica se a origem é root
    pub fn is_root(&self) -> bool {
        matches!(self, Origin::Root)
    }

    /// Obtém a origem customizada, se presente
    pub fn as_custom(&self) -> Option<&CustomOrigin> {
        match self {
            Origin::Custom(custom) => Some(custom),
            _ => None,
        }
    }
}

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


pub struct PermissionPallet {
    counter: u32,
    admin_setting: bool,
    origin_filter: OriginFilter
}

impl PermissionPallet {
    pub fn new(admin_account: u32) -> Self {
        let role_manager = RoleManager::new(admin_account);
        let origin_filter = OriginFilter::new(role_manager);

        Self {
            counter: 0,
            admin_setting: false,
            origin_filter
        }
    }

    pub fn increment_counter(&mut self, origin: Origin) -> Result<u32, &'static str> {
        self.origin_filter.ensure_member(&origin)?;
        self.counter = self.counter.saturating_add(1);
        Ok(self.counter)
    }

    pub fn reset_counter(&mut self, origin: Origin) -> Result<(), &'static str> {
        self.origin_filter.ensure_admin(&origin)?;
        self.counter = 0;
        Ok(())
    }

    pub fn toggle_admin_setting(&mut self, origin: Origin) -> Result<bool, &'static str> {
        self.origin_filter.ensure_admin(&origin)?;
        self.admin_setting = !self.admin_setting;
        Ok(self.admin_setting)
    }

    pub fn assign_role(&mut self, origin: Origin, target_account: u32, role: CustomOrigin)
                       -> Result<(), &'static str> {
        self.origin_filter.ensure_admin(&origin)?;

        self.origin_filter.role_manager_mut().assign_role(target_account, role)
    }

    pub fn remove_role(&mut self, origin: Origin, target_account: u32) -> Result<(), &'static str> {
        self.origin_filter.ensure_admin(&origin)?;
        self.origin_filter.role_manager_mut().remove_role(target_account)
    }

    pub fn get_counter(&self) -> u32 {
        self.counter
    }

    pub fn get_admin_setting(&self) -> bool {
        self.admin_setting
    }
    
    pub fn get_user_role(&self, account_id: u32) -> Option<&CustomOrigin> {
        self.origin_filter.role_manager().get_role(account_id)
    }
    pub fn list_admins(&self) -> Vec<u32> {
        self.origin_filter
            .role_manager()
            .accounts_with_role(&CustomOrigin::Admin)
    }
    pub fn list_members(&self) -> Vec<u32> {
        self.origin_filter
            .role_manager()
            .accounts_with_role(&CustomOrigin::Member)
    }
    
    
}


pub struct OriginBuilder;
impl OriginBuilder {
    pub fn signed(accound_id: u32) -> Origin {
        Origin::Signed(accound_id)
    }
    pub fn root() -> Origin {
        Origin::Root
    }
    pub fn admin() -> Origin {
        Origin::Custom(CustomOrigin::Admin)
    }
    pub fn member() -> Origin {
        Origin::Custom(CustomOrigin::Member)
    }
}

mod tests {
    use crate::advanced::challenge_05::PermissionPallet;

    #[test]
    fn pallet_increment_counter() {
        let pallet = PermissionPallet::new(0);
        //let result = pallet.increment_counter()
    }
    
}



