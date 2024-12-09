// Note grid here includes the \n character at the end of row

use std::collections::HashMap;

/// Special value to denote a break in the grid, e.g. a line break when parsing horizontally
const BREAK: usize = usize::MAX;

struct Grid {
    values: String,
    width: usize,
    height: usize,
}

impl Grid {
    fn count_xmas_words(&self) -> usize {
        let str_bytes = self.values.as_bytes();
        let mut counter = XmasCounter::default();
        for direction in GridDirection::values() {
            for str_index in direction.to_grid_iterator(self.width, self.height) {
                if str_index == BREAK {
                    counter.break_count();
                } else {
                    let val = str_bytes[str_index] as char;
                    counter.take(val);
                }
            }
        }
        counter.count
    }
    fn count_mas_crosses(&self) -> usize {
        let str_bytes = self.values.as_bytes();
        let mut detector = MasDetector::default();
        for direction in [GridDirection::DiagonalA, GridDirection::DiagonalB] {
            println!("direction: {direction:?}");
            for str_index in direction.to_grid_iterator(self.width, self.height) {
                if str_index == BREAK {
                    detector.break_count();
                } else {
                    let val = str_bytes[str_index] as char;
                    detector.take(val, str_index);
                }
            }
        }
        detector.count()
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let row_length = value.find('\n').unwrap_or_default();
        let row_count = (value.len() + 1) / (row_length + 1);
        Self {
            values: value.replace("\n", ""),
            width: row_length,
            height: row_count,
        }
    }
}

#[derive(Default, Copy, Clone)]
struct XmasCounter {
    forward_state: u8,
    backward_state: u8,
    count: usize,
}

impl XmasCounter {
    fn take(&mut self, c: char) {
        self.forward_state = match (self.forward_state, c) {
            (_, 'X') => 1,
            (1, 'M') => 2,
            (2, 'A') => 3,
            (3, 'S') => {
                self.count += 1;
                0
            }
            _ => 0,
        };
        self.backward_state = match (self.backward_state, c) {
            (_, 'S') => 1,
            (1, 'A') => 2,
            (2, 'M') => 3,
            (3, 'X') => {
                self.count += 1;
                0
            }
            _ => 0,
        };
    }

    fn break_count(&mut self) {
        self.forward_state = 0;
        self.backward_state = 0;
    }
}

#[derive(Default)]
struct MasDetector {
    detections: Vec<usize>,
    forward_state: u8,
    forward_index: usize,
    backward_state: u8,
    backward_index: usize,
}

impl MasDetector {
    fn take(&mut self, c: char, idx: usize) {
        self.forward_state = match (self.forward_state, c) {
            (_, 'M') => 1,
            (1, 'A') => {
                self.forward_index = idx;
                2
            }
            (2, 'S') => {
                self.detections.push(self.forward_index);
                0
            }
            _ => 0,
        };
        self.backward_state = match (self.backward_state, c) {
            (_, 'S') => 1,
            (1, 'A') => {
                self.backward_index = idx;
                2
            }
            (2, 'M') => {
                self.detections.push(self.backward_index);
                0
            }
            _ => 0,
        }
    }

    fn count(&self) -> usize {
        self.detections
            .iter()
            .fold(HashMap::<usize, usize>::new(), |mut map, &x| {
                let entry = map.entry(x).or_default();
                *entry += 1;
                map
            })
            .iter()
            .filter(|(_, &v)| v == 2)
            .count()
    }

    fn break_count(&mut self) {
        self.forward_state = 0;
        self.backward_state = 0;
    }
}

struct GridCrawler {
    direction: GridDirection,
    row: usize,
    index: usize,
    width: usize,
    height: usize,
    counter: usize,
}

