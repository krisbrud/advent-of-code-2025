advent_of_code::solution!(9);

use itertools::Itertools;
use std::{cmp, collections::HashMap};

type Coord = (i64, i64);

fn parse_coords(input: &str) -> Vec<Coord> {
    let coords = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            // (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
            (y.parse::<i64>().unwrap(), x.parse::<i64>().unwrap())
        })
        .collect();
    coords
}

fn size_of_rect(first: &Coord, second: &Coord) -> i64 {
    ((first.0 - second.0).abs() + 1) * ((first.1 - second.1).abs() + 1)
}

fn largest_rect(coords: Vec<Coord>) -> i64 {
    let mut max = 0;
    for i in 0..coords.len() - 1 {
        for j in i + 1..coords.len() {
            let first = coords[i];
            let second = coords[j];
            let size = size_of_rect(&first, &second);
            if size > max {
                max = size;
            }
        }
    }
    max
}

pub fn part_one(input: &str) -> Option<i64> {
    let coords = parse_coords(input);
    Some(largest_rect(coords))
}

// How can we find out if something is in the interior?
// ..............
// .......#XXX#..
// .......X...X..
// ..#XXXX#...X..
// ..X........X..
// ..#XXXXXX#.X..
// .........X.X..
// .........#X#..
// ..............
//
// Is it tractable to make a bitmap?
// 100000 * 100000 = 10**10 bytes = 10 GB RAM (not ideal)
//
// Can potentially make a map of all the x- and y- coords in the lines as well as their directions.
// Then, for each potential rectangle (~500 * 500 / 2= 12500),
//      Check if there are any unexpected directions.
//
// What is an unexpected coord?
//      If we are going clockwise
//      Top side: Down or left before the corner
//      Right side: Left or up before the corner
//      Lower side: Up or right...
//      Left side: Right or down...
//
//      => Rotating the degree 180 degs gives an unexpected direction
//
// Example above now becomes
// ..............
// .......>>>>v..
// .......^...v..
// ..>>>>>^...v..
// ..^........v..
// ..^<<<<<<<.v..
// .........^.v..
// .........^<<..
// ..............
//
// Does the direction of the cycle matter? Yes, but won't every cycle fail if we pick the wrong one?

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn add(first: &Coord, second: &Coord) -> Coord {
    (first.0 + second.0, first.1 + second.1)
}

