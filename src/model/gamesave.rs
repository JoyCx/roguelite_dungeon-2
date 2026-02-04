use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameSave {
    pub player_stats: PlayerStats,
    pub floor_level: u32,
    pub position_x: i32,
    pub position_y: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerStats {
    pub attack_damage: i32,
    pub attack_length: i32,
    pub attack_width: i32,
    pub dash_distance: i32,
    pub health: i32,
    pub max_health: i32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            attack_damage: 5,
            attack_length: 2,
            attack_width: 1,
            dash_distance: 5,
            health: 100,
            max_health: 100,
        }
    }
}

impl Default for GameSave {
    fn default() -> Self {
        Self {
            player_stats: PlayerStats::default(),
            floor_level: 1,
            position_x: 0,
            position_y: 0,
        }
    }
}

impl GameSave {
    pub fn ensure_saves_dir() -> std::io::Result<()> {
        let saves_dir = "saves";
        if !Path::new(saves_dir).exists() {
            fs::create_dir(saves_dir)?;
        }
        Ok(())
    }

    pub fn save(&self, slot: u32) -> std::io::Result<()> {
        Self::ensure_saves_dir()?;
        let path = format!("saves/save_{}.json", slot);
        let data = serde_json::to_string_pretty(self).unwrap();
        fs::write(path, data)
    }

    pub fn load(slot: u32) -> std::io::Result<Self> {
        let path = format!("saves/save_{}.json", slot);
        fs::read_to_string(&path).and_then(|data| {
            serde_json::from_str(&data)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
        })
    }

}
