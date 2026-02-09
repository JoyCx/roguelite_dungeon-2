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

    pub fn get_glyph(&self) -> &'static str {
        match &self.item_type {
            ItemDropType::Consumable(c) => c.consumable_type.get_glyph(),
            ItemDropType::Gold(_) => "¤",
            ItemDropType::Weapon(w) => w.weapon_type.get_glyph(),
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
        match &self.item_type {
            // Consumables use function-aware coloring (not tier-based)
            ItemDropType::Consumable(c) => c.consumable_type.get_color(),
            // Gold is always gray
            ItemDropType::Gold(_) => Color::DarkGray,
            // Weapons use tier-based coloring
            ItemDropType::Weapon(_) => match self.tier {
                ItemTier::Common => Color::DarkGray,
                ItemTier::Rare => Color::Cyan,
                ItemTier::Epic => Color::Blue,
                ItemTier::Exotic => Color::Yellow,
                ItemTier::Legendary => Color::Rgb(255, 215, 0), // Gold
                ItemTier::Mythic => Color::Rgb(255, 200, 80),   // Sunfire gold
                ItemTier::Godly => Color::Rgb(255, 255, 210),   // Radiant white-gold
            },
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.time_on_ground += delta;
    }
}
