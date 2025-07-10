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

    /// Check if the origin is root
    pub fn is_root(&self) -> bool {
        matches!(self, Origin::Root)
    }

    /// Get the custom origin, if present
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
    pub fn signed(account_id: u32) -> Origin {
        Origin::Signed(account_id)
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





#[cfg(test)]
mod tests {
    use super::{CustomOrigin, OriginBuilder, PermissionPallet, RoleManager};
    const ADMIN_ACCOUNT: u32 = 1;
    const MEMBER_ACCOUNT: u32 = 2;
    const NORMAL_ACCOUNT: u32 = 3;

    #[test]
    fn origin_helpers_work_correctly() {
        assert_eq!(OriginBuilder::signed(10).as_signed(), Some(10));
        assert!(OriginBuilder::root().is_root());
        assert_eq!(OriginBuilder::admin().as_custom(), Some(&CustomOrigin::Admin));
        assert_eq!(OriginBuilder::member().as_custom(), Some(&CustomOrigin::Member));
        assert!(CustomOrigin::Admin.is_admin());
        assert!(!CustomOrigin::Member.is_admin());
        assert!(CustomOrigin::Admin.is_member_or_above());
        assert!(CustomOrigin::Member.is_member_or_above());
    }

    #[test]
    fn role_manager_assigns_and_removes_roles() {
        let mut role_manager = RoleManager::new(ADMIN_ACCOUNT);
        assert_eq!(role_manager.get_role(ADMIN_ACCOUNT), Some(&CustomOrigin::Admin));

        role_manager.assign_role(MEMBER_ACCOUNT, CustomOrigin::Member).unwrap();
        assert_eq!(role_manager.get_role(MEMBER_ACCOUNT), Some(&CustomOrigin::Member));

        role_manager.remove_role(MEMBER_ACCOUNT).unwrap();
        assert_eq!(role_manager.get_role(MEMBER_ACCOUNT), None);
    }

    #[test]
    fn role_manager_protects_system_admin() {
        let mut role_manager = RoleManager::new(ADMIN_ACCOUNT);
        let result = role_manager.remove_role(ADMIN_ACCOUNT);
        assert_eq!(result, Err("Cannot remove admin role from system administrator"));
        assert_eq!(role_manager.get_role(ADMIN_ACCOUNT), Some(&CustomOrigin::Admin));
    }

    #[test]
    fn increment_counter_permission_logic_is_correct() {
        let mut pallet = PermissionPallet::new(ADMIN_ACCOUNT);
        pallet.assign_role(OriginBuilder::root(), MEMBER_ACCOUNT, CustomOrigin::Member).unwrap();
        
        assert!(pallet.increment_counter(OriginBuilder::root()).is_ok());
        assert!(pallet.increment_counter(OriginBuilder::signed(ADMIN_ACCOUNT)).is_ok());

        assert!(pallet.increment_counter(OriginBuilder::signed(MEMBER_ACCOUNT)).is_ok());
        assert_eq!(pallet.get_counter(), 3);
        
        let result = pallet.increment_counter(OriginBuilder::signed(NORMAL_ACCOUNT));
        assert_eq!(result, Err("Member privileges required"));
        assert_eq!(pallet.get_counter(), 3); // O contador não deve mudar
    }

    #[test]
    fn reset_counter_requires_admin_privileges() {
        let mut pallet = PermissionPallet::new(ADMIN_ACCOUNT);
        pallet.increment_counter(OriginBuilder::root()).unwrap();
        assert_eq!(pallet.get_counter(), 1);
        
        let result = pallet.reset_counter(OriginBuilder::signed(MEMBER_ACCOUNT));
        assert_eq!(result, Err("Admin privileges required"));
        assert_eq!(pallet.get_counter(), 1);
        
        assert!(pallet.reset_counter(OriginBuilder::signed(ADMIN_ACCOUNT)).is_ok());
        assert_eq!(pallet.get_counter(), 0);
    }

    #[test]
    fn toggle_admin_setting_requires_admin_privileges() {
        let mut pallet = PermissionPallet::new(ADMIN_ACCOUNT);
        assert!(!pallet.get_admin_setting());
        
        pallet.toggle_admin_setting(OriginBuilder::root()).unwrap();
        assert!(pallet.get_admin_setting());
        
        pallet.toggle_admin_setting(OriginBuilder::signed(ADMIN_ACCOUNT)).unwrap();
        assert!(!pallet.get_admin_setting());
        
        let result = pallet.toggle_admin_setting(OriginBuilder::signed(MEMBER_ACCOUNT));
        assert_eq!(result, Err("Admin privileges required"));
    }

    #[test]
    fn assign_and_remove_role_permission_logic_is_correct() {
        let mut pallet = PermissionPallet::new(ADMIN_ACCOUNT);
        
        assert!(pallet.assign_role(OriginBuilder::signed(ADMIN_ACCOUNT), NORMAL_ACCOUNT, CustomOrigin::Member).is_ok());
        assert_eq!(pallet.get_user_role(NORMAL_ACCOUNT), Some(&CustomOrigin::Member));
        
        let result = pallet.assign_role(OriginBuilder::signed(MEMBER_ACCOUNT), 4, CustomOrigin::Member);
        assert_eq!(result, Err("Admin privileges required"));
        
        assert!(pallet.remove_role(OriginBuilder::signed(ADMIN_ACCOUNT), NORMAL_ACCOUNT).is_ok());
        assert_eq!(pallet.get_user_role(NORMAL_ACCOUNT), None);
    }

    #[test]
    fn cannot_remove_role_from_system_admin_via_pallet() {
        let mut pallet = PermissionPallet::new(ADMIN_ACCOUNT);
        let result = pallet.remove_role(OriginBuilder::root(), ADMIN_ACCOUNT);
        assert_eq!(result, Err("Cannot remove admin role from system administrator"));
    }

    #[test]
    fn query_functions_list_correct_accounts() {
        let mut pallet = PermissionPallet::new(ADMIN_ACCOUNT);
        let other_admin = 4;
        pallet.assign_role(OriginBuilder::root(), MEMBER_ACCOUNT, CustomOrigin::Member).unwrap();
        pallet.assign_role(OriginBuilder::root(), other_admin, CustomOrigin::Admin).unwrap();

        let mut admins = pallet.list_admins();
        admins.sort(); 
        assert_eq!(admins, vec![ADMIN_ACCOUNT, other_admin]);

        let members = pallet.list_members();
        assert_eq!(members, vec![MEMBER_ACCOUNT]);
    }
}
