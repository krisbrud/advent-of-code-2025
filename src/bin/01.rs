advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i64> {
    let numbers: Vec<i64> = input
        .replace("L", "-")
        .replace("R", "")
        .lines()
        .map(|s| s.parse::<i64>().expect("Should parse"))
        .collect();

    let mut count = 0;
    let mut sum = 50;
    for num in numbers {
        sum += num;
        sum = sum.rem_euclid(100); // TODO fix

        if sum == 0 {
            count += 1;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<i64> {
    let numbers: Vec<i64> = input
        .replace("L", "-")
        .replace("R", "")
        .lines()
        .map(|s| s.parse::<i64>().expect("Should parse"))
        .collect();

    let mut count = 0;
    let mut sum = 50;
    for num in numbers {
        let before = sum;
        sum += num;

        // If positive: Just add the number and do integer division by 100 to find number of times we hit 0
        // If negative: Do the same, but check if we get below 0, take the abs and add 1, since -50 means 1 rotation etc.
        if num > 0 {
            count += (sum.abs() / 100);
        } else if sum <= 0 {
            count += (sum.abs() / 100);
            if before > 0 {
                // Passed 0
                count += 1;
            }
        }
        sum = sum.rem_euclid(100);
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));

        // 6817 too high
    }
}
