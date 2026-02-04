use std::collections::HashMap;
use crate::model::enemy::Position;

/// Simple pathfinding cache to avoid recalculating paths
/// Caches the next step in a path for each (from, to) pair
#[derive(Clone, Debug, Default)]
pub struct PathfindingCache {
    // Cache key: (from_x, from_y, to_x, to_y)
    // Value: next position in optimal path
    cache: HashMap<(i32, i32, i32, i32), Option<(i32, i32)>>,
    max_size: usize,
}

impl PathfindingCache {
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: HashMap::new(),
            max_size,
        }
    }

    /// Get a cached path result
    pub fn get(&self, from: &Position, to: &Position) -> Option<Option<(i32, i32)>> {
        self.cache.get(&(from.x, from.y, to.x, to.y)).copied()
    }

    /// Cache a path result
    pub fn set(&mut self, from: &Position, to: &Position, next_step: Option<(i32, i32)>) {
        // Simple LRU - if cache is full, clear it
        if self.cache.len() >= self.max_size {
            self.cache.clear();
        }
        self.cache.insert((from.x, from.y, to.x, to.y), next_step);
    }

    /// Invalidate all cache entries (for when map changes)
    pub fn clear(&mut self) {
        self.cache.clear();
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        CacheStats {
            entries: self.cache.len(),
            max_size: self.max_size,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CacheStats {
    pub entries: usize,
    pub max_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pathfinding_cache() {
        let mut cache = PathfindingCache::new(100);
        let from = Position::new(0, 0);
        let to = Position::new(10, 10);

        // Initially empty
        assert!(cache.get(&from, &to).is_none());

        // Cache a result
        cache.set(&from, &to, Some((1, 0)));
        assert_eq!(cache.get(&from, &to), Some(Some((1, 0))));

        // Cache miss for different coordinates
        let other = Position::new(5, 5);
        assert!(cache.get(&from, &other).is_none());

        // Clear works
        cache.clear();
        assert!(cache.get(&from, &to).is_none());
    }

    #[test]
    fn test_cache_limit() {
        let mut cache = PathfindingCache::new(2);
        let pos1 = Position::new(0, 0);
        let pos2 = Position::new(1, 1);
        let pos3 = Position::new(2, 2);
        let pos4 = Position::new(3, 3);

        cache.set(&pos1, &pos2, Some((1, 0)));
        cache.set(&pos2, &pos3, Some((2, 0)));
        cache.set(&pos3, &pos4, Some((3, 0))); // This should trigger clear

        // After clear, old entries are gone
        assert!(cache.get(&pos1, &pos2).is_none());
    }
}
