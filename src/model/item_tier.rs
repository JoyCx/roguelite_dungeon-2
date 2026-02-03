//! Legacy module providing backward compatibility aliases.
//!
//! New code should use:
//! - `game_difficulty::GameDifficulty` for difficulty levels
//! - `item_rarity::ItemRarity` for item rarity tiers
//!
//! This module re-exports from those modules for backward compatibility.

pub use crate::model::game_difficulty::GameDifficulty as Difficulty;
pub use crate::model::item_rarity::ItemRarity as ItemTier;
