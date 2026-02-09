use crate::model::attack_pattern::AttackPattern;
use crate::model::item_tier::ItemTier;
use crate::model::reach_shape::ReachShape;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WeaponType {
    Sword,
    Bow,
    Mace,
    Spear,
    Axe,
    Staff,
}

impl WeaponType {
    /// Get consistent glyph for this weapon type (applies across all rarities)
    pub fn get_glyph(&self) -> &'static str {
        match self {
            WeaponType::Sword => "🗡️",
            WeaponType::Bow => "🏹",
            WeaponType::Mace => "🔨",
            WeaponType::Spear => "⚜️",
            WeaponType::Axe => "🪓",
            WeaponType::Staff => "⚚",
        }
    }
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
    pub rarity: ItemTier,
    pub attack_pattern: AttackPattern,
    #[serde(skip)]
    pub reach_shape: ReachShape,
}

impl Weapon {
    // === COMMON WEAPONS (ItemTier::Common) ===
    pub fn new_sword() -> Self {
        Self {
            weapon_type: WeaponType::Sword,
            damage: 5,
            cooldown: 0.5,
            name: "Iron Sword".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Common,
            attack_pattern: AttackPattern::BasicSlash,
            reach_shape: ReachShape::Arc,
        }
    }

    pub fn new_bow() -> Self {
        Self {
            weapon_type: WeaponType::Bow,
            damage: 3,
            cooldown: 0.3,
            name: "Wood Bow".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Common,
            attack_pattern: AttackPattern::ArrowShot(5),
            reach_shape: ReachShape::Line(1),
        }
    }

    pub fn new_mace() -> Self {
        Self {
            weapon_type: WeaponType::Mace,
            damage: 8,
            cooldown: 0.8,
            name: "Stone Mace".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Common,
            attack_pattern: AttackPattern::GroundSlam(2),
            reach_shape: ReachShape::Area,
        }
    }

    // === RARE WEAPONS ===
    pub fn steel_sword() -> Self {
        Self {
            weapon_type: WeaponType::Sword,
            damage: 10,
            cooldown: 0.45,
            name: "Steel Sword".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Rare,
            attack_pattern: AttackPattern::SwordThrust(3),
            reach_shape: ReachShape::Arc,
        }
    }

    pub fn composite_bow() -> Self {
        Self {
            weapon_type: WeaponType::Bow,
            damage: 7,
            cooldown: 0.25,
            name: "Composite Bow".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Rare,
            attack_pattern: AttackPattern::MultiShot(5, 2),
            reach_shape: ReachShape::Line(2),
        }
    }

    pub fn steel_mace() -> Self {
        Self {
            weapon_type: WeaponType::Mace,
            damage: 14,
            cooldown: 0.7,
            name: "Steel Mace".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Rare,
            attack_pattern: AttackPattern::GroundSlam(3),
            reach_shape: ReachShape::Area,
        }
    }

    pub fn spear() -> Self {
        Self {
            weapon_type: WeaponType::Spear,
            damage: 9,
            cooldown: 0.4,
            name: "Iron Spear".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Rare,
            attack_pattern: AttackPattern::SwordThrust(4),
            reach_shape: ReachShape::Line(1),
        }
    }

    pub fn battle_axe() -> Self {
        Self {
            weapon_type: WeaponType::Axe,
            damage: 13,
            cooldown: 0.75,
            name: "Battle Axe".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Rare,
            attack_pattern: AttackPattern::Barrage(3),
            reach_shape: ReachShape::Arc,
        }
    }

    pub fn quarterstaff() -> Self {
        Self {
            weapon_type: WeaponType::Staff,
            damage: 6,
            cooldown: 0.35,
            name: "Quarterstaff".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Rare,
            attack_pattern: AttackPattern::WhirlwindAttack,
            reach_shape: ReachShape::Area,
        }
    }

    // === EPIC WEAPONS ===
    pub fn mithril_sword() -> Self {
        Self {
            weapon_type: WeaponType::Sword,
            damage: 18,
            cooldown: 0.4,
            name: "Mithril Sword".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Epic,
            attack_pattern: AttackPattern::CrescentSlash,
            reach_shape: ReachShape::Arc,
        }
    }

    pub fn longbow() -> Self {
        Self {
            weapon_type: WeaponType::Bow,
            damage: 14,
            cooldown: 0.2,
            name: "Longbow".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Epic,
            attack_pattern: AttackPattern::Barrage(5),
            reach_shape: ReachShape::Line(3),
        }
    }

