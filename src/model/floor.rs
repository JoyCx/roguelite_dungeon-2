use crate::model::item::ItemDrop;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use ratatui::style::{Color, Style};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct RoomId(pub usize);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Room {
    pub id: RoomId,
    pub center: (i32, i32),
    pub tiles: Vec<(i32, i32)>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Floor {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<bool>,
    pub seed: u64,
    pub rooms: Vec<Room>,
    pub tile_to_room: Vec<Option<RoomId>>,
    pub items: Vec<ItemDrop>,
    pub enemies: Vec<crate::model::enemy::Enemy>,
}

impl Floor {
    pub fn new(width: i32, height: i32, seed: u64) -> Self {
        let mut floor = Self {
            width,
            height,
            tiles: vec![true; (width * height) as usize],
            seed,
            rooms: Vec::new(),
            tile_to_room: vec![None; (width * height) as usize],
            items: Vec::new(),
            enemies: Vec::new(),
        };
        floor.generate();
        floor.detect_rooms();
        floor
    }

    #[allow(dead_code)]
    pub fn as_string(&self) -> String {
        let mut result = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let is_wall = self.get_tile(x, y);
                result.push(if is_wall { '#' } else { '.' });
            }
            result.push('\n');
        }
        result
    }

    fn generate(&mut self) {
        let mut rng = StdRng::seed_from_u64(self.seed);

        let fill_probability = 45;
        for y in 0..self.height {
            for x in 0..self.width {
                if y == 0 || y == self.height - 1 || x == 0 || x == self.width - 1 {
                    self.set_tile(x, y, true);
                } else if rng.random_range(0..100) < fill_probability {
                    self.set_tile(x, y, false);
                } else {
                    self.set_tile(x, y, true);
                }
            }
        }

        let total_iterations = 5;
        let cutoff_big_area = 3;

        for iteration in 0..total_iterations {
            let mut new_tiles = self.tiles.clone();

            for y in 1..self.height - 1 {
                for x in 1..self.width - 1 {
                    let wall_count_1 = self.count_walls_within_distance(x, y, 1);
                    let wall_count_2 = self.count_walls_within_distance(x, y, 2);

                    let new_is_wall = if iteration < cutoff_big_area {
                        wall_count_1 >= 5 || wall_count_2 <= 2
                    } else {
                        wall_count_1 >= 5
                    };

                    let idx = (y * self.width + x) as usize;
                    new_tiles[idx] = new_is_wall;
                }
            }

            self.tiles = new_tiles;
        }

        self.connect_caves();
    }

    pub fn get_tile(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            true
        } else {
            self.tiles[(y * self.width + x) as usize]
        }
    }

    fn set_tile(&mut self, x: i32, y: i32, is_wall: bool) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            let idx = (y * self.width + x) as usize;
            self.tiles[idx] = is_wall;
        }
    }

    fn count_walls_within_distance(&self, x: i32, y: i32, distance: i32) -> i32 {
        let mut count = 0;
        for dy in -distance..=distance {
            for dx in -distance..=distance {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x + dx;
                let ny = y + dy;
                if self.get_tile(nx, ny) {
                    count += 1;
                }
            }
        }
        count
    }

    fn connect_caves(&mut self) {
        let map_sections = self.get_map_sections();

        if map_sections.len() <= 1 {
            return;
        }

        let mut union_find = UnionFind::new(map_sections.len());

        while union_find.count() > 1 {
            for i in 0..map_sections.len() {
                let closest_idx = self.find_nearest_cave(&map_sections, i, &mut union_find);

                if closest_idx != i && !union_find.connected(i, closest_idx) {
                    let (x1, y1) = map_sections[i].center();
                    let (x2, y2) = map_sections[closest_idx].center();
                    self.carve_tunnel(x1, y1, x2, y2);
                    union_find.union(i, closest_idx);
                }
            }
        }
    }

    fn get_map_sections(&self) -> Vec<MapSection> {
        let mut visited = vec![false; (self.width * self.height) as usize];
        let mut sections = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (y * self.width + x) as usize;
                if !self.tiles[idx] && !visited[idx] {
                    let mut section = MapSection::new();
                    let mut queue = VecDeque::new();

                    queue.push_back((x, y));
                    visited[idx] = true;

                    while let Some((cx, cy)) = queue.pop_front() {
                        section.add_tile(cx, cy);

                        for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                            let nx = cx + dx;
                            let ny = cy + dy;
                            if nx >= 0 && nx < self.width && ny >= 0 && ny < self.height {
                                let n_idx = (ny * self.width + nx) as usize;
                                if !self.tiles[n_idx] && !visited[n_idx] {
                                    visited[n_idx] = true;
                                    queue.push_back((nx, ny));
                                }
                            }
                        }
                    }

                    if !section.tiles.is_empty() {
                        sections.push(section);
                    }
                }
            }
        }

        sections
    }

    fn find_nearest_cave(
        &self,
        sections: &[MapSection],
        section_idx: usize,
        union_find: &mut UnionFind,
    ) -> usize {
        let start = &sections[section_idx];
        let mut closest_idx = section_idx;
        let mut closest_distance = i32::MAX;

        for (i, section) in sections.iter().enumerate() {
            if i == section_idx || union_find.connected(i, section_idx) {
                continue;
            }

            let distance = Self::distance_between(start, section);
            if distance < closest_distance {
                closest_distance = distance;
                closest_idx = i;
            }
        }

        closest_idx
    }

    fn distance_between(section_a: &MapSection, section_b: &MapSection) -> i32 {
        let (x1, y1) = section_a.center();
        let (x2, y2) = section_b.center();
        (x1 - x2).abs() + (y1 - y2).abs()
    }

    fn carve_tunnel(&mut self, mut x1: i32, mut y1: i32, x2: i32, y2: i32) {
        while x1 != x2 {
            self.set_tile(x1, y1, false);
            x1 += if x1 < x2 { 1 } else { -1 };
        }
        while y1 != y2 {
            self.set_tile(x1, y1, false);
            y1 += if y1 < y2 { 1 } else { -1 };
        }
    }

    pub fn is_walkable(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return false;
        }

        let tile_val = self.get_tile(x, y);
        !tile_val
    }

    fn detect_rooms(&mut self) {
        let mut visited = vec![false; (self.width * self.height) as usize];
        let mut room_list = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (y * self.width + x) as usize;
                if !self.tiles[idx] && !visited[idx] {
                    let mut room_tiles = Vec::new();
                    let mut queue = VecDeque::new();

                    queue.push_back((x, y));
                    visited[idx] = true;

                    while let Some((cx, cy)) = queue.pop_front() {
                        room_tiles.push((cx, cy));

                        for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                            let nx = cx + dx;
                            let ny = cy + dy;
                            if nx >= 0 && nx < self.width && ny >= 0 && ny < self.height {
                                let n_idx = (ny * self.width + nx) as usize;
                                if !self.tiles[n_idx] && !visited[n_idx] {
                                    visited[n_idx] = true;
                                    queue.push_back((nx, ny));
                                }
                            }
                        }
                    }

                    if !room_tiles.is_empty() {
                        let sum_x: i32 = room_tiles.iter().map(|(x, _)| x).sum();
                        let sum_y: i32 = room_tiles.iter().map(|(_, y)| y).sum();
                        let center = (
                            sum_x / room_tiles.len() as i32,
                            sum_y / room_tiles.len() as i32,
                        );

                        let room_id = RoomId(room_list.len());
                        let room = Room {
                            id: room_id,
                            center,
                            tiles: room_tiles.clone(),
                        };
                        room_list.push(room);

                        for (tx, ty) in room_tiles {
                            let tile_idx = (ty * self.width + tx) as usize;
                            self.tile_to_room[tile_idx] = Some(room_id);
                        }
                    }
                }
            }
        }

        self.rooms = room_list;
    }

    pub fn find_walkable_tile(&self) -> Option<(i32, i32)> {
        let center_x = self.width / 2;
        let center_y = self.height / 2;

        for radius in 0..((self.width.max(self.height)) / 2) {
            for dx in -radius..=radius {
                for dy in -radius..=radius {
                    if radius > 0 && dx.abs() != radius && dy.abs() != radius {
                        continue;
                    }

                    let x = center_x + dx;
                    let y = center_y + dy;

                    if x >= 0
                        && x < self.width
                        && y >= 0
                        && y < self.height
                        && self.is_walkable(x, y)
                    {
                        return Some((x, y));
                    }
                }
            }
        }

        for y in 0..self.height {
            for x in 0..self.width {
                if self.is_walkable(x, y) {
                    return Some((x, y));
                }
            }
        }

        None
    }

    pub fn styled_grid(&self) -> Vec<(i32, i32, char, Style)> {
        let mut result = Vec::new();

        self.tiles.iter().enumerate().for_each(|(i, &is_wall)| {
            let x = (i as i32) % self.width;
            let y = (i as i32) / self.width;

            let ch = if is_wall { '█' } else { '.' };

            let style = if is_wall {
                let nearby_floors = self.count_nearby_floors(x, y, 2);
                match nearby_floors {
                    0..=1 => Style::new().fg(Color::Indexed(236)),
                    2..=3 => Style::new().fg(Color::Indexed(238)),
                    4..=5 => Style::new().fg(Color::Indexed(240)),
                    _ => Style::new().fg(Color::Indexed(242)),
                }
            } else {
                let wall_proximity = self.count_walls_near(x, y, 1);
                match wall_proximity {
                    0..=2 => Style::new().fg(Color::Indexed(246)),
                    3..=4 => Style::new().fg(Color::Indexed(244)),
                    5..=6 => Style::new().fg(Color::Indexed(242)),
                    7..=8 => Style::new().fg(Color::Indexed(240)),
                    _ => Style::new().fg(Color::Indexed(238)),
                }
            };

            result.push((x, y, ch, style));
        });

        result
    }

    pub fn get_styled_tile(&self, x: i32, y: i32) -> Option<(char, Style)> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return None;
        }

        let is_wall = self.get_tile(x, y);

        if is_wall {
            // Multiple wall characters for visual variety
            let wall_chars = ['█', '▓', '▒', '▀', '▄', '■', '◆', '◊'];
            let nearby_floors = self.count_nearby_floors(x, y, 2);

            // Choose wall character based on position (seeded from coordinates for consistency)
            let char_idx =
                ((x as usize).wrapping_mul(73) ^ (y as usize).wrapping_mul(97)) % wall_chars.len();
            let ch = wall_chars[char_idx];

            let color = match nearby_floors {
                0..=1 => Color::Indexed(236),
                2..=3 => Color::Indexed(238),
                4..=5 => Color::Indexed(240),
                _ => Color::Indexed(242),
            };

            let style = Style::new().fg(color);
            Some((ch, style))
        } else {
            let wall_proximity = self.count_walls_near(x, y, 1);
            let ch = '.';
            let color = match wall_proximity {
                0..=2 => Color::Indexed(246),
                3..=4 => Color::Indexed(244),
                5..=6 => Color::Indexed(242),
                7..=8 => Color::Indexed(240),
                _ => Color::Indexed(238),
            };

            let style = Style::new().fg(color);
            Some((ch, style))
        }
    }

    #[allow(dead_code)]
    fn count_nearby_floors(&self, x: i32, y: i32, distance: i32) -> i32 {
        let mut count = 0;
        for dy in -distance..=distance {
            for dx in -distance..=distance {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x + dx;
                let ny = y + dy;
                if nx >= 0
                    && nx < self.width
                    && ny >= 0
                    && ny < self.height
                    && !self.get_tile(nx, ny)
                {
                    count += 1;
                }
            }
        }
        count
    }

    #[allow(dead_code)]
    fn count_walls_near(&self, x: i32, y: i32, distance: i32) -> i32 {
        let mut count = 0;
        for dy in -distance..=distance {
            for dx in -distance..=distance {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x + dx;
                let ny = y + dy;
                if nx >= 0
                    && nx < self.width
                    && ny >= 0
                    && ny < self.height
                    && self.get_tile(nx, ny)
                {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn add_item(&mut self, item: ItemDrop) {
        self.items.push(item);
    }

    pub fn items_at(&self, x: i32, y: i32) -> Vec<&ItemDrop> {
        self.items
            .iter()
            .filter(|item| item.x == x && item.y == y)
            .collect()
    }

    pub fn items_at_mut(&mut self, x: i32, y: i32) -> Vec<&mut ItemDrop> {
        self.items
            .iter_mut()
            .filter(|item| item.x == x && item.y == y)
            .collect()
    }

    pub fn pickup_item(&mut self, x: i32, y: i32) -> Option<ItemDrop> {
        // Find the actual index in self.items of an item at (x, y)
        if let Some(pos) = self
            .items
            .iter()
            .position(|item| item.x == x && item.y == y)
        {
            Some(self.items.remove(pos))
        } else {
            None
        }
    }

    /// Try to drop an item at an adjacent empty position next to (x, y)
    /// Returns true if item was dropped, false if no empty space found
    pub fn try_drop_item_adjacent(&mut self, item: ItemDrop, player_x: i32, player_y: i32) -> bool {
        // Try adjacent tiles in order
        let adjacent = vec![
            (player_x + 1, player_y),
            (player_x - 1, player_y),
            (player_x, player_y + 1),
            (player_x, player_y - 1),
        ];

        for (x, y) in adjacent {
            // Check if position is walkable and empty
            if x >= 0
                && x < self.width as i32
                && y >= 0
                && y < self.height as i32
                && self.is_walkable(x, y)
                && !self.item_exists_at(x, y)
                && !self.enemy_exists_at(x, y)
            {
                let mut dropped = item;
                dropped.x = x;
                dropped.y = y;
                self.add_item(dropped);
                return true;
            }
        }
        false
    }

    /// Find a random spawn position for the player
    /// Ensures the player spawns:
    /// - In a walkable space (not a wall)
    /// - In the largest connected region of the map (to avoid isolated pockets)
    /// - At a random location (not always the same spot)
    pub fn find_player_spawn(&self) -> Option<(i32, i32)> {
        use rand::Rng;

        // Find all walkable tiles and identify the largest connected region
        let mut visited = vec![false; (self.width * self.height) as usize];
        let mut largest_region_start = None;
        let mut largest_region_size = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (y * self.width + x) as usize;
                if self.is_walkable(x, y) && !visited[idx] {
                    // BFS to find all connected walkable tiles and count them
                    let mut region_size = 0;
                    let mut queue = VecDeque::new();

                    queue.push_back((x, y));
                    visited[idx] = true;

                    while let Some((cx, cy)) = queue.pop_front() {
                        region_size += 1;

                        // Check all 4 adjacent tiles
                        for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                            let nx = cx + dx;
                            let ny = cy + dy;
                            if nx >= 0 && nx < self.width && ny >= 0 && ny < self.height {
                                let n_idx = (ny * self.width + nx) as usize;
                                if self.is_walkable(nx, ny) && !visited[n_idx] {
                                    visited[n_idx] = true;
                                    queue.push_back((nx, ny));
                                }
                            }
                        }
                    }

                    // Keep track of the largest region
                    if region_size > largest_region_size {
                        largest_region_size = region_size;
                        largest_region_start = Some((x, y));
                    }
                }
            }
        }

        // Now that we know the start of the largest region, do another BFS to collect positions
        if let Some((start_x, start_y)) = largest_region_start {
            let mut visited2 = vec![false; (self.width * self.height) as usize];
            let mut region_tiles = Vec::new();
            let mut queue = VecDeque::new();

            let start_idx = (start_y * self.width + start_x) as usize;
            queue.push_back((start_x, start_y));
            visited2[start_idx] = true;

            while let Some((cx, cy)) = queue.pop_front() {
                region_tiles.push((cx, cy));

                for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let nx = cx + dx;
                    let ny = cy + dy;
                    if nx >= 0 && nx < self.width && ny >= 0 && ny < self.height {
                        let n_idx = (ny * self.width + nx) as usize;
                        if self.is_walkable(nx, ny) && !visited2[n_idx] {
                            visited2[n_idx] = true;
                            queue.push_back((nx, ny));
                        }
                    }
                }
            }

            // Spawn in a random location within the largest region
            if !region_tiles.is_empty() {
                let mut rng = rand::thread_rng();
                let idx = rng.gen_range(0..region_tiles.len());
                return Some(region_tiles[idx]);
            }
        }

        None
    }

    pub fn spawn_random_items(
        &mut self,
        count: i32,
        difficulty: &crate::model::item_tier::Difficulty,
    ) {
        use crate::model::consumable::{Consumable, ConsumableType};
        use crate::model::item_tier::ItemTier;

        let mut rng = StdRng::seed_from_u64(self.seed.wrapping_add(999));
        let consumable_types = [
            ConsumableType::WeakHealingDraught,
            ConsumableType::BandageRoll,
            ConsumableType::AntitoxinVial,
            ConsumableType::FireOilFlask,
            ConsumableType::BlessedBread,
        ];

        let all_tiers = [
            ItemTier::Common,
            ItemTier::Rare,
            ItemTier::Epic,
            ItemTier::Exotic,
            ItemTier::Legendary,
            ItemTier::Mythic,
            ItemTier::Godly,
        ];

        let mut spawned = 0;
        let mut total_attempts = 0;
        let max_total_attempts = (count as usize) * 100; // Prevent infinite loop

        while spawned < count && total_attempts < max_total_attempts {
            total_attempts += 1;
            let mut attempts = 0;
            while attempts < 50 {
                let x = rng.random_range(1..self.width - 1);
                let y = rng.random_range(1..self.height - 1);

                if !self.get_tile(x, y) && !self.item_exists_at(x, y) {
                    // It's a floor tile with no item, spawn item here

                    // Determine tier based on difficulty drop chances
                    let tier = self.determine_tier(&mut rng, difficulty, &all_tiers);

                    let idx = rng.random_range(0..consumable_types.len());
                    let consumable_type = consumable_types[idx].clone();
                    let consumable = Consumable::new(consumable_type);
                    let item = ItemDrop::consumable_with_tier(consumable, x, y, tier);
                    self.add_item(item);
                    spawned += 1;
                    break;
                }
                attempts += 1;
            }
        }
    }

    /// Spawn enemies scaled by difficulty
    /// Spawns 5-15 enemies depending on difficulty
    /// Ensures no overlaps with items, other enemies, or walls
    pub fn spawn_enemies(&mut self, difficulty: &crate::model::item_tier::Difficulty) {
        use crate::model::enemy_type;

        let mut rng = StdRng::seed_from_u64(self.seed.wrapping_add(1337));

        // Determine number of enemies based on difficulty
        let enemy_count = match difficulty {
            crate::model::item_tier::Difficulty::Easy => rng.random_range(5..8),
            crate::model::item_tier::Difficulty::Normal => rng.random_range(8..12),
            crate::model::item_tier::Difficulty::Hard => rng.random_range(12..16),
            crate::model::item_tier::Difficulty::Death => rng.random_range(15..20),
        };

        // Get available enemy templates for this difficulty
        let templates = enemy_type::get_enemies_for_difficulty(difficulty);

        let mut spawned = 0;
        let mut total_attempts = 0;
        let max_total_attempts = (enemy_count as usize) * 100;

        while spawned < enemy_count && total_attempts < max_total_attempts {
            total_attempts += 1;
            let mut attempts = 0;

            while attempts < 50 {
                let x = rng.random_range(1..self.width - 1);
                let y = rng.random_range(1..self.height - 1);

                // Check if position is valid: floor tile, no item, no other enemy
                if !self.get_tile(x, y) && !self.item_exists_at(x, y) && !self.enemy_exists_at(x, y)
                {
                    // Pick random enemy template
                    let template_idx = rng.random_range(0..templates.len());
                    let template = templates[template_idx].clone();

                    // Create enemy from template
                    let mut enemy = crate::model::enemy::Enemy::new(x, y, template.speed);
                    enemy.health = template.health;
                    enemy.max_health = template.health;
                    enemy.rarity = template.rarity.clone();
                    enemy.base_gold = template.rarity.calculate_gold_drop(difficulty);
                    enemy.detection_radius = template.rarity.calculate_detection_radius(difficulty);

                    self.enemies.push(enemy);
                    spawned += 1;
                    break;
                }
                attempts += 1;
            }
        }
    }

    /// Check if an enemy already exists at this position
    pub fn enemy_exists_at(&self, x: i32, y: i32) -> bool {
        self.enemies
            .iter()
            .any(|enemy| enemy.position.x == x && enemy.position.y == y)
    }

    /// Check if an item already exists at this position
    pub fn item_exists_at(&self, x: i32, y: i32) -> bool {
        self.items.iter().any(|item| item.x == x && item.y == y)
    }

    /// Determine item tier based on difficulty drop chances
    fn determine_tier(
        &self,
        rng: &mut StdRng,
        difficulty: &crate::model::item_tier::Difficulty,
        tiers: &[crate::model::item_tier::ItemTier],
    ) -> crate::model::item_tier::ItemTier {
        use crate::model::item_tier::ItemTier;

        let roll = rng.random_range(0.0..100.0);
        let mut cumulative = 0.0;

        // Check from rarest to most common
        for tier in tiers.iter().rev() {
            let chance = difficulty.get_tier_drop_chance(tier);
            cumulative += chance;
            if roll <= cumulative {
                return tier.clone();
            }
        }

        // Fallback to common if something goes wrong
        ItemTier::Common
    }

    pub fn update_items(&mut self, delta: f32) {
        for item in &mut self.items {
            item.update(delta);
        }
    }
}

#[derive(Clone, Debug)]
struct MapSection {
    tiles: Vec<(i32, i32)>,
}

impl MapSection {
    fn new() -> Self {
        Self { tiles: Vec::new() }
    }

    fn add_tile(&mut self, x: i32, y: i32) {
        self.tiles.push((x, y));
    }

    fn center(&self) -> (i32, i32) {
        if self.tiles.is_empty() {
            return (0, 0);
        }
        let sum_x: i32 = self.tiles.iter().map(|(x, _)| x).sum();
        let sum_y: i32 = self.tiles.iter().map(|(_, y)| y).sum();
        (
            sum_x / self.tiles.len() as i32,
            sum_y / self.tiles.len() as i32,
        )
    }
}

#[derive(Debug)]
struct UnionFind {
    parent: Vec<usize>,
    count: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        Self { parent, count: n }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            self.parent[root_x] = root_y;
            self.count -= 1;
        }
    }

    fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    fn count(&self) -> usize {
        self.count
    }
}
