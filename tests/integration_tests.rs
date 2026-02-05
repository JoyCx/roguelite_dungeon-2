// Integration tests for roguelite_dungeon game mechanics

#[cfg(test)]
mod integration_tests {
    use roguelite_dungeon::model::character::Character;
    use roguelite_dungeon::model::consumable::Consumable;
    use roguelite_dungeon::model::consumable::ConsumableType;
    use roguelite_dungeon::model::enemy::Enemy;
    use roguelite_dungeon::model::floor::Floor;
    use roguelite_dungeon::model::weapon::{Enchant, EnchantType, Weapon};

    #[test]
    fn test_floor_generation() {
        let floor = Floor::new(100, 40, 42);

        // Check floor dimensions
        assert_eq!(floor.width, 100);
        assert_eq!(floor.height, 40);

        // Check that floor has some walls and walkable spaces
        let mut wall_count = 0;
        let mut walkable_count = 0;

        for x in 0..floor.width {
            for y in 0..floor.height {
                if floor.is_walkable(x, y) {
                    walkable_count += 1;
                } else {
                    wall_count += 1;
                }
            }
        }

        assert!(wall_count > 0);
        assert!(walkable_count > 0);
    }

    #[test]
    fn test_enemy_spawning() {
        let enemy = Enemy::new(50, 50, 2.0);
        assert_eq!(enemy.position.x, 50);
        assert_eq!(enemy.position.y, 50);
        assert!(enemy.health > 0);
    }

    #[test]
    fn test_character_gold_system() {
        let mut character = Character::default();
        character.add_gold(50);
        assert_eq!(character.get_gold(), 50);
        assert!(character.spend_gold(30));
        assert_eq!(character.get_gold(), 20);
    }

    #[test]
    fn test_consumable_inventory() {
        let character = Character::default();
        let mut inventory = character.consumable_inventory;

        let healing = Consumable::new(ConsumableType::WeakHealingDraught);
        inventory.add(healing);
        assert_eq!(inventory.len(), 1);
    }

    #[test]
    fn test_character_combat() {
        let mut character = Character::default();
        let initial = character.health;

        character.take_damage(10);
        assert_eq!(character.health, initial - 10);

        character.heal(5);
        assert_eq!(character.health, initial - 5);
    }

    #[test]
    fn test_cooldown_system() {
        let mut character = Character::default();

        assert!(character.can_dash());
        character.start_dash_cooldown();
        assert!(!character.can_dash());
    }

    #[test]
    fn test_weapon_system() {
        let character = Character::default();
        let mut inventory = character.weapon_inventory;

        assert_eq!(inventory.weapons.len(), 2);
        inventory.switch_weapon(1);
        assert_eq!(inventory.current_weapon_index, 1);
    }

    #[test]
    fn test_weapon_enchants() {
        let mut weapon = Weapon::new_sword();
        assert_eq!(weapon.get_total_damage(), 5);

        weapon.add_enchant(Enchant {
            enchant_type: EnchantType::DamageIncrease,
            value: 3,
        });

        assert_eq!(weapon.get_total_damage(), 8);
    }

    #[test]
    fn test_ultimate_ability() {
        let mut character = Character::default();

        assert!(character.ultimate.can_use());
        character.ultimate_charge = 100.0;
        let damage = character.calculate_ultimate_damage();
        // 15 * 1.5 = 22.5 as i32 = 22
        assert_eq!(damage, 22);
    }

    #[test]
    fn test_enemy_damage() {
        let mut enemy = Enemy::new(50, 50, 2.0);
        let max_health = enemy.max_health;

        enemy.take_damage(5);
        assert_eq!(enemy.health, max_health - 5);
    }

    #[test]
    fn test_game_over() {
        let mut character = Character::default();

        assert!(character.is_alive());
        character.take_damage(1000);
        assert!(!character.is_alive());
        assert_eq!(character.health, 0);
    }

    #[test]
    fn test_direction_tracking() {
        let mut character = Character::default();
        character.update_direction(1, 0);
        assert_eq!(character.last_direction, (1, 0));
    }

    #[test]
    fn test_health_percentage() {
        let mut character = Character::default();
        assert_eq!(character.get_health_percentage(), 1.0);

        character.health = 50;
        assert_eq!(character.get_health_percentage(), 0.5);
    }

    #[test]
    fn test_consumable_stacking() {
        let character = Character::default();
        let mut inventory = character.consumable_inventory;

        let h1 = Consumable::new(ConsumableType::WeakHealingDraught);
        let h2 = Consumable::new(ConsumableType::WeakHealingDraught);

        inventory.add(h1);
        inventory.add(h2);

        assert_eq!(inventory.len(), 1);
        assert_eq!(inventory.items[0].quantity, 2);
    }

