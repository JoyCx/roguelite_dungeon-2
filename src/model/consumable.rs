use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ConsumableType {
    WeakHealingDraught, // Healing over 5 sec
    BandageRoll,        // Removes bleed + instant heal
    AntitoxinVial,      // Removes poison + immunity
    FireOilFlask,       // Throw damage + burn
    BlessedBread,       // Slow healing over 8 sec
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Consumable {
    pub consumable_type: ConsumableType,
    pub quantity: u32,
    pub name: String,
    pub description: String,
}

impl Consumable {
    pub fn new(consumable_type: ConsumableType) -> Self {
        match consumable_type {
            ConsumableType::WeakHealingDraught => Self {
                consumable_type,
                quantity: 1,
                name: "Weak Healing Draught".to_string(),
                description: "Sour, cloudy, and vaguely alive.".to_string(),
            },
            ConsumableType::BandageRoll => Self {
                consumable_type,
                quantity: 1,
                name: "Bandage Roll".to_string(),
                description: "Clean-ish linen. Good enough.".to_string(),
            },
            ConsumableType::AntitoxinVial => Self {
                consumable_type,
                quantity: 1,
                name: "Antitoxin Vial".to_string(),
                description: "Burns worse than the poison. That's how you know it's working."
                    .to_string(),
            },
            ConsumableType::FireOilFlask => Self {
                consumable_type,
                quantity: 1,
                name: "Fire Oil Flask".to_string(),
                description: "Lamp oil with violent intent.".to_string(),
            },
            ConsumableType::BlessedBread => Self {
                consumable_type,
                quantity: 1,
                name: "Blessed Bread".to_string(),
                description: "Dry. Holy. Comforting.".to_string(),
            },
        }
    }

    pub fn get_healing(&self) -> Option<(f32, f32)> {
        // Returns (healing_per_sec, duration)
        match self.consumable_type {
            ConsumableType::WeakHealingDraught => Some((2.0, 5.0)), // 10 total
            ConsumableType::BandageRoll => Some((6.0, 0.0)),        // Instant
            ConsumableType::BlessedBread => Some((1.0, 8.0)),       // 8 total
            _ => None,
        }
    }

    pub fn get_use_time(&self) -> f32 {
        match self.consumable_type {
            ConsumableType::WeakHealingDraught => 0.0, // Instant
            ConsumableType::BandageRoll => 2.0,        // 2 seconds
            ConsumableType::AntitoxinVial => 0.0,      // Instant
            ConsumableType::FireOilFlask => 0.5,       // 0.5 second throw
            ConsumableType::BlessedBread => 1.0,       // 1 second
        }
    }

    pub fn is_stackable(&self) -> bool {
        matches!(
            self.consumable_type,
            ConsumableType::WeakHealingDraught
                | ConsumableType::BandageRoll
                | ConsumableType::AntitoxinVial
                | ConsumableType::BlessedBread
        )
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ConsumableInventory {
    pub items: Vec<Consumable>,
}

impl ConsumableInventory {
    pub fn add(&mut self, consumable: Consumable) {
        // Try to stack if possible
        if consumable.is_stackable() {
            if let Some(existing) = self
                .items
                .iter_mut()
                .find(|c| c.consumable_type == consumable.consumable_type)
            {
                existing.quantity += consumable.quantity;
                return;
            }
        }
        self.items.push(consumable);
    }

    pub fn use_item(&mut self, index: usize) -> Option<Consumable> {
        if index < self.items.len() {
            self.items[index].quantity -= 1;
            if self.items[index].quantity == 0 {
                Some(self.items.remove(index))
            } else {
                Some(self.items[index].clone())
            }
        } else {
            None
        }
    }

    pub fn get_item(&self, index: usize) -> Option<&Consumable> {
        self.items.get(index)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
