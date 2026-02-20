use crate::model::attack_pattern::AttackPattern;
use crate::model::item_tier::Difficulty;
use ratatui::prelude::{Color, Modifier, Style};
use serde::{Deserialize, Serialize};

/// Enemy rarity determines number of attacks, ultimate availability, and buff spells
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EnemyRarity {
    Fighter,  // 1 attack, no ultimate, no buffs
    Guard,    // 1 attack, no ultimate, no buffs
    Champion, // 2 attacks, weak ultimate, 1-2 buffs
    Elite,    // 3 attacks, average ultimate, 2-3 buffs
    Boss,     // 5 attacks, devastating ultimate, multiple buffs
}

impl EnemyRarity {
    pub fn num_attacks(&self) -> usize {
        match self {
            EnemyRarity::Fighter | EnemyRarity::Guard => 1,
            EnemyRarity::Champion => 2,
            EnemyRarity::Elite => 3,
            EnemyRarity::Boss => 5,
        }
    }

    pub fn has_ultimate(&self) -> bool {
        matches!(
            self,
            EnemyRarity::Champion | EnemyRarity::Elite | EnemyRarity::Boss
        )
    }

    pub fn num_buffs(&self) -> usize {
        match self {
            EnemyRarity::Fighter | EnemyRarity::Guard => 0,
            EnemyRarity::Champion => 1,
            EnemyRarity::Elite => 2,
            EnemyRarity::Boss => 3,
        }
    }

    pub fn base_gold(&self) -> u32 {
        match self {
            EnemyRarity::Fighter => 10,
            EnemyRarity::Guard => 15,
            EnemyRarity::Champion => 25,
            EnemyRarity::Elite => 50,
            EnemyRarity::Boss => 150,
        }
    }

    pub fn calculate_gold_drop(&self, difficulty: &Difficulty) -> u32 {
        let base = self.base_gold();
        let multiplier = match difficulty {
            Difficulty::Easy => 1.0,
            Difficulty::Normal => 1.5,
            Difficulty::Hard => 2.0,
            Difficulty::Death => 3.0,
        };
        (base as f32 * multiplier).ceil() as u32
    }

    /// Get base detection radius for this enemy tier (in tiles)
    /// Used to determine when enemy starts chasing the player
    pub fn base_detection_radius(&self) -> i32 {
        match self {
            EnemyRarity::Fighter => 5, // Weakest enemies have smallest radius
            EnemyRarity::Guard => 6,
            EnemyRarity::Champion => 8,
            EnemyRarity::Elite => 10,
            EnemyRarity::Boss => 15, // Bosses can detect player from far away
        }
    }

    /// Calculate final detection radius based on difficulty
    pub fn calculate_detection_radius(&self, difficulty: &Difficulty) -> i32 {
        let base = self.base_detection_radius() as f32;
        let multiplier = difficulty.detection_radius_multiplier();
        (base * multiplier).ceil() as i32
    }

    /// Get unique ASCII art representation for this enemy type
    pub fn get_glyph(&self) -> &'static str {
        match self {
            EnemyRarity::Fighter => "x",
            EnemyRarity::Guard => "⛊ ",
            EnemyRarity::Champion => "◆",
            EnemyRarity::Elite => "☠",
            EnemyRarity::Boss => "♛ ",
        }
    }

    /// Get color for this enemy rarity (using "Blood & Shadow" palette)
    pub fn get_color(&self) -> Color {
        match self {
            EnemyRarity::Fighter => Color::Red, // Standard red aggression
            EnemyRarity::Guard => Color::Rgb(200, 50, 50), // Deep crimson - feels armored
            EnemyRarity::Champion => Color::Rgb(255, 0, 150), // Vivid magenta - mechanical step up
            EnemyRarity::Elite => Color::Rgb(140, 0, 255), // Deep sinister purple - shadow magic
            EnemyRarity::Boss => Color::Rgb(0, 255, 100), // Neon acid green - calamity/contrast
        }
    }

    /// Get text style for this enemy rarity with modifier for threat indication
    pub fn get_style(&self) -> Style {
        let base = Style::default()
            .fg(self.get_color())
            .add_modifier(Modifier::BOLD);

        match self {
            EnemyRarity::Elite | EnemyRarity::Boss => {
                // Elites and Bosses get rapid blink to indicate high threat
                base.add_modifier(Modifier::RAPID_BLINK)
            }
            _ => base,
        }
    }
}

