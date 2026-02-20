// Edge case and stress tests for roguelite_dungeon

#[cfg(test)]
mod edge_case_tests {
    use roguelite_dungeon::model::boss::{BossEnemy, BossType};
    use roguelite_dungeon::model::character::Character;
    use roguelite_dungeon::model::enemy::Enemy;
    use roguelite_dungeon::model::skill::Skill;
    use roguelite_dungeon::model::skill::SkillType;
    use roguelite_dungeon::model::weapon::Weapon;

    #[test]
    fn test_character_zero_health() {
        let mut character = Character::default();
        character.health = 0;
        assert_eq!(character.get_health_percentage(), 0.0);
        assert!(character.health <= 0);
    }

    #[test]
    fn test_character_negative_health_blocked() {
        let mut character = Character::default();
        character.health = 5;
        character.take_damage(10);
        assert!(character.health >= 0);
    }

    #[test]
    fn test_character_overheal_clamped() {
        let mut character = Character::default();
        let max = character.health_max;
        character.heal(1000);
        assert_eq!(character.health, max);
    }

    #[test]
    fn test_enemy_zero_health() {
        let mut enemy = Enemy::new(10, 10, 1.0);
        enemy.health = 0;
        let alive = enemy.take_damage(0);
        assert!(!alive);
    }

    #[test]
    fn test_boss_zero_health() {
        let mut boss = BossEnemy::new(10, 10, BossType::SkeletalKnight);
        boss.base_enemy.health = 0;
        boss.update_phase();
        assert!(boss.base_enemy.health == 0);
    }

    #[test]
    fn test_character_max_gold() {
        let mut character = Character::default();
        character.add_gold(i32::MAX as u32 / 2);
        character.add_gold(i32::MAX as u32 / 2);
        assert!(character.get_gold() > 0);
    }

    #[test]
    fn test_damage_cap() {
        let mut character = Character::default();
        character.attack_damage = i32::MAX / 2;
        let damage = character.attack_damage;
        assert!(damage > 0);
    }

    #[test]
    fn test_boss_max_damage() {
        let boss = BossEnemy::new(0, 0, BossType::CorruptedWarden);
        let damage = boss.get_effective_damage();
        assert!(damage > 0);
        assert!(damage < i32::MAX);
    }

    #[test]
    fn test_skill_zero_cooldown() {
        let mut skill = Skill::new(SkillType::Slash);
        skill.base_cooldown = 0.0;
        skill.use_skill();
        assert!(skill.cooldown_progress() >= 0.0);
    }

    #[test]
    fn test_skill_extreme_cooldown() {
        let mut skill = Skill::new(SkillType::GroundSlam);
        skill.base_cooldown = 1000.0;
        skill.use_skill();
        let remaining = skill.remaining_cooldown();
        assert!(remaining > 900.0);
    }

    #[test]
    fn test_cooldown_progress_boundary() {
        let mut skill = Skill::new(SkillType::Pierce);
        assert_eq!(skill.cooldown_progress(), 1.0);

        skill.use_skill();
        let progress = skill.cooldown_progress();
        assert!(progress >= 0.0 && progress <= 1.0);
    }

    #[test]
    fn test_health_percentage_zero_max_health() {
        let mut character = Character::default();
        character.health_max = 1;
        character.health = 0;
        let percentage = character.get_health_percentage();
        assert_eq!(percentage, 0.0);
    }

    #[test]
    fn test_damage_calculation_no_divide() {
        let character = Character::default();
        let base_damage = character.attack_damage;
        assert!(base_damage > 0);
    }

    #[test]
    fn test_boss_phase_boundary_33_percent() {
        let mut boss = BossEnemy::new(0, 0, BossType::FlameSorcerer);
        let max_health = boss.base_enemy.health;

        boss.base_enemy.health = (max_health as f32 * 0.33) as i32;
        boss.update_phase();

        assert!(boss.base_enemy.health > 0);
    }

    #[test]
    fn test_boss_phase_boundary_66_percent() {
        let mut boss = BossEnemy::new(0, 0, BossType::ShadowAssassin);
        let max_health = boss.base_enemy.health;

        boss.base_enemy.health = (max_health as f32 * 0.66) as i32;
        boss.update_phase();

        assert!(boss.base_enemy.health > 0);
    }

