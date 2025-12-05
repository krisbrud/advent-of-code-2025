advent_of_code::solution!(5);

struct Range {
    lower: u64,
    upper: u64,
}

impl Range {
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
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        let range = Range::parse("10-14").expect("should parse");
        assert_eq!(false, range.contains(9));
        assert_eq!(true, range.contains(10));
        assert_eq!(true, range.contains(11));
        assert_eq!(true, range.contains(14));
        assert_eq!(false, range.contains(15));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
