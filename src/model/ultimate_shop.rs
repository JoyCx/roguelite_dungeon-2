use crate::model::ultimate::UltimateType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents an ultimate ability available in the shop
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShopUltimate {
    pub ultimate_type: UltimateType,
    pub cost: u32,
    pub unlock_level: u32, // Minimum floor level to purchase
    pub damage_modifier: f32,
    pub cooldown_reduction: f32, // 0.1 = 10% reduction
}

impl ShopUltimate {
    pub fn new(
        ultimate_type: UltimateType,
        cost: u32,
        unlock_level: u32,
        damage_modifier: f32,
        cooldown_reduction: f32,
    ) -> Self {
        Self {
            ultimate_type,
            cost,
            unlock_level,
            damage_modifier,
            cooldown_reduction,
        }
    }

    pub fn is_locked(&self, current_level: u32) -> bool {
        current_level < self.unlock_level
    }

    pub fn get_damage(&self, base_damage: i32) -> i32 {
        (base_damage as f32 * self.damage_modifier) as i32
    }

    pub fn get_cooldown_reduced(&self, base_cooldown: f32) -> f32 {
        base_cooldown * (1.0 - self.cooldown_reduction)
    }
}

/// Different types of stat upgrades
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum StatUpgradeType {
    MaxHealth,
    AttackDamage,
    AttackSpeed,
    MovementSpeed,
    DashDistance,
}

impl StatUpgradeType {
    pub fn name(&self) -> &'static str {
        match self {
            StatUpgradeType::MaxHealth => "Max Health",
            StatUpgradeType::AttackDamage => "Attack Damage",
            StatUpgradeType::AttackSpeed => "Attack Speed",
            StatUpgradeType::MovementSpeed => "Movement Speed",
            StatUpgradeType::DashDistance => "Dash Distance",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            StatUpgradeType::MaxHealth => "Increases maximum health by 10",
            StatUpgradeType::AttackDamage => "Increases attack damage by 2",
            StatUpgradeType::AttackSpeed => "Reduces attack cooldown by 5%",
            StatUpgradeType::MovementSpeed => "Increases movement speed by 0.1",
            StatUpgradeType::DashDistance => "Increases dash distance by 1",
        }
    }
}

/// Represents a purchasable stat upgrade
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShopStatUpgrade {
    pub upgrade_type: StatUpgradeType,
    pub cost: u32,
    pub unlock_level: u32,
    pub max_upgrades: u32, // Max times this can be purchased (-1 for unlimited)
    pub stack_amount: f32,
}

impl ShopStatUpgrade {
    pub fn new(
        upgrade_type: StatUpgradeType,
        cost: u32,
        unlock_level: u32,
        max_upgrades: u32,
        stack_amount: f32,
    ) -> Self {
        Self {
            upgrade_type,
            cost,
            unlock_level,
            max_upgrades,
            stack_amount,
        }
    }

    pub fn is_locked(&self, current_level: u32) -> bool {
        current_level < self.unlock_level
    }

    pub fn can_purchase(&self, current_level: u32, times_purchased: u32) -> bool {
        !self.is_locked(current_level)
            && (self.max_upgrades == 0 || times_purchased < self.max_upgrades)
    }
}

/// Tracks owned ultimates and upgrade purchases
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UltimateShopInventory {
    pub owned_ultimates: Vec<UltimateType>,
    pub stat_upgrades: HashMap<StatUpgradeType, u32>, // Maps upgrade type to times purchased
}

impl Default for UltimateShopInventory {
    fn default() -> Self {
        Self {
            owned_ultimates: vec![],
            stat_upgrades: HashMap::new(),
        }
    }
}

impl UltimateShopInventory {
    pub fn owns_ultimate(&self, ultimate_type: &UltimateType) -> bool {
        self.owned_ultimates.contains(ultimate_type)
    }

    pub fn purchase_ultimate(&mut self, ultimate_type: UltimateType) -> bool {
        if !self.owns_ultimate(&ultimate_type) {
            self.owned_ultimates.push(ultimate_type);
            true
        } else {
            false
        }
    }

    pub fn purchase_stat_upgrade(&mut self, upgrade_type: StatUpgradeType) -> bool {
        let count = self.stat_upgrades.entry(upgrade_type).or_insert(0);
        *count += 1;
        true
    }

    pub fn get_upgrade_count(&self, upgrade_type: &StatUpgradeType) -> u32 {
        self.stat_upgrades.get(upgrade_type).copied().unwrap_or(0)
    }
}