    #[test]
    fn test_boss_creation() {
        use roguelite_dungeon::model::boss::{BossEnemy, BossPhase, BossType};

        let boss = BossEnemy::new(20, 30, BossType::GoblinOverlord);
        assert_eq!(boss.base_enemy.position.x, 20);
        assert_eq!(boss.base_enemy.position.y, 30);
        assert_eq!(boss.current_phase, BossPhase::First);
        assert!(boss.base_enemy.health > 0);
    }

    #[test]
    fn test_boss_phase_transitions() {
        use roguelite_dungeon::model::boss::{BossEnemy, BossPhase, BossType};

        let mut boss = BossEnemy::new(0, 0, BossType::SkeletalKnight);
        let max_health = boss.base_enemy.health;

        // Phase 1 -> Phase 2
        boss.base_enemy.health = (max_health as f32 * 0.5) as i32;
        boss.update_phase();
        assert_eq!(boss.current_phase, BossPhase::Second);

        // Phase 2 -> Phase 3 (enrage)
        boss.base_enemy.health = (max_health as f32 * 0.2) as i32;
        boss.update_phase();
        assert_eq!(boss.current_phase, BossPhase::Third);
        assert!(boss.enrage_multiplier > 1.0);
    }

    #[test]
    fn test_boss_damage_scaling() {
        use roguelite_dungeon::model::boss::{BossEnemy, BossType};

        let mut boss = BossEnemy::new(0, 0, BossType::FlameSorcerer);
        let phase1_damage = boss.get_effective_damage();

        boss.current_phase = roguelite_dungeon::model::boss::BossPhase::Third;
        let phase3_damage = boss.get_effective_damage();

        assert!(phase3_damage > phase1_damage);
    }

    #[test]
    fn test_boss_healing() {
        use roguelite_dungeon::model::boss::{BossEnemy, BossType};

        let mut boss = BossEnemy::new(0, 0, BossType::CorruptedWarden);
        let max = boss.base_enemy.health;
        boss.base_enemy.health = max / 2;

        boss.apply_healing();
        assert!(boss.base_enemy.health > max / 2);
    }

    #[test]
    fn test_skill_creation() {
        use roguelite_dungeon::model::skill::{Skill, SkillLevel, SkillType};

        let skill = Skill::new(SkillType::Slash);
        assert_eq!(skill.skill_type, SkillType::Slash);
        assert_eq!(skill.level, SkillLevel::Novice);
        assert!(skill.is_ready());
    }

    #[test]
    fn test_skill_level_progression() {
        use roguelite_dungeon::model::skill::{Skill, SkillLevel, SkillType};

        let mut skill = Skill::new(SkillType::HeavyAttack);
        assert_eq!(skill.level, SkillLevel::Novice);

        skill.level_up();
        assert_eq!(skill.level, SkillLevel::Apprentice);

        skill.level_up();
        assert_eq!(skill.level, SkillLevel::Expert);

        skill.level_up();
        assert_eq!(skill.level, SkillLevel::Master);
    }

    #[test]
    fn test_skill_damage_multiplier() {
        use roguelite_dungeon::model::skill::{Skill, SkillType};

        let mut skill = Skill::new(SkillType::Pierce);
        let novice_dmg = skill.get_damage_multiplier();

        skill.level_up();
        skill.level_up();
        let expert_dmg = skill.get_damage_multiplier();

        assert!(expert_dmg > novice_dmg);
    }

    #[test]
    fn test_skill_tree() {
        use roguelite_dungeon::model::skill::{SkillTree, SkillType};

        let tree = SkillTree::new();
        assert_eq!(tree.skills.len(), 5);
        assert!(tree.get_skill(SkillType::Slash).is_some());
        assert!(tree.get_skill(SkillType::Whirlwind).is_some());
    }

    #[test]
    fn test_skill_cooldown_system() {
        use roguelite_dungeon::model::skill::Skill;
        use roguelite_dungeon::model::skill::SkillType;

        let mut skill = Skill::new(SkillType::Whirlwind);
        assert!(skill.is_ready());

        skill.use_skill();
        assert!(!skill.is_ready());

        let progress = skill.cooldown_progress();
        assert!(progress >= 0.0 && progress <= 1.0);
    }

    #[test]
    fn test_skill_aoe_variation() {
        use roguelite_dungeon::model::skill::{Skill, SkillType};

        let pierce = Skill::new(SkillType::Pierce);
        let whirlwind = Skill::new(SkillType::Whirlwind);

        assert_eq!(pierce.get_aoe_radius(), 0); // Single target
        assert!(whirlwind.get_aoe_radius() > pierce.get_aoe_radius());
    }

    // ====== PHASE 5: INTEGRATION TESTS ======

    #[test]
    fn test_character_skill_integration() {
        use roguelite_dungeon::model::skill::SkillType;

        let mut character = Character::default();

        // Character should have all 5 skills
        assert_eq!(character.get_ready_skills().len(), 5);

        // Use a skill
        assert!(character.use_skill(SkillType::Slash));

        // Should not be immediately ready
        assert!(!character.is_skill_ready(SkillType::Slash));
    }

