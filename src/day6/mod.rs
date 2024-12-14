use log::error;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

impl Orientation {
    fn dx(&self) -> i32 {
        match self {
            Orientation::Up | Orientation::Down => 0,
            Orientation::Left => -1,
            Orientation::Right => 1,
        }
    }
    fn dy(&self) -> i32 {
        match self {
            Orientation::Up => -1,
            Orientation::Down => 1,
            Orientation::Left | Orientation::Right => 0,
        }
    }

    fn rotate_clockwise(&self) -> Orientation {
        match self {
            Orientation::Up => Orientation::Right,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
            Orientation::Right => Orientation::Down,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum CellState {
    Clear,
    Obstruction,
    Visited { orientations: [usize; 4] },
}

impl CellState {
    fn is_visited(&self) -> bool {
        std::mem::discriminant(self)
            == std::mem::discriminant(&CellState::Visited {
                orientations: [0; 4],
            })
    }
}

struct Grid {
    cells: Vec<CellState>,
    width: usize,
    height: usize,
    guard_original_x: i32,
    guard_original_y: i32,
    guard_x: i32,
    guard_y: i32,
    guard_orientation: Orientation,
    is_consumed: bool,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let width = value.find('\n').unwrap_or_default();
        let height = (value.len() + 1) / (width + 1);
        let guard_index = value
            .replace('\n', "")
            .chars()
            .enumerate()
            .find(|(_, c)| *c == '^')
            .unwrap()
            .0;
        let guard_x = guard_index.rem_euclid(width) as i32;
        let guard_y = guard_index.div_euclid(width) as i32;
        Self {
            cells: value
                .replace('\n', "")
                .chars()
                .map(|c| match c {
                    '#' => CellState::Obstruction,
                    '^' => CellState::Visited {
                        orientations: [1, 0, 0, 0],
                    },
                    _ => CellState::Clear,
                })
                .collect(),
            width,
            height,
            guard_x,
            guard_y,
            guard_original_x: guard_x,
            guard_original_y: guard_y,
            guard_orientation: Orientation::Up,
            is_consumed: false,
        }
    }
}

impl Grid {
    fn is_blocked(&self, x: i32, y: i32) -> bool {
        self.cells[(y * self.width as i32 + x) as usize] == CellState::Obstruction
    }

    fn is_in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }

    fn can_block(&self, x: i32, y: i32) -> bool {
        println!("attempting to block ({:02}, {:02})", x, y);
        !self.is_blocked(x, y)
            && self.is_in_bounds(x, y)
            && !(x == self.guard_original_x && y == self.guard_original_y)
    }

    fn can_join_segment(
        &self,
        mut x: i32,
        mut y: i32,
        orientation: Orientation,
        segment_orientation: Orientation,
        step_time: usize,
    ) -> bool {
        loop {
            x += orientation.dx();
            y += orientation.dy();
            if !self.is_in_bounds(x, y) {
                return false;
            }
            match self.cells[(y * self.width as i32 + x) as usize] {
                CellState::Clear => {}
                CellState::Obstruction => {
                    return false;
                }
                CellState::Visited { orientations } => {
                    let cell_orientation_time = orientations[segment_orientation as usize];
                    if cell_orientation_time > 0 && cell_orientation_time < step_time {
                        return true;
                    }
                }
            }
        }
    }

    fn walk(&mut self) {
        if self.is_consumed {
            return;
        }
        self.is_consumed = true;
        let mut step_time = 1;
        loop {
            let next_x = self.guard_x + self.guard_orientation.dx();
            let next_y = self.guard_y + self.guard_orientation.dy();
            step_time += 1;
            if !self.is_in_bounds(next_x, next_y) {
                break;
            } else if self.is_blocked(next_x, next_y) {
                self.guard_orientation = self.guard_orientation.rotate_clockwise();
            } else {
                self.guard_x = next_x;
                self.guard_y = next_y;
                self.visit(
                    self.guard_x,
                    self.guard_y,
                    self.guard_orientation,
                    step_time,
                );
            }
        }
    }

    fn count_visited(&self) -> usize {
        self.cells.iter().map(|c| c.is_visited() as usize).sum()
    }

    fn count_obstructable(&self) -> usize {
        let mut blockable: HashSet<(i32, i32)> = HashSet::new();
        self.cells.iter().enumerate().for_each(|(idx, c)| match c {
            CellState::Visited { orientations } => {
                let x = idx.rem_euclid(self.width) as i32;
                let y = idx.div_euclid(self.width) as i32;
                println!("({:02}, {:02}) | orientations: {:?}", x, y, orientations);
                let [up, down, left, right] = *orientations;
                println!("up");
                if up > 0
                    && (right > up
                        || self.can_join_segment(x, y - 1, Orientation::Up, Orientation::Right, up))
                    && self.can_block(x, y - 1)
                {
                    println!("blocking ({:02}, {:02})", x, y - 1);
                    blockable.insert((x, y - 1));
                };
                println!("right");
                if right > 0
                    && (down > right
                        || self.can_join_segment(
                            x + 1,
                            y,
                            Orientation::Right,
                            Orientation::Down,
                            right,
                        ))
                    && self.can_block(x + 1, y)
                {
                    println!("blocking ({:02}, {:02})", x + 1, y);
                    blockable.insert((x + 1, y));
                }
                println!("down");
                if down > 0
                    && (left > down
                        || self.can_join_segment(
                            x,
                            y + 1,
                            Orientation::Down,
                            Orientation::Left,
                            down,
                        ))
                    && self.can_block(x, y + 1)
                {
                    println!("blocking ({:02}, {:02})", x, y + 1);
                    blockable.insert((x, y + 1));
                }
                println!("left");
                if left > 0
                    && (up > left
                        || self.can_join_segment(
                            x - 1,
                            y,
                            Orientation::Left,
                            Orientation::Up,
                            left,
                        ))
                    && self.can_block(x - 1, y)
                {
                    println!("blocking ({:02}, {:02})", x - 1, y);
                    blockable.insert((x - 1, y));
                }
            }
            _ => {}
        });
        println!("{:?}", blockable);
        blockable.len()
    }

    fn visit(&mut self, x: i32, y: i32, orientation: Orientation, step_time: usize) {
        let idx = (y * self.width as i32 + x) as usize;
        let cell = self.cells[idx];
        match cell {
            CellState::Clear => {
                let mut orientations = [0; 4];
                orientations[orientation as usize] = step_time;
                self.cells[idx] = CellState::Visited { orientations };
            }
            CellState::Obstruction => {
                error!("tried to visit obstructed cell at ({:02}, {:02})", x, y);
            }
            CellState::Visited { mut orientations } => {
                if orientations[orientation as usize] == 0 {
                    orientations[orientation as usize] = step_time
                };
                self.cells[idx] = CellState::Visited { orientations };
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::day6::Grid;
    use crate::utils::{load_file, load_input_for_day};

    #[test]
    fn test_grid_is_blocked() {
        let raw_grid = load_file(6, "input_test.txt");
        let grid = Grid::from(raw_grid.as_str());
        assert!(!grid.is_blocked(0, 0));
        assert!(!grid.is_blocked(9, 9));
        assert!(grid.is_blocked(0, 8));
        assert!(grid.is_blocked(6, 9));
        assert!(grid.is_blocked(9, 1));
    }
    #[test]
    fn test_grid_in_bounds() {
        let raw_grid = load_file(6, "input_test.txt");
        let grid = Grid::from(raw_grid.as_str());
        assert!(grid.is_in_bounds(0, 0));
        assert!(grid.is_in_bounds(9, 9));
        assert!(!grid.is_in_bounds(-1, 0));
        assert!(!grid.is_in_bounds(0, -1));
        assert!(!grid.is_in_bounds(10, 0));
        assert!(!grid.is_in_bounds(0, 10));
    }

    #[test]
    fn test_parse() {
        let raw_grid = load_file(6, "input_test.txt");
        let grid = Grid::from(raw_grid.as_str());
        assert_eq!(grid.guard_x, 4);
        assert_eq!(grid.guard_y, 6);
    }

    #[test]
    fn calculate_pt_1_test() {
        let raw_grid = load_file(6, "input_test.txt");
        let mut grid = Grid::from(raw_grid.as_str());
        grid.walk();
        assert_eq!(grid.count_visited(), 41);
    }

    #[test]
    fn calculate_pt_1() {
        let raw_grid = load_input_for_day(6);
        let mut grid = Grid::from(raw_grid.as_str());
        grid.walk();
        assert_eq!(grid.count_visited(), 4722);
    }

    #[test]
    fn calculate_pt_2_test() {
        let raw_grid = load_file(6, "input_test.txt");
        let mut grid = Grid::from(raw_grid.as_str());
        grid.walk();
        assert_eq!(grid.count_obstructable(), 6);
    }

    #[test]
    fn calculate_pt_2() {
        let raw_grid = load_input_for_day(6);
        let mut grid = Grid::from(raw_grid.as_str());
        grid.walk();
        assert_eq!(grid.count_obstructable(), 2588);
    }

    #[test]
    fn calculate_edge_case() {
        let mut grid = Grid::from(".#\n.^");
        grid.walk();
        assert_eq!(grid.count_visited(), 1);
    }
}
