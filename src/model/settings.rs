use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings {
    pub move_up: String,
    pub move_left: String,
    pub move_down: String,
    pub move_right: String,
    pub attack: String,
    pub dash: String,
    pub block: String,
    pub pick_up: String,
    pub toggle_inv: String,
    pub special_item: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            move_up: "W".into(),
            move_left: "A".into(),
            move_down: "S".into(),
            move_right: "D".into(),
            attack: "LeftClick".into(),
            dash: "Space".into(),
            block: "RightClick".into(),
            pick_up: "E".into(),
            toggle_inv: "C".into(),
            special_item: "Q".into(),
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        fs::read_to_string("settings.json")
            .and_then(|data| serde_json::from_str(&data).map_err(|e| e.into()))
            .unwrap_or_else(|_| {
                let s = Self::default();
                let _ = s.save();
                s
            })
    }

    pub fn save(&self) -> std::io::Result<()> {
        let data = serde_json::to_string_pretty(self).unwrap();
        fs::write("settings.json", data)
    }
}
