use std::collections::HashMap;

advent_of_code::solution!(7);

fn add_beam_if_new(x: usize, y: usize, beams: &mut Vec<(usize, usize)>) {
    if beams.iter().any(|beam| beam.0 == x && beam.1 == y) {
        return;
    }

    beams.push((x, y));
}

pub fn part_one(input: &str) -> Option<u64> {
    let manifold = Manifold::new(input);
    let mut beams: Vec<(usize, usize)> = vec![manifold.start];
    let mut splits = 0;

    for _ in 1..manifold.height {
        let mut new_beams: Vec<(usize, usize)> = vec![];
        beams
            .iter()
            .filter_map(|beam| {
                let target = manifold.get(beam.0, beam.1 + 1);
                if target.is_none() {
                    None
                } else {
                    Some((beam.0, beam.1 + 1, target))
                }
            })
            .for_each(|(x, y, value)| {
                if value.unwrap() != b'^' {
                    add_beam_if_new(x, y, &mut new_beams);
                } else {
                    splits += 1;
                    add_beam_if_new(x - 1, y, &mut new_beams);
                    add_beam_if_new(x + 1, y, &mut new_beams);
                }
            });

        beams = new_beams;
    }

    Some(splits)
}

pub fn travel_timeline(
    start: (usize, usize),
    manifold: &Manifold,
    timelines: &mut u64,
    memo: &mut HashMap<(usize, usize), u64>,
) {
    if let Some(count) = memo.get(&start) {
        *timelines += count;
        return;
    }

    let initial_timelines = *timelines;
    *timelines += 1;

    let mut x = start.0;
    for y in (start.1)..manifold.height {
        if let Some(next) = manifold.get(x, y + 1)
            && next == b'^'
        {
            travel_timeline((x + 1, y + 1), manifold, timelines, memo);
            x -= 1;
        }
    }

    memo.insert(start, *timelines - initial_timelines);
}

pub fn part_two(input: &str) -> Option<u64> {
    let manifold = Manifold::new(input);
    let mut timelines: u64 = 0;
    let mut memo: HashMap<(usize, usize), u64> = HashMap::new();

    travel_timeline(manifold.start, &manifold, &mut timelines, &mut memo);

    Some(timelines)
}

#[derive(Debug)]
pub struct Manifold {
    pub height: usize,
    pub width: usize,
    pub start: (usize, usize),
    pub fields: Vec<u8>,
}

impl Manifold {
    fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        if lines.is_empty() {
            panic!("No lines provides");
        }

        let width = lines[0].len();

        Self {
            height: lines.len(),
            width,
            start: (width / 2, 0),
            fields: lines.iter().flat_map(|line| line.bytes()).collect(),
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<u8> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(self.fields[self.width * y + x])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
