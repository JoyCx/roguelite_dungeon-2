use crate::model::reach_shape::ReachShape;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WeaponType {
    Sword,
    Bow,
    Mace,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EnchantType {
    DamageIncrease,
    RadiusIncrease,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Enchant {
    pub enchant_type: EnchantType,
    pub value: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Weapon {
    pub weapon_type: WeaponType,
    pub damage: i32,
    pub cooldown: f32,
    pub name: String,
    pub enchants: Vec<Enchant>,
    #[serde(skip)]
    pub reach_shape: ReachShape,
}

impl Weapon {
    pub fn new_sword() -> Self {
        Self {
            weapon_type: WeaponType::Sword,
            damage: 5,
            cooldown: 0.5,
            name: "Sword".to_string(),
            enchants: Vec::new(),
            reach_shape: ReachShape::Arc,
        }
    }

    pub fn new_bow() -> Self {
        Self {
            weapon_type: WeaponType::Bow,
            damage: 3,
            cooldown: 0.3,
            name: "Bow".to_string(),
            enchants: Vec::new(),
            reach_shape: ReachShape::Line(1),
        }
    }

    pub fn new_mace() -> Self {
        Self {
            weapon_type: WeaponType::Mace,
            damage: 8,
            cooldown: 0.8,
            name: "Mace".to_string(),
            enchants: Vec::new(),
            reach_shape: ReachShape::Area,
        }
    }

    pub fn get_total_damage(&self) -> i32 {
        let mut total = self.damage;
        for enchant in &self.enchants {
            if matches!(enchant.enchant_type, EnchantType::DamageIncrease) {
                total += enchant.value;
            }
        }
        total
    }

    pub fn get_radius_bonus(&self) -> i32 {
        let mut bonus = 0;
        for enchant in &self.enchants {
            if matches!(enchant.enchant_type, EnchantType::RadiusIncrease) {
                bonus += enchant.value;
            }
        }
        bonus
    }

    pub fn add_enchant(&mut self, enchant: Enchant) {
        self.enchants.push(enchant);
    }
}

#[derive(Clone, Debug)]
pub struct WeaponInventory {
    pub weapons: Vec<Weapon>,
    pub current_weapon_index: usize,
}

impl Default for WeaponInventory {
    fn default() -> Self {
        Self {
            weapons: vec![Weapon::new_sword(), Weapon::new_bow()],
            current_weapon_index: 0,
        }
    }
}

impl WeaponInventory {
    pub fn get_current_weapon(&self) -> Option<&Weapon> {
        self.weapons.get(self.current_weapon_index)
    }

    pub fn switch_weapon(&mut self, slot: usize) {
        if slot < self.weapons.len() {
            self.current_weapon_index = slot;
        }
    }

    pub fn add_weapon(&mut self, weapon: Weapon) {
        if self.weapons.len() < 9 {
            self.weapons.push(weapon);
        }
    }
}
