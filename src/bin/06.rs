advent_of_code::solution!(6);

enum Operator {
    Add,
    Mul,
}

impl Operator {
    fn perform(&self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Operator::Add => lhs + rhs,
            Operator::Mul => lhs * rhs,
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<_> = input.lines().collect();
    let num_lines = lines.len();

    let numbers: Vec<Vec<u64>> = lines
        .iter()
        .take(num_lines - 1)
        .map(|line| {
            line.split_whitespace()
                .map(|number| number.parse::<u64>().ok())
                .collect()
        })
        .collect::<Option<Vec<Vec<_>>>>()?;
    let operators: Vec<char> = lines
        .last()?
        .split_whitespace()
        .map(|x| x.chars())
        .flatten()
        .collect();

    let rows = numbers.len();
    let cols = numbers.first()?.len();

    let mut total = 0;
    for i in 0..cols {
        let mut acc = numbers[0][i];
        let operator = match operators[i] {
            '+' => Operator::Add,
            '*' => Operator::Mul,
            _ => panic!("Invalid oeprator"),
        };

        for j in 1..rows {
            acc = operator.perform(acc, numbers[j][i]);
        }
        total += acc;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
