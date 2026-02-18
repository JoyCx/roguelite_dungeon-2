use crate::constants::*;
use crate::model::attack_pattern::AttackPattern;
use crate::model::enemy::Enemy;
use std::time::Instant;

/// Boss enemy types with unique attack patterns and mechanics
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BossType {
    /// Goblin Overlord - Fast attacks with knockback
    GoblinOverlord,
    /// Skeletal Knight - Heavy armor, slow powerful attacks
    SkeletalKnight,
    /// Flame Sorcerer - Ranged attacks with burning effect
    FlameSorcerer,
    /// Shadow Assassin - Quick dashes, high damage
    ShadowAssassin,
    /// Corrupted Warden - Multiple phases, heal over time
    CorruptedWarden,
}

/// Boss phase state for multi-phase mechanics
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BossPhase {
    /// Phase 1: Boss at 66-100% health
    First,
    /// Phase 2: Boss at 33-66% health
    Second,
    /// Phase 3: Boss at 0-33% health (enraged)
    Third,
}

/// Enhanced enemy with boss-specific mechanics
#[derive(Clone, Debug)]
pub struct BossEnemy {
    /// Base enemy stats and position
    pub base_enemy: Enemy,

    /// Boss type determines attack patterns
    pub boss_type: BossType,

    /// Current phase of the fight
    pub current_phase: BossPhase,

    /// Maximum health for phase calculations
    pub max_health_base: i32,

    /// Special ability cooldown
    pub special_ability_cooldown: Option<Instant>,
    pub special_ability_duration: f32,

    /// Phase transition cooldown (prevents spam switching)
    pub phase_transition_cooldown: Option<Instant>,

    /// Boss enrage mechanic - damage multiplier when below 33% health
    pub enrage_multiplier: f32,

    /// Attack patterns for this boss
    pub attack_patterns: Vec<AttackPattern>,

    /// Current attack pattern index
    pub attack_pattern_index: usize,

    /// Loot multiplier (bosses drop more rewards)
    pub loot_multiplier: i32,
}

impl BossEnemy {
    /// Create a new boss enemy at the given position
    pub fn new(x: i32, y: i32, boss_type: BossType) -> Self {
        let (base_health, base_max_health, attack_patterns) = match boss_type {
            BossType::GoblinOverlord => (
                120,
                120,
                vec![
                    AttackPattern::WhirlwindAttack,
                    AttackPattern::BasicSlash,
                    AttackPattern::GroundSlam(2),
                ],
            ),
            BossType::SkeletalKnight => (
                180,
                180,
                vec![
                    AttackPattern::BasicSlash,
                    AttackPattern::SwordThrust(2),
                    AttackPattern::GroundSlam(1),
                ],
            ),
            BossType::FlameSorcerer => (
                100,
                100,
                vec![
                    AttackPattern::Fireball(3),
                    AttackPattern::MeteorShower(4, 2),
                    AttackPattern::Fireball(2),
                ],
            ),
            BossType::ShadowAssassin => (
                110,
                110,
                vec![
                    AttackPattern::SwordThrust(2),
                    AttackPattern::WhirlwindAttack,
                    AttackPattern::BasicSlash,
                ],
            ),
            BossType::CorruptedWarden => (
                200,
                200,
                vec![
                    AttackPattern::ChainLightning(4),
                    AttackPattern::Fireball(3),
                    AttackPattern::FrostNova(3),
                ],
            ),
        };

        let mut base_enemy = Enemy::new(x, y, BOSS_BASE_SPEED * ENEMY_SPEED_MULTIPLIER);
        base_enemy.health = base_health;
        base_enemy.max_health = base_max_health;

        Self {
            base_enemy,
            boss_type,
            current_phase: BossPhase::First,
            max_health_base: base_health,
            special_ability_cooldown: None,
            special_ability_duration: 0.0,
            phase_transition_cooldown: None,
            enrage_multiplier: 1.0,
            attack_patterns,
            attack_pattern_index: 0,
            loot_multiplier: 3,
        }
    }

