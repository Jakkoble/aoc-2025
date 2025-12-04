advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::new(input);
    let mut count = 0;

    for x in 0..grid.width {
        for y in 0..grid.height {
            if let Some('@') = grid.get(x, y) {
                if grid.count_neighbours(x, y) < 4 {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = Grid::new(input);
    let mut count = 0;

    loop {
        let mut removed_rolls = 0;

        for x in 0..grid.width {
            for y in 0..grid.height {
                if let Some('@') = grid.get(x, y) {
                    if grid.count_neighbours(x, y) < 4 {
                        count += 1;
                        removed_rolls += 1;
                        grid.set(x, y, '.');
                    }
                }
            }
        }

        if removed_rolls == 0 {
            break;
        }
    }

    Some(count)
}

#[derive(Debug)]
struct Grid {
    height: usize,
    width: usize,
    data: Vec<char>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        if lines.is_empty() {
            return Self {
                height: 0,
                width: 0,
                data: vec![],
            };
        }

        Self {
            height: lines.len(),
            width: lines[0].len(),
            data: lines.iter().flat_map(|line| line.chars()).collect(),
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<char> {
        if x >= self.width || y >= self.width {
            None
        } else {
            Some(self.data[self.width * y + x])
        }
    }

    fn set(&mut self, x: usize, y: usize, value: char) {
        if x >= self.width || y >= self.height {
            return;
        }

        self.data[self.width * y + x] = value;
    }

    fn count_neighbours(&self, x: usize, y: usize) -> usize {
        let mut count = 0;

        for r in y.saturating_sub(1)..(y + 2).min(self.height) {
            for c in x.saturating_sub(1)..(x + 2).min(self.width) {
                if r == y && c == x {
                    continue;
                }

                if let Some('@') = self.get(c, r) {
                    count += 1;
                }
            }
        }

        count
    }
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