    pub fn warhammer() -> Self {
        Self {
            weapon_type: WeaponType::Mace,
            damage: 22,
            cooldown: 0.65,
            name: "Warhammer".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Epic,
            attack_pattern: AttackPattern::GroundSlam(4),
            reach_shape: ReachShape::Area,
        }
    }

    pub fn halberd() -> Self {
        Self {
            weapon_type: WeaponType::Spear,
            damage: 16,
            cooldown: 0.35,
            name: "Halberd".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Epic,
            attack_pattern: AttackPattern::SwordThrust(5),
            reach_shape: ReachShape::Line(2),
        }
    }

    pub fn broad_axe() -> Self {
        Self {
            weapon_type: WeaponType::Axe,
            damage: 20,
            cooldown: 0.65,
            name: "Broad Axe".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Epic,
            attack_pattern: AttackPattern::Barrage(4),
            reach_shape: ReachShape::Arc,
        }
    }

    pub fn frost_staff() -> Self {
        Self {
            weapon_type: WeaponType::Staff,
            damage: 12,
            cooldown: 0.3,
            name: "Frost Staff".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Epic,
            attack_pattern: AttackPattern::FrostNova(4),
            reach_shape: ReachShape::Area,
        }
    }

    pub fn fire_staff() -> Self {
        Self {
            weapon_type: WeaponType::Staff,
            damage: 15,
            cooldown: 0.35,
            name: "Fire Staff".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Epic,
            attack_pattern: AttackPattern::Fireball(3),
            reach_shape: ReachShape::Area,
        }
    }

    // === EXOTIC WEAPONS ===
    pub fn adamant_sword() -> Self {
        Self {
            weapon_type: WeaponType::Sword,
            damage: 26,
            cooldown: 0.35,
            name: "Adamant Sword".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Exotic,
            attack_pattern: AttackPattern::WhirlwindAttack,
            reach_shape: ReachShape::Arc,
        }
    }

    pub fn platinum_bow() -> Self {
        Self {
            weapon_type: WeaponType::Bow,
            damage: 22,
            cooldown: 0.15,
            name: "Platinum Bow".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Exotic,
            attack_pattern: AttackPattern::MultiShot(7, 3),
            reach_shape: ReachShape::Line(4),
        }
    }

    pub fn molten_hammer() -> Self {
        Self {
            weapon_type: WeaponType::Mace,
            damage: 32,
            cooldown: 0.6,
            name: "Molten Hammer".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Exotic,
            attack_pattern: AttackPattern::GroundSlam(5),
            reach_shape: ReachShape::Area,
        }
    }

    pub fn dragon_spear() -> Self {
        Self {
            weapon_type: WeaponType::Spear,
            damage: 24,
            cooldown: 0.3,
            name: "Dragon Spear".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Exotic,
            attack_pattern: AttackPattern::Barrage(6),
            reach_shape: ReachShape::Line(3),
        }
    }

    pub fn storm_axe() -> Self {
        Self {
            weapon_type: WeaponType::Axe,
            damage: 28,
            cooldown: 0.6,
            name: "Storm Axe".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Exotic,
            attack_pattern: AttackPattern::ChainLightning(5),
            reach_shape: ReachShape::Arc,
        }
    }

    pub fn arcane_staff() -> Self {
        Self {
            weapon_type: WeaponType::Staff,
            damage: 20,
            cooldown: 0.25,
            name: "Arcane Staff".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Exotic,
            attack_pattern: AttackPattern::ChainLightning(6),
            reach_shape: ReachShape::Area,
        }
    }

    pub fn meteor_staff() -> Self {
        Self {
            weapon_type: WeaponType::Staff,
            damage: 25,
            cooldown: 0.4,
            name: "Meteor Staff".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Exotic,
            attack_pattern: AttackPattern::MeteorShower(6, 3),
            reach_shape: ReachShape::Area,
        }
    }

    // === LEGENDARY WEAPONS ===
    pub fn excalibur() -> Self {
        Self {
            weapon_type: WeaponType::Sword,
            damage: 35,
            cooldown: 0.3,
            name: "Excalibur".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Legendary,
            attack_pattern: AttackPattern::CrescentSlash,
            reach_shape: ReachShape::Arc,
        }
    }

    pub fn divine_bow() -> Self {
        Self {
            weapon_type: WeaponType::Bow,
            damage: 28,
            cooldown: 0.1,
            name: "Divine Bow".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Legendary,
            attack_pattern: AttackPattern::PiercingShot(10),
            reach_shape: ReachShape::Line(5),
        }
    }

