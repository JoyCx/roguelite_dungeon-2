use crate::model::enemy_type::{AttackType, EnemyBuff, EnemyType};
use std::sync::mpsc;
use std::thread;

/// Offloads heavy damage calculations to background threads
pub struct DamageCalculator;

/// Request for damage calculation
#[derive(Clone, Debug)]
pub struct DamageRequest {
    pub base_damage: i32,
    pub attack_type: AttackType,
    pub target_type: EnemyType,
    pub attacker_buffs: Vec<EnemyBuff>,
    pub target_buffs: Vec<EnemyBuff>,
    pub critical_chance: f32,
}

/// Result of damage calculation
#[derive(Clone, Debug)]
pub struct DamageResult {
    pub final_damage: i32,
    pub is_critical: bool,
    pub damage_breakdown: String,
}

impl DamageCalculator {
    /// Asynchronously calculate damage on a background thread
    /// Returns a receiver to get the result when ready
    pub fn calculate_async(request: DamageRequest) -> mpsc::Receiver<DamageResult> {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let result = Self::calculate_damage(&request);
            let _ = tx.send(result);
        });

        rx
    }

    /// Synchronously calculate damage (use sparingly)
    pub fn calculate_sync(request: &DamageRequest) -> DamageResult {
        Self::calculate_damage(request)
    }

    /// Internal damage calculation logic
    fn calculate_damage(request: &DamageRequest) -> DamageResult {
        let mut damage = request.base_damage as f32;

        // Apply attacker buffs
        for buff in &request.attacker_buffs {
            match buff {
                EnemyBuff::Sharpness(bonus) => {
                    damage *= 1.0 + (*bonus as f32 / 100.0);
                }
                EnemyBuff::BloodFrenzy => {
                    damage *= 1.5; // 50% boost (assume low HP)
                }
                _ => {}
            }
        }

        // Apply type advantage
        let type_mult = request.target_type.damage_multiplier(&request.attack_type);
        damage *= type_mult;

        // Apply target armor
        let mut armor_reduction = 0.0;
        for buff in &request.target_buffs {
            if let EnemyBuff::Armor(val) = buff {
                armor_reduction += *val as f32;
            }
        }
        armor_reduction = armor_reduction.clamp(0.0, 75.0); // Cap at 75% reduction
        damage *= 1.0 - (armor_reduction / 100.0);

        // Critical strike
        let is_critical = rand::random::<f32>() < request.critical_chance;
        if is_critical {
            damage *= 1.5; // 50% crit multiplier
        }

        let final_damage = damage.ceil() as i32;

        let breakdown = format!(
            "Base: {} | Type: {:.1}x | Armor: -{:.0}% | {}{}",
            request.base_damage,
            type_mult,
            armor_reduction,
            if is_critical { "CRITICAL! " } else { "" },
            final_damage
        );

        DamageResult {
            final_damage: final_damage.max(1), // Minimum 1 damage
            is_critical,
            damage_breakdown: breakdown,
        }
    }
}

/// Batch damage calculation for multiple targets
pub fn calculate_batch_damage_async(
    requests: Vec<DamageRequest>,
) -> Vec<mpsc::Receiver<DamageResult>> {
    requests
        .into_iter()
        .map(DamageCalculator::calculate_async)
        .collect()
}

/// Collect results from async calculations with timeout
pub fn collect_damage_results(
    receivers: Vec<mpsc::Receiver<DamageResult>>,
    timeout_ms: u64,
) -> Vec<DamageResult> {
    use std::time::Duration;

    let timeout = Duration::from_millis(timeout_ms);

    receivers
        .into_iter()
        .filter_map(|rx| rx.recv_timeout(timeout).ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_damage() {
        let request = DamageRequest {
            base_damage: 10,
            attack_type: AttackType::Physical,
            target_type: EnemyType::Undead,
            attacker_buffs: vec![],
            target_buffs: vec![],
            critical_chance: 0.0,
        };

        let result = DamageCalculator::calculate_sync(&request);
        assert_eq!(result.final_damage, 10);
        assert!(!result.is_critical);
    }

    #[test]
    fn test_type_advantage() {
        let request = DamageRequest {
            base_damage: 10,
            attack_type: AttackType::Fire,
            target_type: EnemyType::Undead, // Fire weak to undead
            attacker_buffs: vec![],
            target_buffs: vec![],
            critical_chance: 0.0,
        };

        let result = DamageCalculator::calculate_sync(&request);
        assert!(result.final_damage > 10); // Should be boosted
    }

    #[test]
    fn test_armor_reduction() {
        let request = DamageRequest {
            base_damage: 20,
            attack_type: AttackType::Physical,
            target_type: EnemyType::Undead,
            attacker_buffs: vec![],
            target_buffs: vec![EnemyBuff::Armor(25)],
            critical_chance: 0.0,
        };

        let result = DamageCalculator::calculate_sync(&request);
        assert!(result.final_damage < 20); // Should be reduced
    }
}
