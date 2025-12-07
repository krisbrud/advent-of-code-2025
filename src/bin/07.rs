advent_of_code::solution!(7);

use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

const SPLITTER: char = '^';
const START: char = 'S';
const BEAM: char = '|';
const EMPTY: char = '.';

type Coord = (usize, usize);

struct Board {
    tiles: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Board {
    fn new(s: &str) -> Option<Board> {
        let tiles: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();

        let rows = s.lines().collect_vec().len();
        let cols = s.lines().nth(0).map(|x| x.len())?;

        Some(Board { tiles, rows, cols })
    }

    // Assume bounds check has already been done
    fn get(&self, coordinate: &Coord) -> char {
        self.tiles[coordinate.0][coordinate.1]
    }

    fn maybe_tile(&self, coordinate: &Coord) -> Option<char> {
        if coordinate.0 < self.rows && coordinate.1 < self.cols {
            Some(self.get(coordinate))
        } else {
            None
        }
    }

    fn coord_below(&self, coordinate: &Coord) -> Option<Coord> {
        if coordinate.0 < self.rows - 1 {
            Some((coordinate.0 + 1, coordinate.1))
        } else {
            None
        }
    }

    fn shoulder_coords(&self, coordinate: &Coord) -> Vec<Coord> {
        let mut out = vec![];
        if coordinate.1 > 0 {
            out.push((coordinate.0, coordinate.1 - 1));
        }
        if coordinate.1 < self.cols - 1 {
            out.push((coordinate.0, coordinate.1 + 1));
        }
        out
    }
}

fn find_start(board: &Board) -> Option<Coord> {
    let start_col = board.tiles[0]
        .iter()
        .find_position(|c| **c == START)
        .map(|tup| tup.0)?;
    Some((0, start_col))
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut num_splits = 0;

    let board = Board::new(input)?;
    let mut queue: VecDeque<Coord> = VecDeque::new();
    let mut seen: HashSet<Coord> = HashSet::new();

    // Start by locating the position of S
    let start_coord = find_start(&board)?;
    // Add S (or its position?) to the stack or queue
    queue.push_front(start_coord);
    seen.insert(start_coord);

    // While there are positions in the queue
    while let Some(current_coord) = queue.pop_front() {
        let tile = board.maybe_tile(&current_coord).expect("Should find tile");
        match tile {
            SPLITTER => {
                num_splits += 1;
                let shoulder_coords = board.shoulder_coords(&current_coord);
                let unseen_sides = shoulder_coords
                    .iter()
                    .filter(|coord| !seen.contains(coord))
                    .collect_vec();
                for side in unseen_sides {
                    seen.insert(*side);
                    queue.push_front(*side);
                }
            }
            EMPTY | START => {
                if let Some(coord_below) = board.coord_below(&current_coord) {
                    if !seen.contains(&coord_below) {
                        seen.insert(coord_below);
                        queue.push_front(coord_below);
                    }
                }
            }
            _ => {
                panic!("Unexpected tile {}", tile)
            }
        }
    }
    Some(num_splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_start() {
        let start_coord =
            find_start(&Board::new(&advent_of_code::template::read_file("examples", DAY)).unwrap());
        assert_eq!(start_coord, Some((0, 7)));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
