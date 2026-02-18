use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameSave {
    pub player_name: String,
    pub player_stats: PlayerStats,
    pub inventory_data: InventoryData,
    pub skill_tree_data: SkillTreeData,
    pub ultimate_shop_data: UltimateShopData,
    pub floor_level: u32,
    pub max_levels: u32,
    pub position_x: i32,
    pub position_y: i32,
    pub difficulty: String,
    pub time_elapsed: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerStats {
    pub attack_damage: i32,
    pub attack_length: i32,
    pub attack_width: i32,
    pub dash_distance: i32,
    pub health: i32,
    pub max_health: i32,
    pub gold: u32,
    pub enemies_killed: u32,
    pub speed: f32,
    pub ultimate_charge: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InventoryData {
    pub weapons: Vec<WeaponData>,
    pub current_weapon_index: usize,
    pub consumables: Vec<ConsumableData>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WeaponData {
    pub weapon_type: String,
    pub damage: i32,
    pub cooldown: f32,
    pub name: String,
    pub rarity: String,
    pub enchants: Vec<EnchantData>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EnchantData {
    pub enchant_type: String,
    pub value: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConsumableData {
    pub consumable_type: String,
    pub quantity: u32,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SkillTreeData {
    pub path_nodes: Vec<PathNodeData>,
    pub chosen_path: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PathNodeData {
    pub path_type: String,
    pub level: u32,
    pub total_cost: u32,
    pub health_multiplier: f32,
    pub damage_multiplier: f32,
    pub speed_multiplier: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UltimateShopData {
    pub owned_ultimates: Vec<String>,
    pub stat_upgrades: Vec<(String, u32)>,
    pub current_ultimate_type: String,
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
            gold: 0,
            enemies_killed: 0,
            speed: 100.0,
            ultimate_charge: 0.0,
        }
    }
}

impl Default for InventoryData {
    fn default() -> Self {
        Self {
            weapons: vec![],
            current_weapon_index: 0,
            consumables: Vec::new(),
        }
    }
}

impl Default for SkillTreeData {
    fn default() -> Self {
        Self {
            path_nodes: vec![],
            chosen_path: None,
        }
    }
}

impl Default for UltimateShopData {
    fn default() -> Self {
        Self {
            owned_ultimates: vec![],
            stat_upgrades: vec![],
            current_ultimate_type: "Shockwave".to_string(),
        }
    }
}

impl Default for GameSave {
    fn default() -> Self {
        Self {
            player_name: "Player".to_string(),
            player_stats: PlayerStats::default(),
            inventory_data: InventoryData::default(),
            skill_tree_data: SkillTreeData::default(),
            ultimate_shop_data: UltimateShopData::default(),
            floor_level: 1,
            max_levels: 10,
            position_x: 0,
            position_y: 0,
            difficulty: "Normal".to_string(),
            time_elapsed: 0.0,
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

    /// Save game with player name as filename
    pub fn save(&self) -> std::io::Result<()> {
        Self::ensure_saves_dir()?;
        // Sanitize player name for filename
        let safe_name = self
            .player_name
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '_' })
            .collect::<String>();
        let path = format!("saves/{}.json", safe_name);
        let data = serde_json::to_string_pretty(self).unwrap();
        fs::write(path, data)
    }

    /// Load game by player name
    pub fn load(player_name: &str) -> std::io::Result<Self> {
        // Sanitize player name for filename
        let safe_name = player_name
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '_' })
            .collect::<String>();
        let path = format!("saves/{}.json", safe_name);
        fs::read_to_string(&path).and_then(|data| {
            serde_json::from_str(&data)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
        })
    }

    /// List all available saves
    pub fn list_saves() -> std::io::Result<Vec<String>> {
        Self::ensure_saves_dir()?;
        let mut saves = Vec::new();
        if let Ok(entries) = fs::read_dir("saves") {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "json") {
                    if let Some(filename) = path.file_stem() {
                        if let Some(name) = filename.to_str() {
                            saves.push(name.to_string());
                        }
                    }
                }
            }
        }
        saves.sort();
        Ok(saves)
    }

    /// Check if save exists
    pub fn save_exists(player_name: &str) -> bool {
        let safe_name = player_name
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '_' })
            .collect::<String>();
        let path = format!("saves/{}.json", safe_name);
        Path::new(&path).exists()
    }
}