    pub fn mjolnir() -> Self {
        Self {
            weapon_type: WeaponType::Mace,
            damage: 40,
            cooldown: 0.55,
            name: "Mjolnir".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Legendary,
            attack_pattern: AttackPattern::GroundSlam(6),
            reach_shape: ReachShape::Area,
        }
    }

    pub fn gungnir() -> Self {
        Self {
            weapon_type: WeaponType::Spear,
            damage: 32,
            cooldown: 0.25,
            name: "Gungnir".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Legendary,
            attack_pattern: AttackPattern::SwordThrust(7),
            reach_shape: ReachShape::Line(4),
        }
    }

    pub fn world_splitter() -> Self {
        Self {
            weapon_type: WeaponType::Axe,
            damage: 36,
            cooldown: 0.55,
            name: "World Splitter".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Legendary,
            attack_pattern: AttackPattern::Vortex(5),
            reach_shape: ReachShape::Arc,
        }
    }

    pub fn infinity_staff() -> Self {
        Self {
            weapon_type: WeaponType::Staff,
            damage: 30,
            cooldown: 0.2,
            name: "Infinity Staff".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Legendary,
            attack_pattern: AttackPattern::ChainLightning(8),
            reach_shape: ReachShape::Area,
        }
    }

    // === MYTHIC WEAPONS ===
    pub fn primordial_blade() -> Self {
        Self {
            weapon_type: WeaponType::Sword,
            damage: 45,
            cooldown: 0.25,
            name: "Primordial Blade".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Mythic,
            attack_pattern: AttackPattern::WhirlwindAttack,
            reach_shape: ReachShape::Arc,
        }
    }

    pub fn celestial_bow() -> Self {
        Self {
            weapon_type: WeaponType::Bow,
            damage: 38,
            cooldown: 0.08,
            name: "Celestial Bow".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Mythic,
            attack_pattern: AttackPattern::Barrage(8),
            reach_shape: ReachShape::Line(6),
        }
    }

    pub fn titan_hammer() -> Self {
        Self {
            weapon_type: WeaponType::Mace,
            damage: 50,
            cooldown: 0.5,
            name: "Titan Hammer".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Mythic,
            attack_pattern: AttackPattern::GroundSlam(7),
            reach_shape: ReachShape::Area,
        }
    }

    pub fn void_spear() -> Self {
        Self {
            weapon_type: WeaponType::Spear,
            damage: 42,
            cooldown: 0.2,
            name: "Void Spear".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Mythic,
            attack_pattern: AttackPattern::CrescentSlash,
            reach_shape: ReachShape::Line(5),
        }
    }

    pub fn chaos_axe() -> Self {
        Self {
            weapon_type: WeaponType::Axe,
            damage: 46,
            cooldown: 0.5,
            name: "Chaos Axe".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Mythic,
            attack_pattern: AttackPattern::Vortex(7),
            reach_shape: ReachShape::Arc,
        }
    }

    pub fn cosmic_staff() -> Self {
        Self {
            weapon_type: WeaponType::Staff,
            damage: 40,
            cooldown: 0.15,
            name: "Cosmic Staff".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Mythic,
            attack_pattern: AttackPattern::MeteorShower(8, 4),
            reach_shape: ReachShape::Area,
        }
    }

    // === GODLY WEAPONS ===
    pub fn godly_greatsword() -> Self {
        Self {
            weapon_type: WeaponType::Sword,
            damage: 55,
            cooldown: 0.2,
            name: "Godly Greatsword".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Godly,
            attack_pattern: AttackPattern::SwordThrust(8),
            reach_shape: ReachShape::Arc,
        }
    }

    pub fn heavens_bow() -> Self {
        Self {
            weapon_type: WeaponType::Bow,
            damage: 48,
            cooldown: 0.05,
            name: "Heaven's Bow".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Godly,
            attack_pattern: AttackPattern::PiercingShot(12),
            reach_shape: ReachShape::Line(8),
        }
    }

    pub fn omnipotent_hammer() -> Self {
        Self {
            weapon_type: WeaponType::Mace,
            damage: 60,
            cooldown: 0.45,
            name: "Omnipotent Hammer".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Godly,
            attack_pattern: AttackPattern::GroundSlam(8),
            reach_shape: ReachShape::Area,
        }
    }

    pub fn dimensional_spear() -> Self {
        Self {
            weapon_type: WeaponType::Spear,
            damage: 52,
            cooldown: 0.15,
            name: "Dimensional Spear".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Godly,
            attack_pattern: AttackPattern::Barrage(10),
            reach_shape: ReachShape::Line(6),
        }
    }

