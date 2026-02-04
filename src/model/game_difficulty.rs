use serde::{Deserialize, Serialize};

/// Game difficulty levels affecting item drop rates and gameplay challenge
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq, Default)]
pub enum GameDifficulty {
    Easy,
    #[default]
    Normal,
    Hard,
    Death,
}

impl GameDifficulty {
    pub fn name(&self) -> &'static str {
        match self {
            GameDifficulty::Easy => "Easy",
            GameDifficulty::Normal => "Normal",
            GameDifficulty::Hard => "Hard",
            GameDifficulty::Death => "Death",
        }
    }

    /// Returns multiplier for rare item drop chance (higher = more rare items)
    pub fn rarity_multiplier(&self) -> f32 {
        match self {
            GameDifficulty::Easy => 0.5,   // 50% of normal
            GameDifficulty::Normal => 1.0, // Baseline
            GameDifficulty::Hard => 1.5,   // 150% of normal
            GameDifficulty::Death => 2.5,  // 250% of normal
        }
    }

    /// Returns drop chance for each rarity tier (as percentage, 0-100)
    /// Adjusted by difficulty multiplier
    pub fn get_rarity_drop_chance(&self, rarity: &crate::model::item_rarity::ItemRarity) -> f32 {
        let multiplier = self.rarity_multiplier();
        let base_chance = match rarity {
            crate::model::item_rarity::ItemRarity::Common => 50.0,
            crate::model::item_rarity::ItemRarity::Rare => 25.0,
            crate::model::item_rarity::ItemRarity::Epic => 15.0,
            crate::model::item_rarity::ItemRarity::Exotic => 5.0,
            crate::model::item_rarity::ItemRarity::Legendary => 3.0,
            crate::model::item_rarity::ItemRarity::Mythic => 1.5,
            crate::model::item_rarity::ItemRarity::Godly => 0.5,
        };
        // Apply multiplier but cap max chance at 100%
        (base_chance * multiplier).min(100.0)
    }

    /// Backward-compatible alias for get_rarity_drop_chance()
    pub fn get_tier_drop_chance(&self, rarity: &crate::model::item_rarity::ItemRarity) -> f32 {
        self.get_rarity_drop_chance(rarity)
    }

    /// Returns multiplier for enemy detection radius (higher difficulty = bigger radius)
    pub fn detection_radius_multiplier(&self) -> f32 {
        match self {
            GameDifficulty::Easy => 0.7,      // 70% of normal
            GameDifficulty::Normal => 1.0,    // Baseline
            GameDifficulty::Hard => 1.4,      // 140% of normal
            GameDifficulty::Death => 1.8,     // 180% of normal
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difficulty_multipliers() {
        assert_eq!(GameDifficulty::Easy.rarity_multiplier(), 0.5);
        assert_eq!(GameDifficulty::Normal.rarity_multiplier(), 1.0);
        assert_eq!(GameDifficulty::Hard.rarity_multiplier(), 1.5);
        assert_eq!(GameDifficulty::Death.rarity_multiplier(), 2.5);
    }

    #[test]
    fn test_default_difficulty() {
        assert_eq!(GameDifficulty::default(), GameDifficulty::Normal);
    }
}
