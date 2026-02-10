use ratatui::prelude::Color;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq, PartialOrd, Ord, Default)]
pub enum ItemRarity {
    #[default]
    Common,
    Rare,
    Epic,
    Exotic,
    Legendary,
    Mythic,
    Godly,
}

impl ItemRarity {
    pub fn name(&self) -> &'static str {
        match self {
            ItemRarity::Common => "Common",
            ItemRarity::Rare => "Rare",
            ItemRarity::Epic => "Epic",
            ItemRarity::Exotic => "Exotic",
            ItemRarity::Legendary => "Legendary",
            ItemRarity::Mythic => "Mythic",
            ItemRarity::Godly => "Godly",
        }
    }

    /// Get rarity color for UI rendering (base fade progression)
    pub fn get_color(&self) -> Color {
        match self {
            ItemRarity::Common => Color::DarkGray,
            ItemRarity::Rare => Color::Cyan,
            ItemRarity::Epic => Color::Blue,
            ItemRarity::Exotic => Color::Yellow,
            ItemRarity::Legendary => Color::Rgb(255, 215, 0), // True gold
            ItemRarity::Mythic => Color::Rgb(255, 200, 80),   // Sunfire gold
            ItemRarity::Godly => Color::Rgb(255, 255, 210),   // Radiant white-gold
        }
    }

    /// Get fade/highlight color for animations
    pub fn get_fade_color(&self) -> Color {
        match self {
            ItemRarity::Common => Color::Gray,
            ItemRarity::Rare => Color::LightBlue,
            ItemRarity::Epic => Color::LightBlue,
            ItemRarity::Exotic => Color::LightYellow,
            ItemRarity::Legendary => Color::Rgb(255, 240, 180),
            ItemRarity::Mythic => Color::Rgb(255, 255, 160),
            ItemRarity::Godly => Color::White,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rarity_names() {
        assert_eq!(ItemRarity::Common.name(), "Common");
        assert_eq!(ItemRarity::Godly.name(), "Godly");
    }

    #[test]
    fn test_default_rarity() {
        assert_eq!(ItemRarity::default(), ItemRarity::Common);
    }
}