    /// Calculate current phase based on health percentage
    pub fn update_phase(&mut self) {
        let health_percent = (self.base_enemy.health as f32 / self.max_health_base as f32) * 100.0;

        let new_phase = match health_percent {
            66.0..=100.0 => BossPhase::First,
            33.0..=66.0 => BossPhase::Second,
            _ => BossPhase::Third,
        };

        if new_phase != self.current_phase {
            self.current_phase = new_phase;
            self.phase_transition_cooldown = Some(Instant::now());

            // Enrage multiplier increases in later phases
            self.enrage_multiplier = match self.current_phase {
                BossPhase::First => 1.0,
                BossPhase::Second => 1.2,
                BossPhase::Third => 1.5,
            };
        }
    }

    /// Check if special ability is ready
    pub fn can_use_special_ability(&self, current_time: Instant) -> bool {
        match self.special_ability_cooldown {
            None => true,
            Some(last_time) => {
                let elapsed = current_time.elapsed().as_secs_f32();
                let last_elapsed = current_time.duration_since(last_time).as_secs_f32();
                last_elapsed >= 8.0 // 8 second cooldown for special abilities
            }
        }
    }

    /// Get modified attack damage based on phase and enrage
    pub fn get_effective_damage(&self) -> i32 {
        let phase_multiplier = match self.current_phase {
            BossPhase::First => 1.0,
            BossPhase::Second => 1.1,
            BossPhase::Third => 1.3,
        };

        // Get base damage from boss type
        let base_damage = match self.boss_type {
            BossType::GoblinOverlord => 25,
            BossType::SkeletalKnight => 35,
            BossType::FlameSorcerer => 30,
            BossType::ShadowAssassin => 40,
            BossType::CorruptedWarden => 28,
        };

        ((base_damage as f32 * phase_multiplier * self.enrage_multiplier) as i32).max(1)
    }

    /// Get attack radius based on boss type
    pub fn get_attack_radius(&self) -> i32 {
        match self.boss_type {
            BossType::GoblinOverlord => 2,
            BossType::SkeletalKnight => 3,
            BossType::FlameSorcerer => 4, // ranged
            BossType::ShadowAssassin => 2,
            BossType::CorruptedWarden => 3,
        }
    }

    /// Determine if boss heals (like Corrupted Warden)
    pub fn heals_over_time(&self) -> bool {
        matches!(self.boss_type, BossType::CorruptedWarden)
    }

    /// Get healing amount if applicable
    pub fn get_healing_amount(&self) -> i32 {
        if self.heals_over_time() {
            match self.current_phase {
                BossPhase::First => 1,
                BossPhase::Second => 2,
                BossPhase::Third => 3,
            }
        } else {
            0
        }
    }

    /// Apply healing if boss supports it
    pub fn apply_healing(&mut self) {
        if self.heals_over_time() {
            let heal_amount = self.get_healing_amount();
            self.base_enemy.health =
                (self.base_enemy.health + heal_amount).min(self.max_health_base);
        }
    }

    /// Get experience reward based on boss type and phase
    pub fn get_experience_reward(&self) -> i32 {
        let base_reward = match self.boss_type {
            BossType::GoblinOverlord => 200,
            BossType::SkeletalKnight => 250,
            BossType::FlameSorcerer => 220,
            BossType::ShadowAssassin => 240,
            BossType::CorruptedWarden => 300,
        };
        base_reward + (base_reward / 2) // +50% to base reward
    }

    /// Trigger special ability effect (phase-dependent)
    pub fn trigger_special_ability(&mut self) {
        self.special_ability_cooldown = Some(Instant::now());

        match self.boss_type {
            BossType::GoblinOverlord => {
                // Knockback wave attack - increase speed temporarily
                self.base_enemy.speed = 4.0;
                self.special_ability_duration = 2.0;
            }
            BossType::SkeletalKnight => {
                // Defensive stance - reduce incoming damage temporarily
                self.special_ability_duration = 3.0;
            }
            BossType::FlameSorcerer => {
                // Area of effect fire attack
                self.special_ability_duration = 2.5;
            }
            BossType::ShadowAssassin => {
                // Quick dash attack
                self.base_enemy.speed = 6.0;
                self.special_ability_duration = 2.0;
            }
            BossType::CorruptedWarden => {
                // Summon corruption - boost health
                let heal_amount = (self.max_health_base / 4) as i32;
                self.base_enemy.health =
                    (self.base_enemy.health + heal_amount).min(self.max_health_base);
            }
        }
    }

    /// Reset to normal state (after special ability)
    pub fn reset_special_state(&mut self) {
        self.special_ability_duration = 0.0;
        // Reset speed to normal
        self.base_enemy.speed = 2.5;
    }