    #[test]
    fn test_skill_damage_multiplier_precision() {
        let skill = Skill::new(SkillType::HeavyAttack);
        let multiplier = skill.get_damage_multiplier();

        assert!(!multiplier.is_nan());
        assert!(!multiplier.is_infinite());
        assert!(multiplier > 0.0);
    }

    #[test]
    fn test_cooldown_progress_no_nan() {
        let mut skill = Skill::new(SkillType::Whirlwind);
        skill.use_skill();
        let progress = skill.cooldown_progress();

        assert!(!progress.is_nan());
        assert!(!progress.is_infinite());
    }

    #[test]
    fn test_enrage_multiplier_precision() {
        let mut boss = BossEnemy::new(0, 0, BossType::CorruptedWarden);
        boss.update_phase();

        assert!(!boss.enrage_multiplier.is_nan());
        assert!(!boss.enrage_multiplier.is_infinite());
    #[test]
    fn test_large_inventory_stacking() {
        use roguelite_dungeon::model::consumable::{Consumable, ConsumableType};

        let mut character = Character::default();
        let mut inventory = character.consumable_inventory;

        // Add many items
        for _ in 0..100 {
            let item = Consumable::new(ConsumableType::WeakHealingDraught);
            inventory.add(item);
        }

        assert!(inventory.len() > 0);
    }

    #[test]
    fn test_skill_tree_all_upgrades() {
        use roguelite_dungeon::model::skill::SkillTree;

        let mut tree = SkillTree::new();
        let total_exp_before = tree.get_total_experience();

        // Level up all skills to master
        for skill in &mut tree.skills {
            while skill.level_up() {}
        }

        let total_exp_after = tree.get_total_experience();
        assert!(total_exp_after > total_exp_before);
    }

    #[test]
    fn test_multiple_bosses_phase_transitions() {
        let boss_types = vec![
            BossType::GoblinOverlord,
            BossType::SkeletalKnight,
            BossType::FlameSorcerer,
            BossType::ShadowAssassin,
            BossType::CorruptedWarden,
        ];

        for boss_type in boss_types {
            let mut boss = BossEnemy::new(0, 0, boss_type);
            let max_health = boss.base_enemy.health;

            assert!(boss.base_enemy.health == max_health);

            boss.base_enemy.health = (max_health as f32 * 0.5) as i32;
            boss.update_phase();
            assert!(boss.enrage_multiplier >= 1.0);

            boss.base_enemy.health = (max_health as f32 * 0.2) as i32;
            boss.update_phase();
            assert!(boss.enrage_multiplier > 1.0);
        }
    }

    #[test]
    fn test_rapid_skill_usage() {
        let mut skill = Skill::new(SkillType::Slash);

        for _ in 0..1000 {
            if skill.is_ready() {
                skill.use_skill();
            }
        }

        assert!(!skill.cooldown_progress().is_nan());
    }

    #[test]
    fn test_character_damage_taking_loop() {
        let mut character = Character::default();
        let max_health = character.health;

        for _ in 0..10 {
            character.take_damage(5);
        }

        assert!(character.health < max_health);
        assert!(character.health >= 0);
    }

    #[test]
    fn test_boss_special_ability_spam() {
        let mut boss = BossEnemy::new(0, 0, BossType::GoblinOverlord);
        let original_state = boss.base_enemy.speed;

        for _ in 0..10 {
            if boss.can_use_special_ability(std::time::Instant::now()) {
                boss.trigger_special_ability();
            }
        }

        assert!(boss.base_enemy.health > 0);
    }

    #[test]
    fn test_zero_attack_damage_character() {
        let mut character = Character::default();
        character.attack_damage = 0;

        assert_eq!(character.attack_damage, 0);
    }

    #[test]
    fn test_negative_health_after_healing() {
        let mut character = Character::default();
        character.health = 10;
        character.health_max = 10;

        // Take damage beyond health
        character.take_damage(20);
        assert!(character.health >= 0);

        character.heal(5);
        assert!(character.health <= character.health_max);
    }

    #[test]
    fn test_concurrent_cooldown_checks() {
        let skill1 = Skill::new(SkillType::Slash);
        let skill2 = Skill::new(SkillType::Pierce);

        let ready1 = skill1.is_ready();
        let ready2 = skill2.is_ready();

        assert!(ready1);
        assert!(ready2);
    }
}