impl GridCrawler {
    fn new(size: (usize, usize), direction: GridDirection) -> Self {
        Self {
            direction,
            row: 0,
            index: 0,
            width: size.0,
            height: size.1,
            counter: 0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum GridDirection {
    Horizontal,
    Vertical,
    DiagonalA,
    DiagonalB,
}

impl GridDirection {
    fn values() -> [GridDirection; 4] {
        [
            GridDirection::Horizontal,
            GridDirection::Vertical,
            GridDirection::DiagonalA,
            GridDirection::DiagonalB,
        ]
    }

    /// returns an iterator over grid indexes to read, usize::MAX is used to denote a break
    fn to_grid_iterator(&self, width: usize, height: usize) -> Box<dyn Iterator<Item = usize>> {
        match self {
            GridDirection::Horizontal => {
                let v_width = width + 1;
                let grid_len = v_width * height; // virtual extra column
                Box::new((0..grid_len).map(move |idx| {
                    if (idx + 1) % v_width == 0 {
                        BREAK
                    } else {
                        idx - idx.div_euclid(v_width)
                    }
                }))
            }
            GridDirection::Vertical => {
                let v_height = height + 1;
                let grid_len = width * v_height; // virtual extra row
                Box::new((0..grid_len).map(move |idx| {
                    if (idx + 1) % v_height == 0 {
                        BREAK
                    } else {
                        (idx + 1).div_euclid(v_height) + (idx % (v_height)) * width
                    }
                }))
            }
            GridDirection::DiagonalA => {
                let v_width = width + 1;
                let v_height = height + 1;
                let grid_len = v_width * v_height; // virtual extra row and column
                let mut prev_val = 0;
                let offset = 1usize + (v_height - v_width);
                Box::new(
                    (0..grid_len)
                        .map(move |idx| {
                            let x = (-1 * (idx as i32) + (offset * (idx / v_height)) as i32)
                                .rem_euclid(v_width as i32)
                                as usize;
                            let y = idx % v_height;
                            if x + 1 == v_width || y + 1 == v_height {
                                BREAK
                            } else {
                                let mut val = x + y * v_width - x / v_width;
                                val = val - val / v_width; // compensate for virtual row
                                val
                            }
                        })
                        // filter out duplicate breaks to be aesthetic
                        .filter(move |&val| {
                            let skip = !(prev_val == BREAK && val == BREAK);
                            prev_val = val;
                            skip
                        }),
                )
            }
            GridDirection::DiagonalB => {
                let v_width = width + 1;
                let v_height = height + 1;
                let grid_len = v_width * v_height; // virtual extra row and column
                let mut prev_val = 0;
                Box::new(
                    (1..grid_len)
                        .map(move |idx| {
                            let x = ((v_width - 1 + idx % v_width) - idx / v_width) % v_width;
                            let y = idx % v_height;
                            if x + 1 == v_width || y + 1 == v_height {
                                BREAK
                            } else {
                                let mut val = x + y * v_width;
                                val = val - val / v_width; // compensate for virtual row
                                val
                            }
                        })
                        // filter out duplicate breaks to be aesthetic
                        .filter(move |&val| {
                            let skip = !(prev_val == BREAK && val == BREAK);
                            prev_val = val;
                            skip
                        }),
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{load_file, load_input_for_day};

    #[test]
    fn test_grid() {
        let grid = Grid::from("xmas\nsamx");
        assert_eq!(grid.width, 4);
        assert_eq!(grid.height, 2);
        assert_eq!(grid.values, "xmassamx".to_string());
    }

    #[test]
    fn test_grid_direction_iterator_horizontal() {
        let grid_indexes = GridDirection::Horizontal
            .to_grid_iterator(4, 2)
            .collect::<Vec<usize>>();
        assert_eq!(grid_indexes, vec![0, 1, 2, 3, BREAK, 4, 5, 6, 7, BREAK])
    }

    #[test]
    fn test_grid_direction_iterator_vertical() {
        let grid_indexes = GridDirection::Vertical
            .to_grid_iterator(2, 4)
            .collect::<Vec<usize>>();
        assert_eq!(grid_indexes, vec![0, 2, 4, 6, BREAK, 1, 3, 5, 7, BREAK])
    }

    #[test]
    fn test_grid_direction_iterator_diagonal_a() {
        let grid_indexes = GridDirection::DiagonalA
            .to_grid_iterator(4, 4)
            .collect::<Vec<usize>>();
        assert_eq!(
            grid_indexes,
            vec![
                0, BREAK, 11, 14, BREAK, 1, 4, BREAK, 15, BREAK, 2, 5, 8, BREAK, 3, 6, 9, 12,
                BREAK, 7, 10, 13, BREAK
            ]
        )
    }

    #[test]
    fn test_grid_direction_iterator_diagonal_a_2() {
        let grid_indexes = GridDirection::DiagonalA
            .to_grid_iterator(3, 4)
            .collect::<Vec<usize>>();
        assert_eq!(
            grid_indexes,
            vec![0, BREAK, 8, 10, BREAK, 1, 3, BREAK, 11, BREAK, 2, 4, 6, BREAK, 5, 7, 9, BREAK]
        )
    }

    #[test]
    fn test_grid_direction_iterator_diagonal_b() {
        let grid_indexes = GridDirection::DiagonalB
            .to_grid_iterator(4, 4)
            .collect::<Vec<usize>>();
        assert_eq!(
            grid_indexes,
            vec![
                4, 9, 14, BREAK, 3, BREAK, 8, 13, BREAK, 2, 7, BREAK, 12, BREAK, 1, 6, 11, BREAK,
                0, 5, 10, 15, BREAK
            ]
        )
    }

    #[test]
    fn test_xmas_counter() {
        let test_cases: [(&str, usize); 5] = [
            ("XMAS", 1),
            ("X", 0),
            ("SAMX", 1),
            ("XMASAMX", 2),
            ("XMASX", 1),
        ];
        for (input, expected_count) in &test_cases {
            let mut counter = XmasCounter::default();
            input.chars().for_each(|c| counter.take(c));
            assert_eq!(counter.count, *expected_count, "test_case: {input}");
        }
    }

    #[test]
    fn test_grid_small() {
        let raw_grid = load_file(4, "input_test_4x4.txt");
        let grid = Grid::from(raw_grid.as_str());
        let count = grid.count_xmas_words();
        assert_eq!(count, 6);
    }

    #[test]
    fn test_grid_small_mas() {
        let raw_grid = load_file(4, "input_test_4x4.txt");
        let grid = Grid::from(raw_grid.as_str());
        let count = grid.count_mas_crosses();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_example_grid() {
        let raw_grid = load_file(4, "input_test.txt");
        let grid = Grid::from(raw_grid.as_str());
        let count = grid.count_xmas_words();
        assert_eq!(count, 18);
    }

    #[test]
    fn test_example_grid_mas_cross() {
        let raw_grid = load_file(4, "input_test.txt");
        let grid = Grid::from(raw_grid.as_str());
        let count = grid.count_mas_crosses();
        assert_eq!(count, 9);
    }

    #[test]
    fn calculate_pt_1() {
        let raw_grid = load_input_for_day(4);
        let grid = Grid::from(raw_grid.as_str());
        let count = grid.count_xmas_words();
        assert_eq!(count, 2468);
    }

    #[test]
    fn calculate_pt_2() {
        let raw_grid = load_input_for_day(4);
        let grid = Grid::from(raw_grid.as_str());
        let count = grid.count_mas_crosses();
        assert_eq!(count, 1864);
    }
}
