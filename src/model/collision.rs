use std::collections::HashMap;

/// Simple spatial hash grid for efficient collision detection
/// Divides space into cells for faster proximity queries
#[derive(Clone, Debug)]
#[allow(dead_code)] // Used in tests, may be useful for future optimizations
pub struct SpatialHash {
    cell_size: i32,
    grid: HashMap<(i32, i32), Vec<(i32, i32)>>, // Map of cell coordinates to entity positions
}

impl SpatialHash {
    pub fn new(cell_size: i32) -> Self {
        Self {
            cell_size,
            grid: HashMap::new(),
        }
    }

    /// Get the grid cell coordinates for a world position
    fn get_cell(&self, x: i32, y: i32) -> (i32, i32) {
        (x / self.cell_size, y / self.cell_size)
    }

    /// Insert an entity at the given position
    pub fn insert(&mut self, x: i32, y: i32) {
        let cell = self.get_cell(x, y);
        self.grid.entry(cell).or_default().push((x, y));
    }

    /// Get all entities in the given radius around a position
    pub fn query_radius(&self, cx: i32, cy: i32, radius: i32) -> Vec<(i32, i32)> {
        let mut results = Vec::new();

        let min_cell = self.get_cell(cx - radius, cy - radius);
        let max_cell = self.get_cell(cx + radius, cy + radius);

        for grid_x in (min_cell.0)..=(max_cell.0) {
            for grid_y in (min_cell.1)..=(max_cell.1) {
                if let Some(entities) = self.grid.get(&(grid_x, grid_y)) {
                    for (ex, ey) in entities {
                        let dx = (*ex - cx).abs();
                        let dy = (*ey - cy).abs();
                        if dx * dx + dy * dy <= radius * radius {
                            results.push((*ex, *ey));
                        }
                    }
                }
            }
        }

        results
    }

    /// Clear all entities from the grid
    pub fn clear(&mut self) {
        self.grid.clear();
    }

    /// Get the number of entities in the hash
    pub fn len(&self) -> usize {
        self.grid.values().map(|v| v.len()).sum()
    }

    /// Check if the hash is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spatial_hash_basic() {
        let mut hash = SpatialHash::new(10);
        hash.insert(5, 5);
        hash.insert(15, 15);

        let near = hash.query_radius(5, 5, 5);
        assert!(near.contains(&(5, 5)));

        let far = hash.query_radius(50, 50, 5);
        assert!(far.is_empty());
    }

    #[test]
    fn test_spatial_hash_query() {
        let mut hash = SpatialHash::new(20);
        hash.insert(0, 0);
        hash.insert(5, 5);
        hash.insert(30, 30);

        let results = hash.query_radius(0, 0, 10);
        assert_eq!(results.len(), 2); // Should find (0,0) and (5,5)
        assert!(results.contains(&(0, 0)));
        assert!(results.contains(&(5, 5)));
    }
}