/// Main Ultimate Shop system
#[derive(Clone, Debug)]
pub struct UltimateShop {
    pub ultimates: Vec<ShopUltimate>,
    pub stat_upgrades: Vec<ShopStatUpgrade>,
}

impl UltimateShop {
    pub fn new() -> Self {
        let shop = Self {
            ultimates: vec![
                ShopUltimate::new(UltimateType::Rage, 150, 1, 1.5, 0.2),
                ShopUltimate::new(UltimateType::Shockwave, 200, 3, 2.0, 0.15),
                ShopUltimate::new(UltimateType::Ghost, 250, 5, 1.2, 0.1),
            ],
            stat_upgrades: vec![
                ShopStatUpgrade::new(StatUpgradeType::MaxHealth, 100, 1, 0, 10.0),
                ShopStatUpgrade::new(StatUpgradeType::AttackDamage, 120, 1, 0, 2.0),
                ShopStatUpgrade::new(StatUpgradeType::AttackSpeed, 150, 2, 0, 0.05),
                ShopStatUpgrade::new(StatUpgradeType::MovementSpeed, 140, 2, 0, 0.1),
                ShopStatUpgrade::new(StatUpgradeType::DashDistance, 110, 1, 0, 1.0),
            ],
        };
        shop
    }

    pub fn get_available_ultimates(&self, current_level: u32) -> Vec<&ShopUltimate> {
        self.ultimates
            .iter()
            .filter(|ult| !ult.is_locked(current_level))
            .collect()
    }

    pub fn get_locked_ultimates(&self, current_level: u32) -> Vec<&ShopUltimate> {
        self.ultimates
            .iter()
            .filter(|ult| ult.is_locked(current_level))
            .collect()
    }

    pub fn get_available_upgrades(&self, current_level: u32) -> Vec<&ShopStatUpgrade> {
        self.stat_upgrades
            .iter()
            .filter(|upg| !upg.is_locked(current_level))
            .collect()
    }

    pub fn get_locked_upgrades(&self, current_level: u32) -> Vec<&ShopStatUpgrade> {
        self.stat_upgrades
            .iter()
            .filter(|upg| upg.is_locked(current_level))
            .collect()
    }

    pub fn can_afford(&self, price: u32, gold: u32) -> bool {
        gold >= price
    }

    pub fn purchase_ultimate(
        &self,
        gold: &mut u32,
        inventory: &mut UltimateShopInventory,
        ultimate_type: &UltimateType,
        current_level: u32,
    ) -> Result<String, String> {
        // Find the ultimate in shop
        let shop_ult = self
            .ultimates
            .iter()
            .find(|u| &u.ultimate_type == ultimate_type)
            .ok_or("Ultimate not found in shop")?;

        // Check if locked
        if shop_ult.is_locked(current_level) {
            return Err(format!("Unlocks at level {}", shop_ult.unlock_level));
        }

        // Check if already owned
        if inventory.owns_ultimate(ultimate_type) {
            return Err("Already owned".to_string());
        }

        // Check gold
        if !self.can_afford(shop_ult.cost, *gold) {
            return Err(format!("Need {} gold", shop_ult.cost - *gold));
        }

        // Purchase
        *gold -= shop_ult.cost;
        inventory.purchase_ultimate(ultimate_type.clone());
        Ok(format!("Purchased {}", ultimate_type.name()))
    }

    pub fn purchase_stat_upgrade(
        &self,
        gold: &mut u32,
        inventory: &mut UltimateShopInventory,
        upgrade_type: &StatUpgradeType,
        current_level: u32,
    ) -> Result<String, String> {
        // Find the upgrade in shop
        let shop_upg = self
            .stat_upgrades
            .iter()
            .find(|u| &u.upgrade_type == upgrade_type)
            .ok_or("Upgrade not found in shop")?;

        // Check if locked
        if shop_upg.is_locked(current_level) {
            return Err(format!("Unlocks at level {}", shop_upg.unlock_level));
        }

        // Check if can purchase
        let times_purchased = inventory.get_upgrade_count(upgrade_type);
        if !shop_upg.can_purchase(current_level, times_purchased) {
            return Err("Max upgrades reached".to_string());
        }

        // Check gold
        if !self.can_afford(shop_upg.cost, *gold) {
            return Err(format!("Need {} gold", shop_upg.cost - *gold));
        }

        // Purchase
        *gold -= shop_upg.cost;
        inventory.purchase_stat_upgrade(*upgrade_type);
        Ok(format!(
            "Purchased {} (Level {})",
            upgrade_type.name(),
            times_purchased + 1
        ))
    }
}

impl Default for UltimateShop {
    fn default() -> Self {
        Self::new()
    }
}