    /// Check if currently in special ability state
    pub fn is_in_special_state(&self) -> bool {
        self.special_ability_duration > 0.0
    }

    /// Get the current attack pattern
    pub fn get_current_attack_pattern(&self) -> AttackPattern {
        self.attack_patterns
            .get(self.attack_pattern_index)
            .cloned()
            .unwrap_or_else(|| AttackPattern::BasicSlash)
    }

    /// Rotate to the next attack pattern
    pub fn rotate_attack_pattern(&mut self) {
        self.attack_pattern_index = (self.attack_pattern_index + 1) % self.attack_patterns.len();
    }
}

/// Helper function to convert attack patterns to enemy attacks with reasonable defaults
pub fn convert_attack_patterns_to_enemy_attacks(
    patterns: &[AttackPattern],
    damage_base: i32,
) -> Vec<crate::model::enemy_type::EnemyAttack> {
    use crate::model::enemy_type::{AttackType, EnemyAttack};

    patterns
        .iter()
        .enumerate()
        .map(|(idx, pattern)| {
            let (name, damage_min, damage_max, attack_type, reach, area_radius, cooldown) =
                match pattern {
                    AttackPattern::BasicSlash => (
                        "Basic Slash".to_string(),
                        damage_base,
                        damage_base + 5,
                        AttackType::Physical,
                        1,
                        0,
                        2.0,
                    ),
                    AttackPattern::GroundSlam(reach_val) => (
                        "Ground Slam".to_string(),
                        (damage_base as f32 * 1.2) as i32,
                        (damage_base as f32 * 1.5) as i32,
                        AttackType::Physical,
                        *reach_val,
                        1,
                        3.0,
                    ),
                    AttackPattern::WhirlwindAttack => (
                        "Whirlwind Attack".to_string(),
                        (damage_base as f32 * 0.9) as i32,
                        (damage_base as f32 * 1.3) as i32,
                        AttackType::Physical,
                        2,
                        1,
                        2.5,
                    ),
                    AttackPattern::SwordThrust(reach_val) => (
                        "Sword Thrust".to_string(),
                        (damage_base as f32 * 1.3) as i32,
                        (damage_base as f32 * 1.7) as i32,
                        AttackType::Physical,
                        *reach_val,
                        0,
                        2.5,
                    ),
                    AttackPattern::Fireball(radius) => (
                        "Fireball".to_string(),
                        (damage_base as f32 * 1.4) as i32,
                        (damage_base as f32 * 1.8) as i32,
                        AttackType::Fire,
                        *radius,
                        2,
                        3.0,
                    ),
                    AttackPattern::MeteorShower(reach_val, _width) => (
                        "Meteor Shower".to_string(),
                        (damage_base as f32 * 1.5) as i32,
                        (damage_base as f32 * 2.0) as i32,
                        AttackType::Fire,
                        *reach_val,
                        2,
                        3.5,
                    ),
                    AttackPattern::ChainLightning(reach_val) => (
                        "Chain Lightning".to_string(),
                        (damage_base as f32 * 1.2) as i32,
                        (damage_base as f32 * 1.6) as i32,
                        AttackType::Magic,
                        *reach_val,
                        1,
                        3.0,
                    ),
                    AttackPattern::FrostNova(reach_val) => (
                        "Frost Nova".to_string(),
                        (damage_base as f32 * 1.1) as i32,
                        (damage_base as f32 * 1.4) as i32,
                        AttackType::Magic,
                        *reach_val,
                        1,
                        2.5,
                    ),
                    AttackPattern::ArrowShot(reach_val) => (
                        "Arrow Shot".to_string(),
                        damage_base,
                        damage_base + 4,
                        AttackType::Physical,
                        *reach_val,
                        0,
                        2.0,
                    ),
                    AttackPattern::MultiShot(reach_val, _spread) => (
                        "Multi Shot".to_string(),
                        (damage_base as f32 * 0.8) as i32,
                        (damage_base as f32 * 1.1) as i32,
                        AttackType::Physical,
                        *reach_val,
                        1,
                        2.5,
                    ),
                    AttackPattern::Barrage(reach_val) => (
                        "Barrage".to_string(),
                        (damage_base as f32 * 0.7) as i32,
                        (damage_base as f32 * 1.0) as i32,
                        AttackType::Physical,
                        *reach_val,
                        0,
                        2.0,
                    ),
                    AttackPattern::PiercingShot(reach_val) => (
                        "Piercing Shot".to_string(),
                        (damage_base as f32 * 1.2) as i32,
                        (damage_base as f32 * 1.5) as i32,
                        AttackType::Physical,
                        *reach_val,
                        0,
                        2.5,
                    ),
                    AttackPattern::CrescentSlash => (
                        "Crescent Slash".to_string(),
                        (damage_base as f32 * 1.1) as i32,
                        (damage_base as f32 * 1.4) as i32,
                        AttackType::Physical,
                        2,
                        0,
                        2.5,
                    ),
                    AttackPattern::Vortex(radius) => (
                        "Vortex".to_string(),
                        (damage_base as f32 * 1.3) as i32,
                        (damage_base as f32 * 1.6) as i32,
                        AttackType::Magic,
                        *radius,
                        1,
                        3.0,
                    ),
                };

            EnemyAttack {
                name,
                damage_min,
                damage_max,
                attack_type,
                reach,
                area_radius,
                effect: None,
                cooldown_duration: cooldown,
                cooldown_remaining: 0.0,
                pattern: pattern.clone(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boss_creation() {
        let boss = BossEnemy::new(10, 15, BossType::GoblinOverlord);
        assert_eq!(boss.base_enemy.position.x, 10);
        assert_eq!(boss.base_enemy.position.y, 15);
        assert_eq!(boss.boss_type, BossType::GoblinOverlord);
        assert_eq!(boss.current_phase, BossPhase::First);
    }

    #[test]
    fn test_phase_transitions() {
        let mut boss = BossEnemy::new(10, 15, BossType::SkeletalKnight);
        assert_eq!(boss.current_phase, BossPhase::First);

        // Drop to 50% health (Phase 2)
        boss.base_enemy.health = 90;
        boss.update_phase();
        assert_eq!(boss.current_phase, BossPhase::Second);
        assert_eq!(boss.enrage_multiplier, 1.2);

        // Drop to 20% health (Phase 3)
        boss.base_enemy.health = 36;
        boss.update_phase();
        assert_eq!(boss.current_phase, BossPhase::Third);
        assert_eq!(boss.enrage_multiplier, 1.5);
    }

    #[test]
    fn test_effective_damage_scaling() {
        let mut boss = BossEnemy::new(10, 15, BossType::FlameSorcerer);
        let phase1_damage = boss.get_effective_damage();

        boss.current_phase = BossPhase::Second;
        let phase2_damage = boss.get_effective_damage();
        assert!(phase2_damage > phase1_damage);

        boss.current_phase = BossPhase::Third;
        let phase3_damage = boss.get_effective_damage();
        assert!(phase3_damage > phase2_damage);
    }

    #[test]
    fn test_healing_mechanic() {
        let mut boss = BossEnemy::new(10, 15, BossType::CorruptedWarden);
        let original_health = boss.base_enemy.health;
        boss.base_enemy.health = original_health / 2;

        boss.current_phase = BossPhase::First;
        boss.apply_healing();
        assert!(boss.base_enemy.health > original_health / 2);

        boss.current_phase = BossPhase::Third;
        let health_before = boss.base_enemy.health;
        boss.apply_healing();
        let heal_amount = boss.get_healing_amount();
        assert_eq!(
            boss.base_enemy.health,
            (health_before + heal_amount).min(boss.max_health_base)
        );
    }

    #[test]
    fn test_attack_radius_variation() {
        let goblin = BossEnemy::new(0, 0, BossType::GoblinOverlord);
        let knight = BossEnemy::new(0, 0, BossType::SkeletalKnight);
        let sorcerer = BossEnemy::new(0, 0, BossType::FlameSorcerer);

        assert_eq!(goblin.get_attack_radius(), 2);
        assert_eq!(knight.get_attack_radius(), 3);
        assert_eq!(sorcerer.get_attack_radius(), 4); // ranged
    }

    #[test]
    fn test_experience_reward() {
        let warden = BossEnemy::new(0, 0, BossType::CorruptedWarden);
        let exp = warden.get_experience_reward();
        assert!(exp > 300); // Should be 300 + 50% = 450
    }

    #[test]
    fn test_loot_multiplier() {
        let boss = BossEnemy::new(0, 0, BossType::ShadowAssassin);
        assert_eq!(boss.loot_multiplier, 3); // All bosses drop 3x loot
    }
}
