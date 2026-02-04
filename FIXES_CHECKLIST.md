# Quick Fix Checklist

## 🔴 CRITICAL (BLOCKING COMPILATION)
- [ ] **Attack Pattern Enum Mismatch** - Align enum variants with code references
  - File: `src/model/attack_pattern.rs`
  - Fix: Change `BasicSlash`, `WhirlwindAttack`, `SwordThrust` to match enum (or vice versa)
  
- [ ] **Deprecated Rand API** - Update rand crate calls
  - File: `src/model/floor.rs:607-608`
  - Fix: `rand::thread_rng()` → `rand::rng()`, `gen_range()` → `random_range()`
  - File: `src/model/enemy.rs:165,296,319-320` - Same fix
  
- [ ] **Type Mismatches** - Fix usize/i32 mismatches
  - File: `src/model/attack_pattern.rs:54,72`
  - Fix: Cast usize parameters to i32

## 🟠 HIGH PRIORITY (Warnings & Safety)
- [ ] **Unused Variables**
  - [ ] `src/model/collision.rs:33` - Remove or use `cell_range`
  - [ ] `src/model/enemy.rs:318` - Remove or use `attempt`
  - [ ] `src/app.rs:241` - Remove unused `use std::io::Write;`

- [ ] **Unused Assignment**
  - [ ] `src/model/enemy.rs:268` - Review `current = parent` assignment

## 🟡 MEDIUM PRIORITY (Code Quality)
- [ ] **Create Reusable Cooldown Struct**
  - Extract common cooldown logic from `character.rs`
  - Refactor dash, attack, bow, block cooldowns
  
- [ ] **Consolidate Animation Methods**
  - Create animation builder helpers in `attack_pattern.rs`
  - Reduce 600+ lines of duplication
  
- [ ] **Move Magic Numbers to constants.rs**
  - [ ] `0.5` animation duration
  - [ ] `8.0` bleed duration
  - [ ] `0.2` attack animation time
  - [ ] All hardcoded color indices

- [ ] **Fix Save/Load Error Handling**
  - [ ] `src/model/gamesave.rs` - Proper error propagation
  - [ ] `src/model/settings.rs` - Proper error propagation

## 🟢 NICE TO HAVE
- [ ] **Optimize Pathfinding** - Add path caching
- [ ] **Fix Camera System** - Make configurable
- [ ] **Add Error Logging** - Log to file
- [ ] **Implement Tests** - Unit + integration tests
- [ ] **Complete Feature Systems**
  - [ ] Consumable effects application
  - [ ] Projectile wall collision
  - [ ] Ultimate damage application
  - [ ] Weapon skill system

## Priority Order (Recommended)
1. Fix attack pattern enum (1-2h) - **BLOCKING**
2. Update rand API (15min) - **BLOCKING**
3. Fix type mismatches (15min) - **BLOCKING**
4. Remove unused code (5min) - **Cleanup**
5. Create Cooldown struct (2h) - **Refactor**
6. Create animation helpers (3h) - **Refactor**
7. Move magic numbers (1h) - **Polish**

Total time for critical issues: **~2.5 hours**
Total time for high priority: **~4.5 hours**
