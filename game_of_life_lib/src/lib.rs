use std::collections::HashSet;
use std::hash::Hash;
use std::iter::Iterator;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

pub fn neighbors(point: Point, x_max: usize, y_max: usize) -> HashSet<Point> {
    let mut result = HashSet::new();

    // Check all 8 directions
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue; // Skip self
            }

            // Calculate neighbor coordinates (using i32 for safe subtraction)
            let nx = point.x as i32 + dx;
            let ny = point.y as i32 + dy;

            // Check bounds: coordinates must be >= 0 and <= max
            if nx >= 0 && ny >= 0 {
                let nx_usize = nx as usize;
                let ny_usize = ny as usize;

                if nx_usize <= x_max && ny_usize <= y_max {
                    result.insert(Point::new(nx_usize, ny_usize));
                }
            }
        }
    }

    result
}

/// The core state for Conway's Game of Life
pub(crate) struct GameState {
    pub x_max: usize,
    pub y_max: usize,
    pub live_cells: HashSet<Point>,
}

impl GameState {
    pub fn new(x_max: usize, y_max: usize, initial_live_cells: HashSet<Point>) -> Self {
        Self {
            x_max,
            y_max,
            live_cells: initial_live_cells,
        }
    }

    pub fn points_to_evaluate(&self) -> HashSet<Point> {
        let mut points = self.live_cells.clone();

        for &cell in &self.live_cells {
            let cell_neighbors = neighbors(cell, self.x_max, self.y_max);
            points.extend(cell_neighbors);
        }

        points
    }
}

impl Iterator for GameState {
    type Item = HashSet<Point>;

    fn next(&mut self) -> Option<Self::Item> {
        let points_to_evaluate = self.points_to_evaluate();

        Some(self.live_cells.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_creation() {
        let p = Point::new(5, 10);
        assert_eq!(p.x, 5);
        assert_eq!(p.y, 10);
    }

    #[test]
    fn test_neighbors_center() {
        // Point in the middle of the grid should have all 8 neighbors
        let p = Point::new(5, 5);
        let neighbors_set = neighbors(p, 10, 10);

        assert_eq!(neighbors_set.len(), 8);

        // Check some expected neighbors
        assert!(neighbors_set.contains(&Point::new(4, 4)));
        assert!(neighbors_set.contains(&Point::new(4, 5)));
        assert!(neighbors_set.contains(&Point::new(4, 6)));
        assert!(neighbors_set.contains(&Point::new(5, 4)));
        assert!(neighbors_set.contains(&Point::new(5, 6)));
        assert!(neighbors_set.contains(&Point::new(6, 4)));
        assert!(neighbors_set.contains(&Point::new(6, 5)));
        assert!(neighbors_set.contains(&Point::new(6, 6)));

        // Self should NOT be included
        assert!(!neighbors_set.contains(&Point::new(5, 5)));
    }

    #[test]
    fn test_neighbors_corner() {
        // Corner point should have fewer neighbors (3)
        let p = Point::new(0, 0);
        let neighbors_set = neighbors(p, 10, 10);

        assert_eq!(neighbors_set.len(), 3);
        assert!(neighbors_set.contains(&Point::new(0, 1)));
        assert!(neighbors_set.contains(&Point::new(1, 0)));
        assert!(neighbors_set.contains(&Point::new(1, 1)));
    }

    #[test]
    fn test_neighbors_edge() {
        // Edge point should have 5 neighbors
        let p = Point::new(0, 5);
        let neighbors_set = neighbors(p, 10, 10);

        assert_eq!(neighbors_set.len(), 5);
    }

    #[test]
    fn test_neighbors_bounds_enforced() {
        // Neighbors beyond bounds should be filtered
        let p = Point::new(10, 10);
        let neighbors_set = neighbors(p, 10, 10);

        // Only 3 neighbors fit within bounds
        assert_eq!(neighbors_set.len(), 3);
        assert!(neighbors_set.contains(&Point::new(9, 9)));
        assert!(neighbors_set.contains(&Point::new(9, 10)));
        assert!(neighbors_set.contains(&Point::new(10, 9)));
    }

    #[test]
    fn test_gamestate_creation() {
        let initial_cells: HashSet<Point> =
            vec![Point::new(1, 1), Point::new(2, 2), Point::new(0, 0)]
                .into_iter()
                .collect();

        let state = GameState::new(10, 10, initial_cells.clone());

        assert_eq!(state.x_max, 10);
        assert_eq!(state.y_max, 10);
        assert_eq!(&state.live_cells, &initial_cells);
    }

    #[test]
    fn test_points_to_evaluate() {
        // Two cells at (1,1) and (3,3)
        let initial_cells: HashSet<Point> = vec![Point::new(1, 1), Point::new(3, 3)]
            .into_iter()
            .collect();

        let state = GameState::new(5, 5, initial_cells);
        let points = state.points_to_evaluate();

        // Should include both live cells
        assert!(points.contains(&Point::new(1, 1)));
        assert!(points.contains(&Point::new(3, 3)));

        // Should include neighbors of (1,1) - 8 neighbors
        assert!(points.contains(&Point::new(0, 0)));
        assert!(points.contains(&Point::new(0, 1)));
        assert!(points.contains(&Point::new(0, 2)));
        assert!(points.contains(&Point::new(1, 0)));
        assert!(points.contains(&Point::new(1, 2)));
        assert!(points.contains(&Point::new(2, 0)));
        assert!(points.contains(&Point::new(2, 1)));
        assert!(points.contains(&Point::new(2, 2)));

        // Should include neighbors of (3,3) - 8 neighbors
        for x in 2..=4 {
            for y in 2..=4 {
                if x != 3 || y != 3 {
                    assert!(
                        points.contains(&Point::new(x, y)),
                        "Should contain ({}, {})",
                        x,
                        y
                    );
                }
            }
        }
    }
}
