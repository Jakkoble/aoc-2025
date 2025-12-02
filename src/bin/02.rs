advent_of_code::solution!(2);

pub fn parse_input(input: &str) -> impl Iterator<Item = (u64, u64)> {
    input.trim().split(",").filter_map(|range| {
        if let Some((start, end)) = range.split_once("-") {
            Some((start.parse().ok()?, end.parse().ok()?))
        } else {
            None
        }
    })
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse_input(&input)
            .flat_map(|(start, end)| start..=end)
            .filter(|n| {
                let digits: Vec<u8> = n.to_string().bytes().map(|b| b - b'0').collect();

                if digits.len() % 2 == 1 {
                    return false;
                }

                let (first, second) = digits.split_at(digits.len() / 2);
                if first.len() != second.len() {
                    return false;
                }

                for i in 0..first.len() {
                    if first[i] != second[i] {
                        return false;
                    }
                }

                true
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        parse_input(&input)
            .flat_map(|(start, end)| start..=end)
            .filter(|n| {
                let digits: Vec<u8> = n.to_string().bytes().map(|b| b - b'0').collect();
                let max_split_size = digits.len() / 2;

                for split_size in 1..=max_split_size {
                    let splits: Vec<String> = digits
                        .chunks(split_size)
                        .map(|chunk| String::from_utf8_lossy(chunk).to_string())
                        .collect();

                    if splits.len() < 2 || splits[0].len() != split_size {
                        continue;
                    }

                    let first = &splits[0];
                    if splits[1..].iter().any(|split| *first != *split) {
                        continue;
                    }

                    return true;
                }

                false
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
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
