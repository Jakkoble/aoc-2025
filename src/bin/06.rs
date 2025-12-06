advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let calcs = parse_part_one(input);
    Some(
        calcs
            .iter()
            .map(|calc| match calc {
                Calculation::Add { operands } => operands.iter().sum::<u64>(),
                Calculation::Mul { operands } => operands.iter().product::<u64>(),
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    let (operators, operand_lines) = lines.split_last().unwrap();

    let column_starts: Vec<usize> = operators
        .chars()
        .enumerate()
        .filter_map(|(idx, char)| {
            if char.is_ascii_whitespace() {
                None
            } else {
                Some(idx)
            }
        })
        .collect();

    Some(
        column_starts
            .iter()
            .enumerate()
            .map(|(idx, start)| {
                // calculate end of current column
                let end = if start == column_starts.last().unwrap() {
                    // until end of line
                    operand_lines[0].len()
                } else {
                    // until one before next start
                    column_starts[idx + 1] - 1
                };

                let operands: Vec<_> = operand_lines
                    .iter()
                    .map(|line| &line[*start..end])
                    .collect();

                let mut numbers: Vec<u64> = vec![];
                for i in 0..end - start {
                    let number: u64 = operands
                        .iter()
                        .filter_map(|operand| {
                            let target = operand.chars().nth(i).unwrap();
                            if target.is_whitespace() {
                                None
                            } else {
                                Some(target)
                            }
                        })
                        .collect::<String>()
                        .parse::<u64>()
                        .unwrap();
                    numbers.push(number);
                }

                match operators.chars().nth(*start).unwrap() {
                    '+' => numbers.iter().sum::<u64>(),
                    '*' => numbers.iter().product::<u64>(),
                    _ => panic!("Unkown operation!"),
                }
            })
            .sum(),
    )
}

#[derive(Debug)]
pub enum Calculation {
    Add { operands: Vec<u64> },
    Mul { operands: Vec<u64> },
}

pub fn parse_part_one(input: &str) -> Vec<Calculation> {
    let lines: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    let amount_of_calculations = lines[0].len();

    let data: Vec<&str> = lines.into_iter().flatten().collect();
    let items_per_calculation = data.len() / amount_of_calculations;

    let mut calcs: Vec<Calculation> = vec![];
    for calculation in 0..amount_of_calculations {
        let mut operands: Vec<u64> = vec![];

        for part in 0..items_per_calculation {
            let target = data[part * amount_of_calculations + calculation];
            match target.parse::<u64>() {
                Ok(number) => operands.push(number),
                Err(_) => match target {
                    "+" => calcs.push(Calculation::Add {
                        operands: operands.clone(),
                    }),
                    "*" => calcs.push(Calculation::Mul {
                        operands: operands.clone(),
                    }),
                    _ => panic!("Unkonw operation!"),
                },
            }
        }
    }

    calcs
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
        assert_eq!(result, Some(3263827));
    }
}
