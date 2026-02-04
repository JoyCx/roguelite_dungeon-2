use crate::model::consumable::Consumable;
use crate::model::item_tier::ItemTier;
use crate::model::weapon::Weapon;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ItemDropType {
    Consumable(Consumable),
    Gold(u32),
    Weapon(Weapon),
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

    pub fn weapon(weapon: Weapon, x: i32, y: i32, tier: ItemTier) -> Self {
        Self {
            item_type: ItemDropType::Weapon(weapon),
            x,
            y,
            time_on_ground: 0.0,
            tier,
            stackable: false,
        }
    }

    pub fn get_glyph(&self) -> char {
        match &self.item_type {
            ItemDropType::Consumable(c) => {
                use crate::model::consumable::ConsumableType;
                match c.consumable_type {
                    ConsumableType::WeakHealingDraught => '❣',
                    ConsumableType::BandageRoll => '〓',
                    ConsumableType::AntitoxinVial => '☣',
                    ConsumableType::FireOilFlask => '⚱',
                    ConsumableType::BlessedBread => '☼',
                }
            }
            ItemDropType::Gold(_) => '❂',
            ItemDropType::Weapon(w) => {
                match w.weapon_type {
                    crate::model::weapon::WeaponType::Sword => '†',
                    crate::model::weapon::WeaponType::Bow => '🏹',
                    crate::model::weapon::WeaponType::Mace => '⚒',
                }
            }
        }
    }

    pub fn get_color(&self) -> ratatui::style::Color {
        use ratatui::style::Color;
        match &self.item_type {
            ItemDropType::Consumable(c) => {
                use crate::model::consumable::ConsumableType;
                match c.consumable_type {
                    ConsumableType::WeakHealingDraught => Color::LightRed,
                    ConsumableType::BandageRoll => Color::LightGreen,
                    ConsumableType::AntitoxinVial => Color::Cyan,
                    ConsumableType::FireOilFlask => Color::LightYellow,
                    ConsumableType::BlessedBread => Color::LightMagenta,
                }
            }
            ItemDropType::Gold(_) => Color::Yellow,
            ItemDropType::Weapon(_) => Color::White,
        }
    }

    pub fn get_description(&self) -> String {
        match &self.item_type {
            ItemDropType::Consumable(c) => c.name.clone(),
            ItemDropType::Gold(amount) => format!("{} gold", amount),
            ItemDropType::Weapon(w) => w.name.clone(),
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.time_on_ground += delta;
    }
}
