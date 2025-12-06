use itertools::Itertools;

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

fn transpose(v: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = v.len();
    let cols = v.first().expect("Should have first").len();

    let mut out: Vec<Vec<char>> = vec![];
    // Col vec from input, row vec in output
    for i in 0..cols {
        let mut out_row: Vec<char> = vec![];
        for j in 0..rows {
            out_row.push(v[j][i]);
        }
        out.push(out_row);
    }
    out
}

pub fn part_two(input: &str) -> Option<u64> {
    let line_count = input.lines().count();
    let number_lines: Vec<Vec<char>> = input
        .lines()
        .take(line_count - 1)
        .map(|line| line.chars().collect())
        .collect();
    let operators: Vec<char> = input
        .lines()
        .last()?
        .split_whitespace()
        .map(|x| x.chars())
        .flatten()
        .collect();

    println!("before transpose");
    let transposed = transpose(number_lines);
    println!("transpose finished");
    let problems: Vec<Vec<Vec<char>>> = transposed
        .split(|col| col.iter().all(|c| c.is_whitespace()))
        .map(|x| x.iter().cloned().collect_vec())
        .collect();

    let total: u64 = problems
        .iter()
        .zip(operators)
        .map(|(problem, op)| {
            let operator = match op {
                '+' => Operator::Add,
                '*' => Operator::Mul,
                _ => panic!("Invalid oeprator"),
            };

            let clean_cols: Vec<u64> = problem
                .iter()
                .map(|col| {
                    col.iter()
                        .filter(|c| !c.is_whitespace())
                        .join("")
                        .to_string()
                        .parse()
                        .expect("should parse")
                })
                .collect();
            // let problem_result: u64 = clean_cols
            //     .iter()
            //     .reduce(|acc, e| operator.perform(*acc, *e))
            //     .unwrap_or(0u64);
            let problem_result: u64 = clean_cols
                .into_iter()
                .reduce(|acc, e| {
                    let x = operator.perform(acc, e);
                    x
                })
                .unwrap_or(0)
                .clone();
            problem_result
        })
        .sum();

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        let expected = vec![vec!['1', '2', '3'], vec!['4', '5', '6']];
        let actual = transpose(vec![vec!['1', '4'], vec!['2', '5'], vec!['3', '6']]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