    pub fn apocalypse_axe() -> Self {
        Self {
            weapon_type: WeaponType::Axe,
            damage: 56,
            cooldown: 0.45,
            name: "Apocalypse Axe".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Godly,
            attack_pattern: AttackPattern::Vortex(8),
            reach_shape: ReachShape::Arc,
        }
    }

    pub fn transcendence_staff() -> Self {
        Self {
            weapon_type: WeaponType::Staff,
            damage: 50,
            cooldown: 0.1,
            name: "Transcendence Staff".to_string(),
            enchants: Vec::new(),
            rarity: ItemTier::Godly,
            attack_pattern: AttackPattern::ChainLightning(10),
            reach_shape: ReachShape::Area,
        }
    }

    /// Generate a random weapon of a given rarity tier
    pub fn random_for_rarity(rarity: &ItemTier) -> Self {
        use rand::Rng;
        let mut rng = rand::rng();

        match rarity {
            ItemTier::Common => match rng.random_range(0..3) {
                0 => Self::new_sword(),
                1 => Self::new_bow(),
                _ => Self::new_mace(),
            },
            ItemTier::Rare => match rng.random_range(0..6) {
                0 => Self::steel_sword(),
                1 => Self::composite_bow(),
                2 => Self::steel_mace(),
                3 => Self::spear(),
                4 => Self::battle_axe(),
                _ => Self::quarterstaff(),
            },
            ItemTier::Epic => match rng.random_range(0..6) {
                0 => Self::mithril_sword(),
                1 => Self::longbow(),
                2 => Self::warhammer(),
                3 => Self::halberd(),
                4 => Self::broad_axe(),
                _ if rng.random_bool(0.5) => Self::frost_staff(),
                _ => Self::fire_staff(),
            },
            ItemTier::Exotic => match rng.random_range(0..7) {
                0 => Self::adamant_sword(),
                1 => Self::platinum_bow(),
                2 => Self::molten_hammer(),
                3 => Self::dragon_spear(),
                4 => Self::storm_axe(),
                5 => Self::arcane_staff(),
                _ => Self::meteor_staff(),
            },
            ItemTier::Legendary => match rng.random_range(0..6) {
                0 => Self::excalibur(),
                1 => Self::divine_bow(),
                2 => Self::mjolnir(),
                3 => Self::gungnir(),
                4 => Self::world_splitter(),
                _ => Self::infinity_staff(),
            },
            ItemTier::Mythic => match rng.random_range(0..6) {
                0 => Self::primordial_blade(),
                1 => Self::celestial_bow(),
                2 => Self::titan_hammer(),
                3 => Self::void_spear(),
                4 => Self::chaos_axe(),
                _ => Self::cosmic_staff(),
            },
            ItemTier::Godly => match rng.random_range(0..6) {
                0 => Self::godly_greatsword(),
                1 => Self::heavens_bow(),
                2 => Self::omnipotent_hammer(),
                3 => Self::dimensional_spear(),
                4 => Self::apocalypse_axe(),
                _ => Self::transcendence_staff(),
            },
        }
    }

    #[allow(dead_code)] // Utility method for calculating weapon damage with enchants
    pub fn get_total_damage(&self) -> i32 {
        let mut total = self.damage;
        for enchant in &self.enchants {
            if matches!(enchant.enchant_type, EnchantType::DamageIncrease) {
                total += enchant.value;
            }
        }
        total
    }

    #[allow(dead_code)] // Utility method for calculating radius bonuses
    pub fn get_radius_bonus(&self) -> i32 {
        let mut bonus = 0;
        for enchant in &self.enchants {
            if matches!(enchant.enchant_type, EnchantType::RadiusIncrease) {
                bonus += enchant.value;
            }
        }
        bonus
    }

    #[allow(dead_code)] // Will be used when enchantments are applied
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

    pub fn add_weapon(&mut self, weapon: Weapon) -> bool {
        if self.weapons.len() < 9 {
            self.weapons.push(weapon);
            true
        } else {
            false // Inventory full
        }
    }

    pub fn is_full(&self) -> bool {
        self.weapons.len() >= 9
    }

    pub fn remove_weapon(&mut self, slot: usize) -> Option<Weapon> {
        if slot < self.weapons.len() {
            let weapon = self.weapons.remove(slot);
            // Adjust current index if needed
            if self.current_weapon_index >= self.weapons.len() && !self.weapons.is_empty() {
                self.current_weapon_index = self.weapons.len() - 1;
            }
            Some(weapon)
        } else {
            None
        }
    }
}
