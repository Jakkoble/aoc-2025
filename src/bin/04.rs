advent_of_code::solution!(4);

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn count_neighbours(rows: &Vec<Vec<char>>, row: i32, col: i32) -> u32 {
    let mut count = 0;
    for r in 0.max(row - 1)..(row + 2).min(rows.len().try_into().unwrap()) {
        for c in 0.max(col - 1)..(col + 2).min(rows[0].len().try_into().unwrap()) {
            if r == row && c == col {
                continue;
            }
            if rows[r as usize][c as usize] == '@' {
                count += 1;
            }
        }
    }
    count
}

pub fn part_one(input: &str) -> Option<u64> {
    let rows = parse_input(input);
    let mut count = 0;

    for r in 0..rows.len() {
        let row = &rows[r];

        for c in 0..row.len() {
            let cell = row[c];

            if cell != '@' {
                continue;
            }

            if count_neighbours(&rows, r.try_into().unwrap(), c.try_into().unwrap()) < 4 {
                count += 1;
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut rows = parse_input(input);
    let mut count = 0;

    loop {
        let mut removed_rolls = 0;

        for r in 0..rows.len() {
            for c in 0..rows[r].len() {
                if rows[r][c] != '@' {
                    continue;
                }

                if count_neighbours(&rows, r.try_into().unwrap(), c.try_into().unwrap()) < 4 {
                    count += 1;
                    removed_rolls += 1;
                    rows[r][c] = '.';
                }
            }
        }

        if removed_rolls == 0 {
            break;
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
