advent_of_code::solution!(5);

use std::cmp;

#[derive(Clone, Debug, PartialEq)]
struct Range {
    lower: u64,
    upper: u64,
}

impl Range {
    fn new(lower: u64, upper: u64) -> Range {
        Range { lower, upper }
    }
    fn parse(line: &str) -> Option<Range> {
        let (lower, upper) = line.split_once("-")?;
        Some(Range {
            lower: lower.parse().ok()?,
            upper: upper.parse().ok()?,
        })
    }

    fn contains(&self, x: u64) -> bool {
        return self.lower <= x && x <= self.upper;
    }

    fn overlaps(&self, other: &Range) -> bool {
        return self.lower <= other.upper && other.lower <= self.upper;
    }

    fn union(&self, other: &Range) -> Range {
        // Assume overlap
        let lower = cmp::min(self.lower, other.lower);
        let upper = cmp::max(self.upper, other.upper);
        Range { lower, upper }
    }

    fn size(&self) -> u64 {
        return self.upper - self.lower + 1;
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (range_str, ingredient_str) = input.split_once("\n\n").expect("Should split");
    let ranges: Vec<Range> = range_str
        .lines()
        .map(|line| Range::parse(line))
        .collect::<Option<_>>()?;
    let ingredients: Vec<u64> = ingredient_str
        .lines()
        .map(|line| line.parse::<u64>().ok())
        .collect::<Option<_>>()?;

    let fresh_count = ingredients
        .iter()
        .filter(|ingredient| {
            if let Some(_) = ranges.iter().find(|range| range.contains(**ingredient)) {
                true
            } else {
                false
            }
        })
        .count();

    Some(fresh_count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (range_str, ingredient_str) = input.split_once("\n\n").expect("Should split");
    let ranges: Vec<Range> = range_str
        .lines()
        .map(|line| Range::parse(line))
        .collect::<Option<_>>()?;

    let mut current_ranges: Vec<Option<Range>> =
        ranges.iter().map(|range| Some(range.clone())).collect();
    let mut next_ranges: Vec<Option<Range>> = vec![];
    let mut iter = 1;
    while current_ranges != next_ranges {
        println!(
            "current size {:?} next size {:?}",
            current_ranges.len(),
            next_ranges.len()
        );
        iter += 1;
        if iter >= 10000 {
            panic!("too many iterations");
        }
        for i in 0..current_ranges.len() {
            if let Some(first) = current_ranges[i].clone() {
                let mut acc = first;
                for j in i + 1..current_ranges.len() {
                    if let Some(second) = current_ranges[j].clone() {
                        if acc.overlaps(&second) {
                            acc = acc.union(&second);
                            current_ranges[j] = None;
                        }
                    }
                }
                next_ranges.push(Some(acc));
            }
        }

        if current_ranges == next_ranges {
            return Some(
                next_ranges
                    .into_iter()
                    .filter_map(|option| option)
                    .map(|range| range.size())
                    .sum(),
            );
        }

        current_ranges = next_ranges;
        next_ranges = vec![];
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_contains() {
        let range = Range::parse("10-14").expect("should parse");
        assert_eq!(false, range.contains(9));
        assert_eq!(true, range.contains(10));
        assert_eq!(true, range.contains(11));
        assert_eq!(true, range.contains(14));
        assert_eq!(false, range.contains(15));
    }

    #[test]
    fn test_range_overlap() {
        let range = Range::new(10, 14);
        assert_eq!(true, range.overlaps(&Range::new(9, 10)));
        assert_eq!(false, range.overlaps(&Range::new(8, 9)));
        assert_eq!(true, range.overlaps(&range));
        assert_eq!(true, range.overlaps(&Range::new(11, 13)));
        assert_eq!(true, range.overlaps(&Range::new(14, 20)));
        assert_eq!(false, range.overlaps(&Range::new(15, 20)));
    }

    #[test]
    fn test_range_union() {
        let range = Range::new(10, 14);
        assert_eq!(Range::new(9, 14), range.union(&Range::new(9, 10)));
        assert_eq!(range, range.union(&range));
        assert_eq!(range, range.union(&Range::new(11, 13)));
        assert_eq!(range, range.union(&Range::new(10, 13)));
        assert_eq!(range, range.union(&Range::new(11, 14)));
        assert_eq!(Range::new(10, 16), range.union(&Range::new(12, 16)));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
