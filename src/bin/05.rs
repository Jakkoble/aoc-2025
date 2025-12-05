advent_of_code::solution!(5);

pub fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut fresh_ranges: Vec<(u64, u64)> = vec![];
    let mut ingredients: Vec<u64> = vec![];

    let mut shows_ingredients = false;
    for line in input.lines().map(|line| line.trim()) {
        if line.is_empty() {
            shows_ingredients = true;
            continue;
        }

        if shows_ingredients {
            ingredients.push(line.parse().unwrap());
        } else {
            let (first, second) = line.split_once('-').unwrap();
            fresh_ranges.push((first.parse().unwrap(), second.parse().unwrap()));
        }
    }
    (fresh_ranges, ingredients)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (fresh_ranges, ingredients) = parse_input(input);
    let mut fresh_indices: Vec<usize> = vec![];

    fresh_ranges.iter().for_each(|range| {
        for (i, ingredient) in ingredients.iter().enumerate() {
            if &range.0 <= ingredient && ingredient <= &range.1 && !fresh_indices.contains(&i) {
                fresh_indices.push(i);
            }
        }
    });

    Some(fresh_indices.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut fresh_ranges, _) = parse_input(input);

    fresh_ranges.sort_by(|(start_a, _), (start_b, _)| start_a.cmp(start_b));

    let mut final_ranges: Vec<(u64, u64)> = vec![fresh_ranges[0]];

    for (start, end) in &mut fresh_ranges[1..] {
        let last = final_ranges
            .last_mut()
            .expect("No element in final_ranges found!");

        if *start > last.1 {
            final_ranges.push((*start, *end));
            continue;
        }

        if *end <= last.1 {
            continue;
        }

        *last = (last.0, *end);
    }

    let mut amount = 0;
    for (start, end) in final_ranges {
        amount += end - start + 1;
    }

    Some(amount)
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
        assert_eq!(result, Some(14));
    }
}
