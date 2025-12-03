advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| {
                let numbers: Vec<u32> = line
                    .chars()
                    .map(|char| char.to_digit(10).unwrap())
                    .collect();

                let mut max = numbers[0];
                let mut max_index = 0;

                for n in 1..(numbers.len() - 1) {
                    if numbers[n] > max {
                        max = numbers[n];
                        max_index = n;
                    }

                    if max == 9 {
                        break;
                    }
                }

                let mut second = numbers[max_index + 1];
                let first_index = (max_index + 2).min(numbers.len() - 1);
                for n in first_index..numbers.len() {
                    if numbers[n] > second {
                        second = numbers[n];
                    }

                    if second == 9 {
                        break;
                    }
                }

                format!("{max}{second}").parse::<u64>().unwrap()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    const AMOUNT_OF_BATTERIES: usize = 12;

    Some(
        input
            .lines()
            .map(|line| line.trim())
            .map(|line| {
                let numbers: Vec<u32> = line
                    .chars()
                    .map(|char| char.to_digit(10).unwrap())
                    .collect();

                let len = numbers.len();

                let mut result: Vec<u64> = Vec::new();
                let mut first_idx = 0;

                for round in 0..AMOUNT_OF_BATTERIES {
                    let last_idx = len - AMOUNT_OF_BATTERIES + round;

                    let mut max = numbers[first_idx];

                    if first_idx == last_idx || first_idx == numbers.len() - 1 {
                        result.push(max.into());
                        first_idx += 1;
                        continue;
                    }

                    first_idx += 1;

                    for idx in first_idx..=last_idx {
                        let target = numbers[idx];
                        if target <= max {
                            continue;
                        }

                        max = target;
                        first_idx = idx + 1;
                    }

                    result.push(max.into());
                }

                result.iter().fold(0, |acc, &digit| acc * 10 + digit)
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

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
