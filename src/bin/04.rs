advent_of_code::solution!(4);

use itertools::Itertools;

type Coord = (usize, usize);

struct Board {
    tiles: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

const FREE_TILE: char = '.';
const ROLL_TILE: char = '@';

impl Board {
    fn new(s: &str) -> Option<Board> {
        let plants: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();

        let rows = s.lines().collect_vec().len();
        let cols = s.lines().nth(0).map(|x| x.len())?;

        Some(Board {
            tiles: plants,
            rows,
            cols,
        })
    }

    // Assume bounds check has already been done
    fn get(&self, coordinate: &Coord) -> char {
        self.tiles[coordinate.0][coordinate.1]
    }

    fn new_without(&self, removable: Vec<Coord>) -> Board {
        let mut new_tiles: Vec<Vec<char>> = self.tiles.clone();

        for coord in removable {
            new_tiles[coord.0][coord.1] = FREE_TILE;
        }

        Board {
            tiles: new_tiles,
            rows: self.rows,
            cols: self.cols,
        }
    }

    fn maybe_tile(&self, coordinate: &Coord) -> Option<char> {
        if coordinate.0 < self.rows && coordinate.1 < self.cols {
            Some(self.get(coordinate))
        } else {
            None
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let board = Board::new(input)?;

    let mut accessible = 0;

    for r in 0..board.rows as i32 {
        // println!("foo");
        for c in 0..board.cols as i32 {
            if board.get(&(r as usize, c as usize)) != ROLL_TILE {
                // Not a roll, no need to count
                continue;
            }
            let mut occupied_adj = 0;
            for dr in -1i32..=1 {
                // println!("baz");
                for dc in -1i32..=1 {
                    // println!("ham");
                    if dr == 0 && dc == 0 {
                        continue; // Same roll
                    }

                    let maybe_row = r + dr;
                    let maybe_col = c + dc;

                    if maybe_row < 0 || maybe_col < 0 {
                        continue;
                    }

                    let row = maybe_row as usize;
                    let col = maybe_col as usize;
                    let coordinate = (row, col);

                    if let Some(neigh) = board.maybe_tile(&coordinate) {
                        if neigh != FREE_TILE {
                            occupied_adj += 1;
                        }
                        // Otherwise free
                    } else {
                        // Out of board, don't count
                    }
                }
            }

            // let free_adj = 8 - occupied_adj;
            // println!("free adjacent: {}", free_adj);
            if occupied_adj < 4 {
                println!("available: {:?}", (r, c));
                accessible += 1;
            }
        }
    }

    Some(accessible)
}

fn num_removable(board: Board) -> usize {
    let mut removable: Vec<Coord> = vec![];

    for r in 0..board.rows as i32 {
        // println!("foo");
        for c in 0..board.cols as i32 {
            let coord = (r as usize, c as usize);
            if board.get(&coord) != ROLL_TILE {
                // Not a roll, no need to count
                continue;
            }
            let mut occupied_adj = 0;
            for dr in -1i32..=1 {
                // println!("baz");
                for dc in -1i32..=1 {
                    // println!("ham");
                    if dr == 0 && dc == 0 {
                        continue; // Same roll
                    }

                    let maybe_row = r + dr;
                    let maybe_col = c + dc;

                    if maybe_row < 0 || maybe_col < 0 {
                        continue;
                    }

                    let row = maybe_row as usize;
                    let col = maybe_col as usize;
                    let coordinate = (row, col);

                    if let Some(neigh) = board.maybe_tile(&coordinate) {
                        if neigh != FREE_TILE {
                            occupied_adj += 1;
                        }
                        // Otherwise free
                    } else {
                        // Out of board, don't count
                    }
                }
            }

            if occupied_adj < 4 {
                removable.push(coord);
                println!("removable: {:?}", (r, c));
            }
        }
    }

    let removed_this_iter = removable.len();
    if removed_this_iter == 0 {
        return 0; // Base case, stop recursing
    } else {
        let next_board = board.new_without(removable);
        return num_removable(next_board) + removed_this_iter;
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let board = Board::new(input)?;

    Some(num_removable(board) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
