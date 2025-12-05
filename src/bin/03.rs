advent_of_code::solution!(3);

fn parse_digits(line: &str) -> Vec<u32> {
    line.chars()
        .map(|c| c.to_digit(10).expect("Char should be digit"))
        .collect()
}

fn output_joltage(digits: Vec<u32>, remaining: usize) -> u64 {
    // Budget: Remaining digits to take
    if remaining == 0 {
        // Base case
        return 0;
    }
    let len = digits.len();

    let next_remaining = remaining - 1;
    let num_to_take = if remaining == 1 {
        len
    } else {
        len - next_remaining
    };
    let first_max = digits
        .iter()
        .take(num_to_take) // First char cannot be the last
        // .max_by(|(_, x1), (_, x2)| x1.cmp(x2))
        .max()
        .expect("Should have max");
    let first_idx = digits
        .iter()
        .position(|x| x == first_max)
        .expect("Max should have position");
    println!("{:?} {:?} {:?}", digits, num_to_take, first_idx);

    let next_digits: Vec<u32> = digits[first_idx + 1..].iter().cloned().collect();

    let base = 10u64.pow((remaining as u32) - 1);
    // println!("{:?}", base);
    return (*first_max as u64) * base + output_joltage(next_digits, remaining - 1);
}

pub fn part_one(input: &str) -> Option<u64> {
    // Ideas: Greedy algorithm, pick the largest digit first (except the last one)
    // for the second digit, pick the largest digit after the first
    let joltage_sum: u64 = input
        .lines()
        .map(|line| output_joltage(parse_digits(line), 2))
        .sum();

    Some(joltage_sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    // Idea: Same approach, but solve it recursively
    // Create a list of digits, create a smaller list from it
    // Create numbers from the lists and sum them
    let joltage_sum: u64 = input
        .lines()
        .map(|line| output_joltage(parse_digits(line), 12))
        .sum();

    Some(joltage_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_joltage() {
        assert_eq!(98, output_joltage(parse_digits("987654321111111"), 2));
        assert_eq!(
            987654321111,
            output_joltage(parse_digits("987654321111111"), 12)
        );
        assert_eq!(
            811111111119,
            output_joltage(parse_digits("811111111111119"), 12)
        );
        assert_eq!(
            434234234278,
            output_joltage(parse_digits("234234234234278"), 12)
        );
        assert_eq!(
            888911112111,
            output_joltage(parse_digits("818181911112111"), 12)
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
