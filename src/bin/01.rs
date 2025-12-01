advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut current: i32 = 50;
    let mut count: u64 = 0;

    for line in input.lines() {
        let mut amount: i32 = line[1..].parse().unwrap();
        amount %= 100;

        let result = if line.starts_with("L") {
            let mut inner = current - amount;
            if inner < 0 {
                inner = 99 + inner + 1;
            }
            inner
        } else {
            (current + amount) % 100
        };

        current = result.abs();

        if result == 0 {
            count += 1;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut current = 50;
    let mut count = 0;

    for line in input.lines() {
        let amount: i32 = line[1..].parse().ok()?;
        for _ in 0..amount {
            if line.starts_with("L") {
                current = if current == 0 { 99 } else { current - 1 };
            } else {
                current = if current == 99 { 0 } else { current + 1 };
            };

            if current == 0 {
                count += 1;
            }
        }
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
    }
}
