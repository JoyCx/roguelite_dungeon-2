use crate::model::consumable::Consumable;
use crate::model::item_tier::ItemTier;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ItemDropType {
    Consumable(Consumable),
    Gold(u32),
    Weapon(crate::model::weapon::Weapon),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemDrop {
    pub item_type: ItemDropType,
    pub x: i32,
    pub y: i32,
    pub time_on_ground: f32, // For despawn logic later
    pub tier: ItemTier,
    pub stackable: bool,
}

impl ItemDrop {
    pub fn consumable(consumable: Consumable, x: i32, y: i32) -> Self {
        Self {
            item_type: ItemDropType::Consumable(consumable),
            x,
            y,
            time_on_ground: 0.0,
            tier: ItemTier::Common,
            stackable: true, // Consumables are stackable
        }
    }

    pub fn consumable_with_tier(consumable: Consumable, x: i32, y: i32, tier: ItemTier) -> Self {
        Self {
            item_type: ItemDropType::Consumable(consumable),
            x,
            y,
            time_on_ground: 0.0,
            tier,
            stackable: true,
        }
    }

    pub fn gold(amount: u32, x: i32, y: i32) -> Self {
        Self {
            item_type: ItemDropType::Gold(amount),
            x,
            y,
            time_on_ground: 0.0,
            tier: ItemTier::Common,
            stackable: true, // Gold always stacks
        }
    }

    pub fn weapon(weapon: crate::model::weapon::Weapon, x: i32, y: i32) -> Self {
        Self {
            item_type: ItemDropType::Weapon(weapon),
            x,
            y,
            time_on_ground: 0.0,
            tier: ItemTier::Common,
            stackable: false, // Weapons don't stack
        }
    }

    pub fn get_glyph(&self) -> char {
        match &self.item_type {
            ItemDropType::Consumable(c) => {
                use crate::model::consumable::ConsumableType;
                match c.consumable_type {
                    ConsumableType::WeakHealingDraught => '◓',
                    ConsumableType::BandageRoll => '⊞',
                    ConsumableType::AntitoxinVial => '✕',
                    ConsumableType::FireOilFlask => '◆',
                    ConsumableType::BlessedBread => '☆',
                }
            }
            ItemDropType::Gold(_) => '¤',
            ItemDropType::Weapon(w) => {
                use crate::model::weapon::WeaponType;
                match w.weapon_type {
                    WeaponType::Sword => match self.tier {
                        ItemTier::Common => 's',
                        ItemTier::Rare => '⚔',
                        ItemTier::Epic => '⚡',
                        ItemTier::Exotic => '✦',
                        ItemTier::Legendary => '✦',
                        ItemTier::Mythic => '§',
                        ItemTier::Godly => '◈',
                    },
                    WeaponType::Bow => match self.tier {
                        ItemTier::Common => 'b',
                        ItemTier::Rare => '🏹',
                        ItemTier::Epic => '⇄',
                        ItemTier::Exotic => '⊳',
                        ItemTier::Legendary => '⊳',
                        ItemTier::Mythic => '◬',
                        ItemTier::Godly => '◉',
                    },
                    WeaponType::Mace => match self.tier {
                        ItemTier::Common => 'm',
                        ItemTier::Rare => '⚒',
                        ItemTier::Epic => '⚙',
                        ItemTier::Exotic => '⊛',
                        ItemTier::Legendary => '⊛',
                        ItemTier::Mythic => '◉',
                        ItemTier::Godly => '⊙',
                    },
                    WeaponType::Spear => match self.tier {
                        ItemTier::Common => 'p',
                        ItemTier::Rare => '⟡',
                        ItemTier::Epic => '⤥',
                        ItemTier::Exotic => '⊲',
                        ItemTier::Legendary => '⊲',
                        ItemTier::Mythic => '⦒',
                        ItemTier::Godly => '⫷',
                    },
                    WeaponType::Axe => match self.tier {
                        ItemTier::Common => 'a',
                        ItemTier::Rare => '⛚',
                        ItemTier::Epic => '⚔',
                        ItemTier::Exotic => '⊬',
                        ItemTier::Legendary => '⊬',
                        ItemTier::Mythic => '◭',
                        ItemTier::Godly => '◬',
                    },
                    WeaponType::Staff => match self.tier {
                        ItemTier::Common => 'w',
                        ItemTier::Rare => '†',
                        ItemTier::Epic => '‡',
                        ItemTier::Exotic => '⊕',
                        ItemTier::Legendary => '⊕',
                        ItemTier::Mythic => '◎',
                        ItemTier::Godly => '☉',
                    },
                }
            }
        }
    }

    pub fn get_description(&self) -> String {
        match &self.item_type {
            ItemDropType::Consumable(c) => c.name.clone(),
            ItemDropType::Gold(amount) => format!("{} gold", amount),
            ItemDropType::Weapon(w) => format!("{} ({})", w.name, w.damage),
        }
    }

    pub fn get_glyph_color(&self) -> ratatui::prelude::Color {
        use ratatui::prelude::Color;
        match self.tier {
            ItemTier::Common => Color::Gray,
            ItemTier::Rare => Color::Blue,
            ItemTier::Epic => Color::Magenta,
            ItemTier::Exotic => Color::Yellow,
            ItemTier::Legendary => Color::LightYellow,
            ItemTier::Mythic => Color::Cyan,
            ItemTier::Godly => Color::Red,
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.time_on_ground += delta;
    }
}
