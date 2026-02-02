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

    fn get_tile(&self, x: i32, y: i32) -> bool {
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
            let msg = format!("is_walkable({}, {}) = false (out of bounds)\n", x, y);
            let _ = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("log.txt")
                .and_then(|mut f| std::io::Write::write_all(&mut f, msg.as_bytes()));
            return false;
        }

        let tile_val = self.get_tile(x, y);
        let walkable = !tile_val;
        let msg = format!(
            "is_walkable({}, {}) = {} (tile={}, is_wall={})\n",
            x,
            y,
            walkable,
            self.tiles[(y * self.width + x) as usize],
            tile_val
        );
        let _ = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("log.txt")
            .and_then(|mut f| std::io::Write::write_all(&mut f, msg.as_bytes()));
        walkable
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

        Some((ch, style))
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