/// Whether enemy is undead (physical, fire weak) or ghost (ethereal, magic weak)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EnemyType {
    Undead,
    Ghost,
}

impl EnemyType {
    /// Ghosts take reduced physical damage and no wall collision
    pub fn damage_multiplier(&self, damage_type: &AttackType) -> f32 {
        match self {
            EnemyType::Undead => {
                match damage_type {
                    AttackType::Physical => 1.0,
                    AttackType::Fire => 1.25,
                    AttackType::Holy => 1.25,
                    AttackType::Poison => 0.5, // undead resist
                    AttackType::Magic => 1.0,
                }
            }
            EnemyType::Ghost => match damage_type {
                AttackType::Physical => 0.7,
                AttackType::Magic => 1.25,
                AttackType::Holy => 1.25,
                _ => 0.9,
            },
        }
    }

    pub fn passes_through_walls(&self) -> bool {
        matches!(self, EnemyType::Ghost)
    }
}

/// Attack type determines damage and effects
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AttackType {
    Physical,
    Fire,
    Holy,
    Poison,
    Magic,
}

/// Each enemy can have different attacks
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnemyAttack {
    pub name: String,
    pub damage_min: i32,
    pub damage_max: i32,
    pub attack_type: AttackType,
    pub reach: i32,       // 1 = adjacent, 2 = 2 tiles, etc
    pub area_radius: i32, // 0 = single target, 1+ = AOE
    pub effect: Option<EnemyEffect>,
    pub cooldown_duration: f32,  // seconds between uses
    pub cooldown_remaining: f32, // current cooldown timer
    pub pattern: AttackPattern,  // visual attack pattern
}

impl EnemyAttack {
    pub fn damage(&self) -> i32 {
        let diff = self.damage_max - self.damage_min;
        self.damage_min + (rand::random::<i32>() % (diff + 1).max(1))
    }

    pub fn is_available(&self) -> bool {
        self.cooldown_remaining <= 0.0
    }

    pub fn use_attack(&mut self) {
        self.cooldown_remaining = self.cooldown_duration;
    }

    pub fn update_cooldown(&mut self, delta_time: f32) {
        self.cooldown_remaining = (self.cooldown_remaining - delta_time).max(0.0);
    }

    pub fn cooldown_percent(&self) -> f32 {
        if self.cooldown_duration <= 0.0 {
            return 0.0;
        }
        (self.cooldown_remaining / self.cooldown_duration).clamp(0.0, 1.0)
    }
}

/// Effects that attacks can apply
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EnemyEffect {
    Slow(f32),        // duration in seconds
    Poison(i32, f32), // damage per second, duration
    Stun(f32),        // duration
}

/// Buff spells that enemies can cast on themselves
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum EnemyBuff {
    Armor(i32),        // flat damage reduction
    Sharpness(i32),    // bonus damage
    Speed(i32),        // % movement speed bonus
    Regeneration(i32), // HP per second
    BloodFrenzy,       // extra damage when below 50% HP
    PhaseShift,        // 20% chance to ignore damage (GHOSTS ONLY)
    EchoAmplification, // bonus to magic damage (GHOSTS ONLY)
}

impl EnemyBuff {
    pub fn name(&self) -> &str {
        match self {
            EnemyBuff::Armor(_) => "Bone Plating",
            EnemyBuff::Sharpness(_) => "Bloodlust",
            EnemyBuff::Speed(_) => "Haste",
            EnemyBuff::Regeneration(_) => "Regeneration",
            EnemyBuff::BloodFrenzy => "Blood Frenzy",
            EnemyBuff::PhaseShift => "Phase Stability",
            EnemyBuff::EchoAmplification => "Echo Amplification",
        }
    }

