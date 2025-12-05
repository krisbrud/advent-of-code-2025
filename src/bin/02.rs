advent_of_code::solution!(2);

use std::cmp;

fn is_repeated(s: &str) -> bool {
    let mid = s.len() / 2; // First index after midpoint
    let (first, second) = s.split_at(mid);
    return first == second;
}

fn split_even_to_halves(n: u64) -> (u64, u64) {
    let digits = num_digits(n);
    assert!(digits % 2 == 0);

    let half_digits = digits / 2;
    let half = 10u64.pow(half_digits);

    let first = n / half;
    let second = n % half;
    (first, second)
}

fn num_digits(n: u64) -> u32 {
    if n < 10 {
        return 1;
    }
    n.checked_ilog10().unwrap() + 1
}

// An invalid ID will always be two halves, for instance `123` in `123123`
fn lower_half(n: u64) -> u64 {
    // In the case where n has an odd number of digits, we round up to 10**(n_digits)
    let n_digits = num_digits(n);
    if n_digits % 2 == 0 {
        let (first, second) = split_even_to_halves(n);
        if first >= second {
            return first;
        } else {
            return first + 1;
        }
    } else {
        return 10u64.pow(n_digits / 2);
    }
}

fn upper_half(n: u64) -> u64 {
    let n_digits = num_digits(n);
    if n_digits % 2 == 0 {
        // Even number of digits, we can split the number to get halves
        let (first, second) = split_even_to_halves(n);
        if first <= second {
            return first;
        } else {
            return first - 1;
        }
        // return cmp::min(first, second);
    } else {
        return 10u64.pow((n_digits / 2)) - 1;
    }
}
#[derive(Debug, PartialEq)]
struct IdRange {
    first: u64,
    second: u64,
}

impl IdRange {
    fn parse(line: &str) -> Option<IdRange> {
        let (first, second) = line.split_once("-")?;
        Some(IdRange {
            first: first.parse().ok()?,
            second: second.parse().ok()?,
        })
    }

    // fn invalid_ids(&self) -> Vec<&str> {
    //     let mut v: Vec<&str> = vec![];
    //     if is_repeated(&self.first) {
    //         v.push(&self.first);
    //     }
    //     if is_repeated(&self.second) {
    //         v.push(&self.second);
    //     }
    //     v
    // }

    // fn invalid_sum(&self) -> u64 {
    //     let invalid = self.invalid_ids();
    //     invalid
    //         .iter()
    //         .map(|inv| {
    //             let num: u64 = inv.parse().unwrap();
    //             num
    //         })
    //         .sum()
    // }
}

fn invalid_ids_in_range(lower: u64, upper: u64) -> Vec<u64> {
    assert!(lower <= upper);
    (lower..=upper)
        .map(|half| 10u64.pow(num_digits(half)) * half + half)
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = input
        .split(",")
        .map(|r| IdRange::parse(r))
        .collect::<Option<Vec<IdRange>>>()?;

    // println!("{:?}", ranges);

    let invalid_sum = ranges
        .iter()
        .map(|range| {
            let lower = lower_half(range.first);
            let upper = upper_half(range.second);
            println!("range {:?}, lower {:?} upper {:?}", range, lower, upper);
            if lower > upper {
                vec![]
            } else {
                let v = invalid_ids_in_range(lower, upper);
                println!("{:?}", v);
                v
            }
        })
        .flatten()
        .sum();

    Some(invalid_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_repeated() {
        assert_eq!(true, is_repeated("1010"));
        assert_eq!(false, is_repeated("101"));
    }

    #[test]
    fn test_num_digits() {
        assert_eq!(1, num_digits(0));
        assert_eq!(1, num_digits(1));
        assert_eq!(1, num_digits(0));
        assert_eq!(2, num_digits(10));
        assert_eq!(2, num_digits(99));
        assert_eq!(5, num_digits(12345));
    }

    #[test]
    fn test_split_even_to_halves() {
        assert_eq!((1, 1), split_even_to_halves(11));
        assert_eq!((1, 0), split_even_to_halves(10));
        assert_eq!((12, 10), split_even_to_halves(1210));
    }

    #[test]
    fn test_lower_half() {
        assert_eq!(1, lower_half(11));
        assert_eq!(2, lower_half(12));
        assert_eq!(10, lower_half(123));
        assert_eq!(10, lower_half(100));
        assert_eq!(10, lower_half(999));
        assert_eq!(270, lower_half(269351));
        assert_eq!(351, lower_half(351269));
        assert_eq!(351, lower_half(351351));
    }

    #[test]
    fn test_upper_half() {
        assert_eq!(1, upper_half(11));
        assert_eq!(1, upper_half(12));
        assert_eq!(9, upper_half(123));
        assert_eq!(9, upper_half(100));
        assert_eq!(99, upper_half(99999));
        assert_eq!(269, upper_half(269351));
        assert_eq!(350, upper_half(351269));
        assert_eq!(351, upper_half(351351));
    }

    #[test]
    fn test_invalid_ids_in_range() {
        assert_eq!(vec![11], invalid_ids_in_range(1, 1));
        assert_eq!(vec![11, 22], invalid_ids_in_range(1, 2));
        assert_eq!(vec![11, 22, 33], invalid_ids_in_range(1, 3));
    }

    #[test]
    fn test_parse() {
        let parsed = IdRange::parse("11-22").unwrap();
        assert_eq!(
            IdRange {
                first: 11,
                second: 22
            },
            parsed
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
        // 4754507101 too low
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
