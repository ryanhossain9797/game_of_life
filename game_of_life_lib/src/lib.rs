use std::collections::HashSet;
use std::hash::Hash;
use std::iter::Iterator;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

/// The core state for Conway's Game of Life
struct GameState {
    pub x_max: usize,
    pub y_max: usize,
    pub live_cells: HashSet<Point>,
}

impl GameState {
    /// Creates a new GameState with the given dimensions and initial live cells
    ///
    /// # Arguments
    ///
    /// * `x_max` - Maximum x coordinate (inclusive)
    /// * `y_max` - Maximum y coordinate (inclusive)
    /// * `initial_live_cells` - Set of initial live cell positions
    fn new(x_max: usize, y_max: usize, initial_live_cells: HashSet<Point>) -> Self {
        Self {
            x_max,
            y_max,
            live_cells: initial_live_cells,
        }
    }
}

impl Iterator for GameState {
    type Item = HashSet<Point>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(HashSet::<Point>::new())
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
    fn test_gamestate_creation() {
        let initial_cells = vec![Point::new(1, 1), Point::new(2, 2), Point::new(0, 0)]
            .into_iter()
            .collect::<HashSet<Point>>();

        let state = GameState::new(10, 10, initial_cells.clone());

        assert_eq!(state.x_max, 10);
        assert_eq!(state.y_max, 10);
        assert_eq!(&state.live_cells, &initial_cells);
    }

    #[test]
    fn test_gamestate_iterator_stub() {
        let initial_cells = vec![Point::new(1, 1)]
            .into_iter()
            .collect::<HashSet<Point>>();
        let mut state = GameState::new(5, 5, initial_cells);

        // Currently returns None (stub implementation)
        assert_eq!(state.next(), None);
    }
}