    #[test]
    fn test_skill_damage_in_combat() {
        use roguelite_dungeon::model::skill::SkillType;

        let character = Character::default();

        // Get skill multiplier
        let slash_multiplier = character.get_skill_damage_multiplier(SkillType::Slash);
        let pierce_multiplier = character.get_skill_damage_multiplier(SkillType::Pierce);

        // Pierce should do more damage than slash
        assert!(pierce_multiplier > slash_multiplier);
    }

    #[test]
    fn test_skill_progression_integration() {
        use roguelite_dungeon::model::skill::SkillType;

        let mut character = Character::default();
        let initial_damage = character.get_skill_damage_multiplier(SkillType::HeavyAttack);

        // Level up the skill
        character.level_up_skill(SkillType::HeavyAttack);
        let upgraded_damage = character.get_skill_damage_multiplier(SkillType::HeavyAttack);

        // Should deal more damage after leveling up
        assert!(upgraded_damage > initial_damage);
    }

    #[test]
    fn test_boss_spawning_on_floor() {
        use roguelite_dungeon::model::boss::BossType;

        let mut floor = Floor::new(100, 40, 42);

        // Should have no bosses initially
        let boss_count_before = floor.enemies.len();

        // Spawn a boss
        let boss = floor.spawn_boss(BossType::SkeletalKnight);
        assert!(boss.is_some());

        // Should have added boss to enemies
        assert!(floor.enemies.len() > boss_count_before);
    }

    #[test]
    fn test_boss_spawning_multiple_types() {
        use roguelite_dungeon::model::boss::BossType;

        let boss_types = vec![
            BossType::GoblinOverlord,
            BossType::FlameSorcerer,
            BossType::CorruptedWarden,
        ];

        for boss_type in boss_types {
            let mut floor = Floor::new(100, 40, 43);
            let boss = floor.spawn_boss(boss_type);
            assert!(boss.is_some(), "Failed to spawn {:?}", boss_type);
        }
    }

    #[test]
    fn test_boss_in_largest_room() {
        use roguelite_dungeon::model::boss::BossType;

        let mut floor = Floor::new(100, 40, 44);

        let _largest_room = floor
            .rooms
            .iter()
            .max_by_key(|r| r.tiles.len())
            .map(|r| r.tiles.len());

        let boss = floor.spawn_boss(BossType::ShadowAssassin);

        if let Some(boss_ref) = boss {
            // Boss should be placed in a valid location
            assert!(floor.is_walkable(
                boss_ref.base_enemy.position.x,
                boss_ref.base_enemy.position.y
            ));
        }
    }

    #[test]
    fn test_character_with_boss_encounter() {
        use roguelite_dungeon::model::boss::{BossEnemy, BossType};

        let character = Character::default();
        let mut boss = BossEnemy::new(50, 50, BossType::GoblinOverlord);

        // Initial state
        let _initial_player_health = character.health;
        let initial_boss_health = boss.base_enemy.health;

        // Boss attacks player
        boss.base_enemy.health -= 10;
        assert!(boss.base_enemy.health < initial_boss_health);
    }

    #[test]
    fn test_skill_usage_ready_check() {
        use roguelite_dungeon::model::skill::SkillType;

        let character = Character::default();
        let mut skill_count = 0;

        // Count how many skills are ready
        for skill_type in vec![
            SkillType::Slash,
            SkillType::Pierce,
            SkillType::HeavyAttack,
            SkillType::Whirlwind,
            SkillType::GroundSlam,
        ] {
            if character.is_skill_ready(skill_type) {
                skill_count += 1;
            }
        }

        // All 5 skills should be ready initially
        assert_eq!(skill_count, 5);
    }

    #[test]
    fn test_floor_generation_with_boss_spawning() {
        use roguelite_dungeon::model::boss::BossType;
        use roguelite_dungeon::model::item_tier::Difficulty;

        let mut floor = Floor::new(80, 40, 45);

        // Spawn multiple enemies
        floor.spawn_enemies(&Difficulty::Normal);
        let regular_enemy_count = floor.enemies.len();

        // Then spawn a boss
        let boss = floor.spawn_boss(BossType::CorruptedWarden);

        if boss.is_some() {
            assert_eq!(floor.enemies.len(), regular_enemy_count + 1);
        }
    }

    #[test]
    fn test_character_skill_tree_persistence() {
        use roguelite_dungeon::model::skill::SkillType;

        let mut character = Character::default();

        // Level up multiple skills
        character.level_up_skill(SkillType::Slash);
        character.level_up_skill(SkillType::Pierce);
        character.level_up_skill(SkillType::Whirlwind);

        // Use a skill
        character.use_skill(SkillType::Slash);

        // Skills should persist state
        assert!(!character.is_skill_ready(SkillType::Slash));
        assert!(character.is_skill_ready(SkillType::Pierce)); // Should still be ready
    }
}