    pub fn description(&self) -> String {
        match self {
            EnemyBuff::Armor(val) => format!("Takes {}% less damage", val),
            EnemyBuff::Sharpness(val) => format!("Deals {}% more damage", val),
            EnemyBuff::Speed(val) => format!("Moves {}% faster", val),
            EnemyBuff::Regeneration(val) => format!("Heals {} HP per second", val),
            EnemyBuff::BloodFrenzy => "Gains +50% damage below 50% HP".to_string(),
            EnemyBuff::PhaseShift => "20% chance to ignore damage".to_string(),
            EnemyBuff::EchoAmplification => "Magic damage amplified".to_string(),
        }
    }
}

/// Ultimate abilities for Champion+ enemies
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnemyUltimate {
    pub name: String,
    pub description: String,
    pub power_level: UltimatePower,
    pub damage_base: i32,
    pub area_radius: i32,
    pub cooldown_duration: f32,  // seconds between uses
    pub cooldown_remaining: f32, // current cooldown timer
    pub time_active: f32,
    pub pattern: AttackPattern, // visual attack pattern
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum UltimatePower {
    Weak,        // Champion
    Average,     // Elite
    Devastating, // Boss
}

impl UltimatePower {
    pub fn damage_mult(&self) -> f32 {
        match self {
            UltimatePower::Weak => 1.0,
            UltimatePower::Average => 1.5,
            UltimatePower::Devastating => 2.5,
        }
    }
}

impl EnemyUltimate {
    pub fn is_available(&self) -> bool {
        self.cooldown_remaining <= 0.0
    }

    pub fn use_ultimate(&mut self) {
        self.cooldown_remaining = self.cooldown_duration;
    }

    pub fn update_cooldown(&mut self, delta_time: f32) {
        self.cooldown_remaining = (self.cooldown_remaining - delta_time).max(0.0);
    }

    pub fn cooldown_percent(&self) -> f32 {
        if self.cooldown_duration <= 0.0 {
            return 0.0;
        }
        (self.cooldown_remaining / self.cooldown_duration).clamp(0.0, 1.0)
    }
}

pub fn create_rotting_footsoldier() -> EnemyTemplate {
    EnemyTemplate {
        name: "Rotting Footsoldier".to_string(),
        description: "A former guard, armor fused to decayed flesh.".to_string(),
        rarity: EnemyRarity::Fighter,
        enemy_type: EnemyType::Undead,
        health: 30,
        speed: 0.15,
        attacks: vec![EnemyAttack {
            name: "Rusted Slash".to_string(),
            damage_min: 2,
            damage_max: 4,
            attack_type: AttackType::Physical,
            reach: 1,
            area_radius: 0,
            effect: None,
            cooldown_duration: 2.0,
            cooldown_remaining: 0.0,
            pattern: AttackPattern::BasicSlash,
        }],
        ultimate: None,
        buffs: vec![],
    }
}

pub fn create_grave_scrabbler() -> EnemyTemplate {
    EnemyTemplate {
        name: "Grave Scrabbler".to_string(),
        description: "A crawling corpse that drags itself with clawed fingers.".to_string(),
        rarity: EnemyRarity::Fighter,
        enemy_type: EnemyType::Undead,
        health: 24,
        speed: 0.12,
        attacks: vec![EnemyAttack {
            name: "Bone Rake".to_string(),
            damage_min: 2,
            damage_max: 4,
            attack_type: AttackType::Physical,
            reach: 1,
            area_radius: 1,
            effect: None,
            cooldown_duration: 1.8,
            cooldown_remaining: 0.0,
            pattern: AttackPattern::WhirlwindAttack,
        }],
        ultimate: None,
        buffs: vec![],
    }
}

pub fn create_whispering_shade() -> EnemyTemplate {
    EnemyTemplate {
        name: "Whispering Shade".to_string(),
        description: "A thin silhouette of regret and rage.".to_string(),
        rarity: EnemyRarity::Fighter,
        enemy_type: EnemyType::Ghost,
        health: 20,
        speed: 0.18,
        attacks: vec![EnemyAttack {
            name: "Chill Touch".to_string(),
            damage_min: 2,
            damage_max: 3,
            attack_type: AttackType::Magic,
            reach: 1,
            area_radius: 0,
            effect: Some(EnemyEffect::Slow(1.0)),
            cooldown_duration: 2.2,
            cooldown_remaining: 0.0,
            pattern: AttackPattern::ArrowShot(3),
        }],
        ultimate: None,
        buffs: vec![],
    }
}

pub fn create_crypt_sentinel() -> EnemyTemplate {
    EnemyTemplate {
        name: "Crypt Sentinel".to_string(),
        description: "An animated suit of armor bound by runes.".to_string(),
        rarity: EnemyRarity::Guard,
        enemy_type: EnemyType::Undead,
        health: 40,
        speed: 0.1,
        attacks: vec![EnemyAttack {
            name: "Shield Bash".to_string(),
            damage_min: 3,
            damage_max: 5,
            attack_type: AttackType::Physical,
            reach: 1,
            area_radius: 0,
            effect: None,
            cooldown_duration: 2.5,
            cooldown_remaining: 0.0,
            pattern: AttackPattern::ArrowShot(3),
        }],
        ultimate: None,
        buffs: vec![EnemyBuff::Armor(30)],
    }
}

pub fn create_tomb_watcher() -> EnemyTemplate {
    EnemyTemplate {
        name: "Tomb Watcher".to_string(),
        description: "A skeletal knight bound to protect the dead.".to_string(),
        rarity: EnemyRarity::Guard,
        enemy_type: EnemyType::Undead,
        health: 36,
        speed: 0.12,
        attacks: vec![EnemyAttack {
            name: "Longspear Thrust".to_string(),
            damage_min: 3,
            damage_max: 5,
            attack_type: AttackType::Physical,
            reach: 2,
            area_radius: 0,
            effect: None,
            cooldown_duration: 2.3,
            cooldown_remaining: 0.0,
            pattern: AttackPattern::SwordThrust(2),
        }],
        ultimate: None,
        buffs: vec![],
    }
}

pub fn create_wailing_doorwarden() -> EnemyTemplate {
    EnemyTemplate {
        name: "Wailing Doorwarden".to_string(),
        description: "An incorporeal spirit fused to the dungeon walls.".to_string(),
        rarity: EnemyRarity::Guard,
        enemy_type: EnemyType::Ghost,
        health: 28,
        speed: 0.14,
        attacks: vec![EnemyAttack {
            name: "Sonic Screech".to_string(),
            damage_min: 2,
            damage_max: 4,
            attack_type: AttackType::Magic,
            reach: 0,
            area_radius: 2,
            effect: None,
            cooldown_duration: 2.8,
            cooldown_remaining: 0.0,
            pattern: AttackPattern::Fireball(2),
        }],
        ultimate: None,
        buffs: vec![],
    }
}

pub fn create_blight_captain() -> EnemyTemplate {
    EnemyTemplate {
        name: "Blight Captain".to_string(),
        description: "A commanding corpse still barking silent orders.".to_string(),
        rarity: EnemyRarity::Champion,
        enemy_type: EnemyType::Undead,
        health: 70,
        speed: 0.13,
        attacks: vec![
            EnemyAttack {
                name: "Cleaving Strike".to_string(),
                damage_min: 4,
                damage_max: 7,
                attack_type: AttackType::Physical,
                reach: 1,
                area_radius: 1,
                effect: None,
                cooldown_duration: 2.0,
                cooldown_remaining: 0.0,
                pattern: AttackPattern::FrostNova(1),
            },
            EnemyAttack {
                name: "Commanding Roar".to_string(),
                damage_min: 2,
                damage_max: 4,
                attack_type: AttackType::Physical,
                reach: 0,
                area_radius: 3,
                effect: None,
                cooldown_duration: 3.5,
                cooldown_remaining: 0.0,
                pattern: AttackPattern::Fireball(3),
            },
        ],
        ultimate: Some(EnemyUltimate {
            name: "Blight Surge".to_string(),
            description: "Emits rot pulse in 3x3 area".to_string(),
            power_level: UltimatePower::Weak,
            damage_base: 6,
            area_radius: 3,
            cooldown_duration: 20.0,
            cooldown_remaining: 0.0,
            time_active: 0.0,
            pattern: AttackPattern::Fireball(3),
        }),
        buffs: vec![EnemyBuff::Armor(15), EnemyBuff::Sharpness(10)],
    }
}

pub fn create_veilbound_duelist() -> EnemyTemplate {
    EnemyTemplate {
        name: "Veilbound Duelist".to_string(),
        description: "A spectral swordsman frozen mid-duel.".to_string(),
        rarity: EnemyRarity::Champion,
        enemy_type: EnemyType::Ghost,
        health: 60,
        speed: 0.16,
        attacks: vec![
            EnemyAttack {
                name: "Phantasmal Lunge".to_string(),
                damage_min: 4,
                damage_max: 6,
                attack_type: AttackType::Magic,
                reach: 2,
                area_radius: 0,
                effect: None,
                cooldown_duration: 2.2,
                cooldown_remaining: 0.0,
                pattern: AttackPattern::SwordThrust(2),
            },
            EnemyAttack {
                name: "Fade Step".to_string(),
                damage_min: 2,
                damage_max: 4,
                attack_type: AttackType::Magic,
                reach: 0,
                area_radius: 1,
                effect: Some(EnemyEffect::Slow(1.5)),
                cooldown_duration: 2.8,
                cooldown_remaining: 0.0,
                pattern: AttackPattern::Fireball(1),
            },
        ],
        ultimate: Some(EnemyUltimate {
            name: "Echo Slash".to_string(),
            description: "Repeats last attack at 50% power".to_string(),
            power_level: UltimatePower::Weak,
            damage_base: 5,
            area_radius: 1,
            cooldown_duration: 18.0,
            cooldown_remaining: 0.0,
            time_active: 0.0,
            pattern: AttackPattern::BasicSlash,
        }),
        buffs: vec![EnemyBuff::Speed(20), EnemyBuff::PhaseShift],
    }
}

pub fn create_corpse_abomination() -> EnemyTemplate {
    EnemyTemplate {
        name: "Corpse Abomination".to_string(),
        description: "A stitched mass of limbs and screaming mouths.".to_string(),
        rarity: EnemyRarity::Elite,
        enemy_type: EnemyType::Undead,
        health: 55,
        speed: 0.11,
        attacks: vec![
            EnemyAttack {
                name: "Slam Fist".to_string(),
                damage_min: 12,
                damage_max: 16,
                attack_type: AttackType::Physical,
                reach: 1,
                area_radius: 0,
                effect: None,
                cooldown_duration: 2.5,
                cooldown_remaining: 0.0,
                pattern: AttackPattern::BasicSlash,
            },
            EnemyAttack {
                name: "Grasping Limbs".to_string(),
                damage_min: 8,
                damage_max: 12,
                attack_type: AttackType::Physical,
                reach: 0,
                area_radius: 2,
                effect: Some(EnemyEffect::Slow(2.0)),
                cooldown_duration: 3.0,
                cooldown_remaining: 0.0,
                pattern: AttackPattern::Fireball(2),
            },
            EnemyAttack {
                name: "Flesh Whip".to_string(),
                damage_min: 10,
                damage_max: 14,
                attack_type: AttackType::Physical,
                reach: 2,
                area_radius: 0,
                effect: None,
                cooldown_duration: 2.8,
                cooldown_remaining: 0.0,
                pattern: AttackPattern::SwordThrust(2),
            },
        ],
        ultimate: Some(EnemyUltimate {
            name: "Harvest of Limbs".to_string(),
            description: "Slams all adjacent tiles twice".to_string(),
            power_level: UltimatePower::Average,
            damage_base: 14,
            area_radius: 2,
            cooldown_duration: 25.0,
            cooldown_remaining: 0.0,
            time_active: 0.0,
            pattern: AttackPattern::Fireball(2),
        }),
        buffs: vec![EnemyBuff::Regeneration(2), EnemyBuff::Armor(20)],
    }
}

pub fn create_lantern_haunt() -> EnemyTemplate {
    EnemyTemplate {
        name: "Lantern Haunt".to_string(),
        description: "A floating lantern possessed by a trapped soul.".to_string(),
        rarity: EnemyRarity::Elite,
        enemy_type: EnemyType::Ghost,
        health: 45,
        speed: 0.14,
        attacks: vec![
            EnemyAttack {
                name: "Soul Beam".to_string(),
                damage_min: 10,
                damage_max: 14,
                attack_type: AttackType::Magic,
                reach: 3,
                area_radius: 0,
                effect: None,
                cooldown_duration: 2.8,
                cooldown_remaining: 0.0,
                pattern: AttackPattern::Barrage(3),
            },
            EnemyAttack {
                name: "Flicker Warp".to_string(),
                damage_min: 7,
                damage_max: 10,
                attack_type: AttackType::Magic,
                reach: 0,
                area_radius: 1,
                effect: None,
                cooldown_duration: 2.2,
                cooldown_remaining: 0.0,
                pattern: AttackPattern::Fireball(1),
            },
            EnemyAttack {
                name: "Dread Glow".to_string(),
                damage_min: 5,
                damage_max: 8,
                attack_type: AttackType::Magic,
                reach: 0,
                area_radius: 2,
                effect: None,
                cooldown_duration: 3.0,
                cooldown_remaining: 0.0,
                pattern: AttackPattern::Fireball(2),
            },
        ],
        ultimate: Some(EnemyUltimate {
            name: "Blackout".to_string(),
            description: "Darkens area, reducing player accuracy".to_string(),
            power_level: UltimatePower::Average,
            damage_base: 0,
            area_radius: 5,
            cooldown_duration: 22.0,
            cooldown_remaining: 0.0,
            time_active: 0.0,
            pattern: AttackPattern::MeteorShower(5, 2),
        }),
        buffs: vec![EnemyBuff::EchoAmplification, EnemyBuff::Speed(15)],
    }
}

pub fn create_ossuary_king() -> EnemyTemplate {
    EnemyTemplate {
        name: "The Ossuary King".to_string(),
        description: "A towering monarch of bones, crowned in skulls.".to_string(),
        rarity: EnemyRarity::Boss,
        enemy_type: EnemyType::Undead,
        health: 120,
        speed: 0.12,
        attacks: vec![
            EnemyAttack {
                name: "Bone Cleave".to_string(),
                damage_min: 18,
                damage_max: 24,
                attack_type: AttackType::Physical,
                reach: 1,
                area_radius: 2,
                effect: None,
                cooldown_duration: 3.0,
                cooldown_remaining: 0.0,
                pattern: AttackPattern::FrostNova(2),
            },
            EnemyAttack {
                name: "Skull Throw".to_string(),
                damage_min: 15,
                damage_max: 20,
                attack_type: AttackType::Physical,
                reach: 3,
                area_radius: 1,
                effect: None,
                cooldown_duration: 3.2,
                cooldown_remaining: 0.0,
                pattern: AttackPattern::Barrage(3),
            },
            EnemyAttack {
                name: "Bone Spikes".to_string(),
                damage_min: 12,
                damage_max: 18,
                attack_type: AttackType::Physical,
                reach: 0,
                area_radius: 3,
                effect: Some(EnemyEffect::Poison(3, 2.0)),
                cooldown_duration: 3.5,
                cooldown_remaining: 0.0,
                pattern: AttackPattern::Fireball(3),
            },
            EnemyAttack {
                name: "March of Dead".to_string(),
                damage_min: 10,
                damage_max: 14,
                attack_type: AttackType::Physical,
                reach: 0,
                area_radius: 4,
                effect: None,
                cooldown_duration: 3.3,
                cooldown_remaining: 0.0,
                pattern: AttackPattern::Vortex(4),
            },
            EnemyAttack {
                name: "Royal Stomp".to_string(),
                damage_min: 20,
                damage_max: 28,
                attack_type: AttackType::Physical,
                reach: 0,
                area_radius: 3,
                effect: None,
                cooldown_duration: 3.8,
                cooldown_remaining: 0.0,
                pattern: AttackPattern::Fireball(3),
            },
        ],
        ultimate: Some(EnemyUltimate {
            name: "Kingdom of Bone".to_string(),
            description: "Room fills with rising spikes".to_string(),
            power_level: UltimatePower::Devastating,
            damage_base: 25,
            area_radius: 6,
            cooldown_duration: 35.0,
            cooldown_remaining: 0.0,
            time_active: 0.0,
            pattern: AttackPattern::Vortex(6),
        }),
        buffs: vec![
            EnemyBuff::Armor(25),
            EnemyBuff::Regeneration(3),
            EnemyBuff::Sharpness(20),
        ],
    }
}

pub fn create_mourning_bell() -> EnemyTemplate {
    EnemyTemplate {
        name: "The Mourning Bell".to_string(),
        description: "A massive spectral bell that tolls doom.".to_string(),
        rarity: EnemyRarity::Boss,
        enemy_type: EnemyType::Ghost,
        health: 100,
        speed: 0.13,
        attacks: vec![
            EnemyAttack {
                name: "Toll Strike".to_string(),
                damage_min: 16,
                damage_max: 22,
                attack_type: AttackType::Magic,
                reach: 0,
                area_radius: 4,
                effect: None,
                cooldown_duration: 3.2,
                cooldown_remaining: 0.0,
                pattern: AttackPattern::Fireball(4),
            },
            EnemyAttack {
                name: "Chain Wail".to_string(),
                damage_min: 14,
                damage_max: 18,
                attack_type: AttackType::Magic,
                reach: 2,
                area_radius: 2,
                effect: Some(EnemyEffect::Stun(1.0)),
                cooldown_duration: 3.0,
                cooldown_remaining: 0.0,
                pattern: AttackPattern::FrostNova(2),
            },
            EnemyAttack {
                name: "Possession".to_string(),
                damage_min: 8,
                damage_max: 12,
                attack_type: AttackType::Magic,
                reach: 3,
                area_radius: 0,
                effect: Some(EnemyEffect::Stun(2.0)),
                cooldown_duration: 3.5,
                cooldown_remaining: 0.0,
                pattern: AttackPattern::Barrage(3),
            },
            EnemyAttack {
                name: "Phasing Drift".to_string(),
                damage_min: 10,
                damage_max: 15,
                attack_type: AttackType::Magic,
                reach: 0,
                area_radius: 2,
                effect: None,
                cooldown_duration: 2.8,
                cooldown_remaining: 0.0,
                pattern: AttackPattern::Fireball(2),
            },
            EnemyAttack {
                name: "Dirge Field".to_string(),
                damage_min: 12,
                damage_max: 16,
                attack_type: AttackType::Magic,
                reach: 0,
                area_radius: 3,
                effect: Some(EnemyEffect::Slow(3.0)),
                cooldown_duration: 3.3,
                cooldown_remaining: 0.0,
                pattern: AttackPattern::Fireball(3),
            },
        ],
        ultimate: Some(EnemyUltimate {
            name: "Final Toll".to_string(),
            description: "Bell rings 3 times, each removes chunk of max HP".to_string(),
            power_level: UltimatePower::Devastating,
            damage_base: 20,
            area_radius: 5,
            cooldown_duration: 32.0,
            cooldown_remaining: 0.0,
            time_active: 0.0,
            pattern: AttackPattern::MeteorShower(5, 2),
        }),
        buffs: vec![
            EnemyBuff::EchoAmplification,
            EnemyBuff::Speed(25),
            EnemyBuff::PhaseShift,
        ],
    }
}

/// Template for creating enemy instances
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnemyTemplate {
    pub name: String,
    pub description: String,
    pub rarity: EnemyRarity,
    pub enemy_type: EnemyType,
    pub health: i32,
    pub speed: f32,
    pub attacks: Vec<EnemyAttack>,
    pub ultimate: Option<EnemyUltimate>,
    pub buffs: Vec<EnemyBuff>,
}

/// Helper to get all available enemy templates scaled by difficulty
pub fn get_enemies_for_difficulty(difficulty: &Difficulty) -> Vec<EnemyTemplate> {
    match difficulty {
        Difficulty::Easy => vec![
            create_rotting_footsoldier(),
            create_grave_scrabbler(),
            create_whispering_shade(),
        ],
        Difficulty::Normal => vec![
            create_rotting_footsoldier(),
            create_grave_scrabbler(),
            create_whispering_shade(),
            create_crypt_sentinel(),
            create_tomb_watcher(),
            create_wailing_doorwarden(),
        ],
        Difficulty::Hard => vec![
            create_crypt_sentinel(),
            create_tomb_watcher(),
            create_wailing_doorwarden(),
            create_blight_captain(),
            create_veilbound_duelist(),
            create_corpse_abomination(),
            create_lantern_haunt(),
        ],
        Difficulty::Death => vec![
            create_blight_captain(),
            create_veilbound_duelist(),
            create_corpse_abomination(),
            create_lantern_haunt(),
            create_ossuary_king(),
            create_mourning_bell(),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enemy_gold_base_values() {
        assert_eq!(EnemyRarity::Fighter.base_gold(), 10);
        assert_eq!(EnemyRarity::Guard.base_gold(), 15);
        assert_eq!(EnemyRarity::Champion.base_gold(), 25);
        assert_eq!(EnemyRarity::Elite.base_gold(), 50);
        assert_eq!(EnemyRarity::Boss.base_gold(), 150);
    }

    #[test]
    fn test_gold_drop_easy_difficulty() {
        let difficulty = Difficulty::Easy;
        assert_eq!(EnemyRarity::Fighter.calculate_gold_drop(&difficulty), 10);
        assert_eq!(EnemyRarity::Guard.calculate_gold_drop(&difficulty), 15);
        assert_eq!(EnemyRarity::Champion.calculate_gold_drop(&difficulty), 25);
        assert_eq!(EnemyRarity::Elite.calculate_gold_drop(&difficulty), 50);
        assert_eq!(EnemyRarity::Boss.calculate_gold_drop(&difficulty), 150);
    }

    #[test]
    fn test_gold_drop_normal_difficulty() {
        let difficulty = Difficulty::Normal;
        assert_eq!(EnemyRarity::Fighter.calculate_gold_drop(&difficulty), 15);
        assert_eq!(EnemyRarity::Guard.calculate_gold_drop(&difficulty), 23);
        assert_eq!(EnemyRarity::Champion.calculate_gold_drop(&difficulty), 38);
        assert_eq!(EnemyRarity::Elite.calculate_gold_drop(&difficulty), 75);
        assert_eq!(EnemyRarity::Boss.calculate_gold_drop(&difficulty), 225);
    }

    #[test]
    fn test_gold_drop_hard_difficulty() {
        let difficulty = Difficulty::Hard;
        assert_eq!(EnemyRarity::Fighter.calculate_gold_drop(&difficulty), 20);
        assert_eq!(EnemyRarity::Guard.calculate_gold_drop(&difficulty), 30);
        assert_eq!(EnemyRarity::Champion.calculate_gold_drop(&difficulty), 50);
        assert_eq!(EnemyRarity::Elite.calculate_gold_drop(&difficulty), 100);
        assert_eq!(EnemyRarity::Boss.calculate_gold_drop(&difficulty), 300);
    }

    #[test]
    fn test_gold_drop_death_difficulty() {
        let difficulty = Difficulty::Death;
        assert_eq!(EnemyRarity::Fighter.calculate_gold_drop(&difficulty), 30);
        assert_eq!(EnemyRarity::Guard.calculate_gold_drop(&difficulty), 45);
        assert_eq!(EnemyRarity::Champion.calculate_gold_drop(&difficulty), 75);
        assert_eq!(EnemyRarity::Elite.calculate_gold_drop(&difficulty), 150);
        assert_eq!(EnemyRarity::Boss.calculate_gold_drop(&difficulty), 450);
    }

    #[test]
    fn test_enemy_health_values() {
        // Test that health values are set properly
        let fighter = create_rotting_footsoldier();
        assert_eq!(fighter.health, 15);

        let guard = create_crypt_sentinel();
        assert_eq!(guard.health, 20);

        let champion = create_blight_captain();
        assert_eq!(champion.health, 35);

        let elite = create_corpse_abomination();
        assert_eq!(elite.health, 55);

        let boss = create_ossuary_king();
        assert_eq!(boss.health, 120);
    }
}
