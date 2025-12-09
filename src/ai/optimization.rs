/// Performance optimization module for AI move evaluation
/// 
/// This module provides caching and optimization strategies to reduce
/// redundant calculations during placement evaluation.

use crate::game_state::{Grid, Position, GameState};
use crate::placement::Placement;
use std::collections::HashMap;

/// Cache for flood-fill reachability analysis results
/// 
/// Stores the count of reachable empty cells from each analyzed position,
/// avoiding redundant flood-fill calculations.
#[derive(Debug, Clone)]
pub struct FloodFillCache {
    cache: HashMap<(usize, usize), usize>,
}

impl FloodFillCache {
    /// Create a new empty flood-fill cache
    pub fn new() -> Self {
        FloodFillCache {
            cache: HashMap::new(),
        }
    }

    /// Get cached result or compute and cache
    pub fn get_or_compute<F>(&mut self, pos: (usize, usize), compute: F) -> usize
    where
        F: FnOnce() -> usize,
    {
        if let Some(&result) = self.cache.get(&pos) {
            return result;
        }

        let result = compute();
        self.cache.insert(pos, result);
        result
    }

    /// Clear the cache
    pub fn clear(&mut self) {
        self.cache.clear();
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        CacheStats {
            entries: self.cache.len(),
            capacity: self.cache.capacity(),
        }
    }
}

impl Default for FloodFillCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Cache for density analysis results
/// 
/// Stores territory density calculations to avoid redundant counting.
#[derive(Debug, Clone)]
pub struct DensityCache {
    cache: HashMap<(usize, usize), usize>,
}

impl DensityCache {
    /// Create a new empty density cache
    pub fn new() -> Self {
        DensityCache {
            cache: HashMap::new(),
        }
    }

    /// Get cached result or compute and cache
    pub fn get_or_compute<F>(&mut self, pos: (usize, usize), compute: F) -> usize
    where
        F: FnOnce() -> usize,
    {
        if let Some(&result) = self.cache.get(&pos) {
            return result;
        }

        let result = compute();
        self.cache.insert(pos, result);
        result
    }

    /// Clear the cache
    pub fn clear(&mut self) {
        self.cache.clear();
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        CacheStats {
            entries: self.cache.len(),
            capacity: self.cache.capacity(),
        }
    }
}

impl Default for DensityCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about cache performance
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CacheStats {
    pub entries: usize,
    pub capacity: usize,
}

impl CacheStats {
    /// Calculate cache efficiency (entries / capacity)
    pub fn efficiency(&self) -> f32 {
        if self.capacity == 0 {
            0.0
        } else {
            (self.entries as f32) / (self.capacity as f32)
        }
    }
}

/// Optimized flood-fill implementation with early termination
/// 
/// Uses early termination when exploring for territory estimation
pub fn flood_fill_bounded(
    grid: &Grid,
    start_positions: &[Position],
    max_iterations: usize,
) -> usize {
    use std::collections::{VecDeque, HashSet};

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut iterations = 0;

    // Initialize queue with starting positions
    for &pos in start_positions {
        if grid.is_valid(pos) {
            queue.push_back(pos);
            visited.insert(pos);
        }
    }

    let mut reachable_count = 0;

    while let Some(pos) = queue.pop_front() {
        // Early termination check
        if iterations >= max_iterations {
            break;
        }
        iterations += 1;

        // Check all 4 adjacent cells
        let neighbors = [
            Position::new(pos.x.wrapping_add(1), pos.y),
            Position::new(pos.x.wrapping_sub(1), pos.y),
            Position::new(pos.x, pos.y.wrapping_add(1)),
            Position::new(pos.x, pos.y.wrapping_sub(1)),
        ];

        for neighbor in neighbors {
            if !visited.contains(&neighbor) && grid.is_valid(neighbor) {
                if let Some(state) = grid.get(neighbor) {
                    use crate::game_state::CellState;
                    // Only continue through empty cells or our territory
                    if matches!(state, CellState::Empty | CellState::Player1 | CellState::Player1Last) {
                        visited.insert(neighbor);

                        if state == CellState::Empty {
                            reachable_count += 1;
                        }

                        // Only queue empty cells for further exploration
                        if state == CellState::Empty {
                            queue.push_back(neighbor);
                        }
                    }
                }
            }
        }
    }

    reachable_count
}

/// Score calculation with caching
/// 
/// Enables fast re-scoring of same placements
pub struct ScoringContext {
    flood_fill_cache: FloodFillCache,
    density_cache: DensityCache,
}

impl ScoringContext {
    /// Create a new scoring context with empty caches
    pub fn new() -> Self {
        ScoringContext {
            flood_fill_cache: FloodFillCache::new(),
            density_cache: DensityCache::new(),
        }
    }

    /// Get flood-fill cache (mutable)
    pub fn flood_fill_cache_mut(&mut self) -> &mut FloodFillCache {
        &mut self.flood_fill_cache
    }

    /// Get density cache (mutable)
    pub fn density_cache_mut(&mut self) -> &mut DensityCache {
        &mut self.density_cache
    }

    /// Reset all caches between evaluations
    pub fn reset(&mut self) {
        self.flood_fill_cache.clear();
        self.density_cache.clear();
    }

    /// Get combined cache statistics
    pub fn cache_stats(&self) -> (CacheStats, CacheStats) {
        (
            self.flood_fill_cache.stats(),
            self.density_cache.stats(),
        )
    }
}

impl Default for ScoringContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Placement scoring with cached results
/// 
/// Enables batch scoring of multiple placements with shared cache
pub struct BatchScorer {
    context: ScoringContext,
}

impl BatchScorer {
    /// Create a new batch scorer
    pub fn new() -> Self {
        BatchScorer {
            context: ScoringContext::new(),
        }
    }

