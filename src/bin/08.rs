advent_of_code::solution!(8);

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

type Coord = (u32, u32, u32);

fn dist(a: &Coord, b: &Coord) -> u32 {
    // Squared euclidian distance
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

lazy_static! {
    static ref regex_pattern: Regex = Regex::new(r"^([\d]+),([\d]+),([\d]+)$").unwrap();
}

fn parse_coords(input: &str) -> Option<Vec<Coord>> {
    let coords: Vec<Coord> = input
        .lines()
        .map(|line| {
            let caps = regex_pattern.captures(line)?;
            let tup = (
                caps[1].parse().ok()?,
                caps[2].parse().ok()?,
                caps[3].parse().ok()?,
            );
            Some(tup)
        })
        .collect::<Option<Vec<Coord>>>()?;

    Some(coords)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CoordDist {
    dist: Reverse<u32>,
    first: Coord,
    second: Coord,
}

// impl Ord for CoordDist {
//     fn cmp(&self, &other) -> Ordering {
//         Reverse(self.2.cmp(other.2))
//     }
// }

fn find_distances(coords: Vec<Coord>) -> Vec<CoordDist> {
    let mut dists = vec![];
    for i in 0..(coords.len() - 1) {
        for j in (i + 1)..coords.len() {
            let first = coords[i];
            let second = coords[j];
            dists.push(CoordDist {
                dist: Reverse(dist(&first, &second)),
                first: first,
                second: second,
            });
        }
    }
    dists
}

fn part_one_impl(input: &str, n: u32) -> Option<u64> {
    let coords = parse_coords(input)?;

    // let mut dist_heap: BinaryHeap<CoordDist> = BinaryHeap::new();
    // for dist in find_distances(coords) {
    //     dist_heap.push(dist);
    // }

    // Mapping from Coord to the enumeration of its associated spanning tree
    // (since we don't really care about the graph itself in this task)
    let mut associated_spanning_tree: HashMap<Coord, u32> = HashMap::new();

    let mut next_assoc = 0;
    let mut connections = 0;
    while let Some(dist) = dist_heap.pop() {
        println!("Popped dist {:?}", dist);
        match (
            associated_spanning_tree.get(&dist.first).cloned(),
            associated_spanning_tree.get(&dist.second).cloned(),
        ) {
            (None, None) => {
                println!("None, None");
                associated_spanning_tree.insert(dist.first, next_assoc);
                associated_spanning_tree.insert(dist.second, next_assoc);
                next_assoc += 1;
            }
            (Some(assoc), None) => {
                println!("Some {}, None", assoc);
                associated_spanning_tree.insert(dist.second, assoc);
            }
            (None, Some(assoc)) => {
                println!("None, Some {}", assoc);
                associated_spanning_tree.insert(dist.first, assoc);
            }
            (Some(first), Some(second)) => {
                println!("Some {}, Some {}", first, second);
                if first == second {
                    println!("Same circuit");
                    // Same circuit
                    continue;
                }
                println!("Not same circuit");
                // Set all the instances of second to first
                let second_tree_coords: Vec<Coord> = associated_spanning_tree
                    .iter()
                    .filter(|it| *it.1 == second)
                    .map(|it| it.0)
                    .cloned()
                    .collect();
                for coord in second_tree_coords {
                    associated_spanning_tree.insert(coord, first);
                }
            }
        }
        connections += 1;
    }

    let associated_circuits = associated_spanning_tree
        .iter()
        .map(|assoc| *assoc.1)
        .sorted()
        .map(|v| v as u64)
        .rev()
        .collect_vec();

    println!("{:?}", associated_circuits);

    let sizes = associated_circuits
        .iter()
        .dedup_with_count()
        .sorted_by_key(|(_, count)| -(**count as i64))
        .collect_vec();

    println!("sizes: {:?}", sizes);
    let biggest_three_sizes = sizes
        .iter()
        .take(3)
        .map(|a| a.0)
        .reduce(|acc, elem| acc * elem)
        .expect("should reduce") as u64;

    Some(biggest_three_sizes)
}

pub fn part_one(input: &str) -> Option<u64> {
    part_one_impl(input, 1000)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_impl(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
