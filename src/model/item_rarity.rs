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

    pub fn color_code(&self) -> &'static str {
        match self {
            ItemRarity::Common => "⚪",    // White
            ItemRarity::Rare => "🟢",      // Green
            ItemRarity::Epic => "🔵",      // Blue
            ItemRarity::Exotic => "🟣",    // Purple
            ItemRarity::Legendary => "🟠", // Orange
            ItemRarity::Mythic => "🔴",    // Red
            ItemRarity::Godly => "⭐",     // Star
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