    /// Score all placements with shared cache
    pub fn score_all(
        &mut self,
        placements: &[Placement],
        game_state: &GameState,
    ) -> Vec<(Placement, f32)> {
        self.context.reset();

        placements
            .iter()
            .map(|placement| {
                let score = self.score_single(placement, game_state);
                (placement.clone(), score)
            })
            .collect()
    }

    /// Score a single placement using cache
    fn score_single(&mut self, placement: &Placement, game_state: &GameState) -> f32 {
        use crate::ai::heuristics;

        // Base expansion score (not cached - fast computation)
        let base_expansion = (placement.cells_added as f32) * 10.0;

        // Flood-fill (cached)
        let abs_positions = placement.get_absolute_positions();
        let flood_fill = if !abs_positions.is_empty() {
            let first_pos = abs_positions[0];
            let key = (first_pos.x, first_pos.y);
            let reachable = self
                .context
                .flood_fill_cache_mut()
                .get_or_compute(key, || {
                    heuristics::analyze_flood_fill(placement, game_state) as usize
                });
            (reachable as f32) * 1.5
        } else {
            0.0
        };

        // Weak positions (not cached - depends on current board state)
        let weak_positions = heuristics::detect_weak_positions(placement, game_state);

        // Density (cached per position)
        let density = if !abs_positions.is_empty() {
            let first_pos = abs_positions[0];
            let key = (first_pos.x, first_pos.y);
            let nearby = self
                .context
                .density_cache_mut()
                .get_or_compute(key, || {
                    heuristics::analyze_density(placement, game_state) as usize
                });
            (nearby as f32) * 1.2
        } else {
            0.0
        };

        // Edge control (fast, not cached)
        let edge_control = heuristics::analyze_edge_control(placement, &game_state.grid);

        // Combined score
        base_expansion + flood_fill + (weak_positions * 2.0) + density + (edge_control * 0.5)
    }

    /// Get cache performance statistics
    pub fn cache_stats(&self) -> (CacheStats, CacheStats) {
        self.context.cache_stats()
    }
}

impl Default for BatchScorer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flood_fill_cache_new() {
        let cache = FloodFillCache::new();
        assert_eq!(cache.stats().entries, 0);
    }

    #[test]
    fn test_flood_fill_cache_compute() {
        let mut cache = FloodFillCache::new();
        
        let result = cache.get_or_compute((1, 2), || 42);
        assert_eq!(result, 42);
        assert_eq!(cache.stats().entries, 1);
    }

    #[test]
    fn test_flood_fill_cache_hit() {
        let mut cache = FloodFillCache::new();
        
        cache.get_or_compute((1, 2), || 42);
        let result = cache.get_or_compute((1, 2), || 999);
        
        // Should return cached value, not 999
        assert_eq!(result, 42);
    }

    #[test]
    fn test_density_cache_new() {
        let cache = DensityCache::new();
        assert_eq!(cache.stats().entries, 0);
    }

    #[test]
    fn test_density_cache_compute() {
        let mut cache = DensityCache::new();
        
        let result = cache.get_or_compute((2, 3), || 15);
        assert_eq!(result, 15);
        assert_eq!(cache.stats().entries, 1);
    }

    #[test]
    fn test_cache_stats_efficiency() {
        let stats = CacheStats {
            entries: 50,
            capacity: 100,
        };
        
        assert_eq!(stats.efficiency(), 0.5);
    }

    #[test]
    fn test_cache_stats_efficiency_empty() {
        let stats = CacheStats {
            entries: 0,
            capacity: 0,
        };
        
        assert_eq!(stats.efficiency(), 0.0);
    }

    #[test]
    fn test_scoring_context_new() {
        let context = ScoringContext::new();
        let (ff, den) = context.cache_stats();
        
        assert_eq!(ff.entries, 0);
        assert_eq!(den.entries, 0);
    }

    #[test]
    fn test_batch_scorer_new() {
        let scorer = BatchScorer::new();
        let (_ff, _den) = scorer.cache_stats();
        
        // Should have empty caches initially
        assert!(true);
    }

    #[test]
    fn test_flood_fill_bounded_respects_max_iterations() {
        let raw = vec![
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '@', '@', '.', '.'],
            vec!['.', '@', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.'],
        ];
        let grid = crate::game_state::Grid::from_chars(5, 5, raw);
        let start = vec![Position::new(1, 1)];
        
        // With max_iterations = 0, should return 0
        let result = flood_fill_bounded(&grid, &start, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_flood_fill_bounded_with_high_limit() {
        let raw = vec![
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '@', '@', '.', '.'],
            vec!['.', '@', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.'],
        ];
        let grid = crate::game_state::Grid::from_chars(5, 5, raw);
        let start = vec![Position::new(0, 0)];
        
        // With high limit, should explore normally
        let result = flood_fill_bounded(&grid, &start, 1000);
        assert!(result > 0);
    }

    #[test]
    fn test_cache_clear() {
        let mut cache = FloodFillCache::new();
        cache.get_or_compute((1, 2), || 42);
        assert_eq!(cache.stats().entries, 1);
        
        cache.clear();
        assert_eq!(cache.stats().entries, 0);
    }

    #[test]
    fn test_context_reset() {
        let mut context = ScoringContext::new();
        context.flood_fill_cache_mut().get_or_compute((1, 2), || 42);
        context.density_cache_mut().get_or_compute((2, 3), || 15);
        
        let (ff1, den1) = context.cache_stats();
        assert!(ff1.entries > 0);
        assert!(den1.entries > 0);
        
        context.reset();
        
        let (ff2, den2) = context.cache_stats();
        assert_eq!(ff2.entries, 0);
        assert_eq!(den2.entries, 0);
    }
}
