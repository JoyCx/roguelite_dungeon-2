use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum StatusEffectType {
    Bleed,          // Takes damage per second, stacks
    Poison,         // Takes damage per second, refreshes duration
    Burn,           // Takes damage per second, spreads to adjacent tiles
    Stun,           // Cannot act
    Cripple,        // Reduced movement speed
    Fear,           // Moves away from player
    PoisonImmunity, // Prevents poison application
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatusEffect {
    pub effect_type: StatusEffectType,
    pub duration: f32,
    pub damage_per_sec: f32,
    pub stacks: u32, // For bleed
}

impl StatusEffect {
    pub fn new(effect_type: StatusEffectType, duration: f32) -> Self {
        let damage_per_sec = match effect_type {
            StatusEffectType::Bleed => 1.0,
            StatusEffectType::Poison => 1.0,
            StatusEffectType::Burn => 3.0,
            StatusEffectType::Stun => 0.0,
            StatusEffectType::Cripple => 0.0,
            StatusEffectType::Fear => 0.0,
            StatusEffectType::PoisonImmunity => 0.0,
        };

        Self {
            effect_type,
            duration,
            damage_per_sec,
            stacks: 1,
        }
    }

    pub fn bleed_with_stacks(stacks: u32) -> Self {
        Self {
            effect_type: StatusEffectType::Bleed,
            duration: 8.0,
            damage_per_sec: 1.0,
            stacks,
        }
    }

    pub fn poison(duration: f32) -> Self {
        Self {
            effect_type: StatusEffectType::Poison,
            duration,
            damage_per_sec: 1.0,
            stacks: 1,
        }
    }

    pub fn burn(duration: f32) -> Self {
        Self {
            effect_type: StatusEffectType::Burn,
            duration,
            damage_per_sec: 2.0,
            stacks: 1,
        }
    }

    pub fn stun(duration: f32) -> Self {
        Self {
            effect_type: StatusEffectType::Stun,
            duration,
            damage_per_sec: 0.0,
            stacks: 1,
        }
    }

    pub fn poison_immunity(duration: f32) -> Self {
        Self {
            effect_type: StatusEffectType::PoisonImmunity,
            duration,
            damage_per_sec: 0.0,
            stacks: 1,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct StatusEffectManager {
    pub effects: Vec<StatusEffect>,
}

impl StatusEffectManager {
    pub fn add(&mut self, effect: StatusEffect) {
        // Special handling for certain effects
        match effect.effect_type {
            StatusEffectType::Bleed => {
                // Bleed stacks additively
                if let Some(existing) = self
                    .effects
                    .iter_mut()
                    .find(|e| e.effect_type == StatusEffectType::Bleed)
                {
                    existing.stacks += effect.stacks;
                    existing.duration = 8.0; // Refresh to max duration
                } else {
                    self.effects.push(effect);
                }
            }
            StatusEffectType::Poison => {
                // Poison refreshes duration
                if let Some(existing) = self
                    .effects
                    .iter_mut()
                    .find(|e| e.effect_type == StatusEffectType::Poison)
                {
                    existing.duration = effect.duration;
                } else {
                    self.effects.push(effect);
                }
            }
            StatusEffectType::Burn => {
                // Burn adds new instance
                self.effects.push(effect);
            }
            _ => {
                // Other effects just add
                self.effects.push(effect);
            }
        }
    }

    pub fn remove_type(&mut self, effect_type: &StatusEffectType) {
        self.effects.retain(|e| e.effect_type != *effect_type);
    }

    pub fn has_effect(&self, effect_type: &StatusEffectType) -> bool {
        self.effects.iter().any(|e| e.effect_type == *effect_type)
    }

    pub fn get_total_damage_per_sec(&self) -> f32 {
        self.effects
            .iter()
            .map(|e| e.damage_per_sec * e.stacks as f32)
            .sum()
    }

    pub fn update(&mut self, delta: f32) {
        for effect in &mut self.effects {
            effect.duration -= delta;
        }
        self.effects.retain(|e| e.duration > 0.0);
    }
}