fn mul(first: &Coord, scalar: i64) -> Coord {
    (first.0 * scalar, first.1 * scalar)
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }

    fn rhs(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn unexpected_line_dirs(&self) -> Vec<Direction> {
        // vec![self.rhs(), self.opposite()]
        vec![self.rhs(), self.opposite()]
    }

    fn as_coord(&self) -> Coord {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}

fn find_direction(first: &Coord, second: &Coord) -> Option<Direction> {
    let diff = (second.0 - first.0, second.1 - first.1);
    let dir = match diff {
        (0, x) => match &x.cmp(&0) {
            std::cmp::Ordering::Greater => Direction::Right,
            std::cmp::Ordering::Equal => return None, // 0, 0
            std::cmp::Ordering::Less => Direction::Left,
        },
        (y, 0) => match &y.cmp(&0) {
            std::cmp::Ordering::Greater => Direction::Down,
            std::cmp::Ordering::Equal => return None, // 0, 0
            std::cmp::Ordering::Less => Direction::Up,
        },
        _ => return None, // 0, 0
    };
    Some(dir)
}

fn line_between(first: &Coord, second: &Coord) -> Option<Vec<(Coord, Direction)>> {
    match find_direction(first, second) {
        Some(direction) => {
            let direction_coord = direction.as_coord();
            let length = size_of_rect(first, second);
            let coords = (0..length - 1)
                .into_iter()
                .scan(first, |acc, i| Some(add(&first, &mul(&direction_coord, i))))
                .map(|coord| (coord, direction.clone()))
                .collect();
            Some(coords)
        }
        None => None,
    }
}

fn wrap(coords: &mut Vec<Coord>) -> Vec<Coord> {
    let first = coords.first().expect("Should not be empty");
    coords.push(*first);
    coords.to_vec()
}

fn corner_and_line_between(first: &Coord, second: &Coord) -> Option<Vec<(Coord, Direction, bool)>> {
    let out = line_between(first, second)?
        .iter()
        .enumerate()
        .map(|(i, coord_dir)| {
            let is_corner = i == 0;
            (coord_dir.0.clone(), coord_dir.1.clone(), is_corner)
        })
        .collect_vec();
    Some(out)
}

pub fn part_two(input: &str) -> Option<i64> {
    let coords = wrap(&mut parse_coords(input));
    let corner_pairs: Vec<(Coord, Coord)> = coords.clone().into_iter().tuple_windows().collect();
    // println!("{:?}", corner_pairs);
    let line_vec: Vec<(Coord, Direction)> = corner_pairs
        .iter()
        .map(|(c1, c2)| line_between(c1, c2))
        .flatten()
        .flatten()
        .collect();
    // .collect::<Option<Vec<(Coord, Direction)>>>()?;
    let line_map: HashMap<Coord, Direction> = line_vec
        .iter()
        .map(|tup| (tup.0.clone(), tup.1.clone()))
        .collect();

    let mut max = 0;
    let mut i = 0;
    for (c1, c2) in coords.iter().tuple_combinations() {
        i += 1;
        if i % 100 == 0 {
            println!("Iteration {}", i);
        }
        let top_left = (cmp::min(c1.0, c2.0), cmp::min(c1.1, c2.1));
        let top_right = (cmp::min(c1.0, c2.0), cmp::max(c1.1, c2.1));
        let bottom_right = (cmp::max(c1.0, c2.0), cmp::max(c1.1, c2.1));
        let bottom_left = (cmp::max(c1.0, c2.0), cmp::min(c1.1, c2.1));

        // Check sides
        let rect_lines = corner_and_line_between(&top_left, &top_right)
            .into_iter()
            .chain(corner_and_line_between(&top_right, &bottom_right).into_iter())
            .chain(corner_and_line_between(&bottom_right, &bottom_left))
            .chain(corner_and_line_between(&bottom_left, &top_left))
            .collect_vec();

        if bottom_left == (5, 2) && top_right == (1, 11) {
            println!("c1 {:?} c2 {:?}", c1, c2);
            println!("rect_lines: {:?}", rect_lines);
        }

        if !rect_lines
            .iter()
            .flatten()
            .all(|(coord, direction, is_corner)| match line_map.get(coord) {
                Some(actual_dir) => {
                    let result = if *is_corner {
                        true
                    } else {
                        !direction.unexpected_line_dirs().contains(actual_dir)
                    };
                    if bottom_left == (5, 2) && top_right == (1, 11) {
                        println!("{:?} {}", coord, result);
                    }
                    result
                }
                _ => true, // No line, no problemo
            })
        {
            continue;
        }

        let rect_size = size_of_rect(&top_left, &bottom_right);
        if rect_size > max {
            // println!("New max found! {:?}", (top_left, bottom_right));
            max = rect_size;
        }
    }
    Some(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_of_rect() {
        assert_eq!(50, size_of_rect(&(2, 5), &(11, 1)));
        assert_eq!(50, size_of_rect(&(11, 1), &(2, 5)));
    }

    #[test]
    fn test_find_dir() {
        assert_eq!(Some(Direction::Up), find_direction(&(2, 5), &(0, 5)));
        assert_eq!(Some(Direction::Right), find_direction(&(2, 5), &(2, 8)));
        assert_eq!(Some(Direction::Down), find_direction(&(2, 5), &(3, 5)));
        assert_eq!(Some(Direction::Left), find_direction(&(2, 5), &(2, 4)));
        assert_eq!(None, find_direction(&(2, 5), &(2, 5)));
    }

    // #[test]
    // fn test_line_between() {
    //     assert_eq!(
    //         Some(vec![((1, 5), Direction::Up),]),
    //         line_between(&(2, 5), &(0, 5))
    //     );
    //     assert_eq!(
    //         Some(vec![
    //             ((6, 6), Direction::Down),
    //             ((7, 6), Direction::Down),
    //             ((8, 6), Direction::Down),
    //             ((9, 6), Direction::Down)
    //         ]),
    //         line_between(&(5, 6), &(10, 6))
    //     );
    // }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
        // 1289405152 too low
    }
}
