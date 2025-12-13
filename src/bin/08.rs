advent_of_code::solution!(8);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

impl JunctionBox {
    fn measure_distance(&self, other: &JunctionBox) -> i64 {
        ((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)).isqrt()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let junction_boxes: Vec<JunctionBox> = input
        .lines()
        .map(|line| {
            let coords: Vec<&str> = line.split(",").collect();

            JunctionBox {
                x: coords[0].parse().unwrap(),
                y: coords[1].parse().unwrap(),
                z: coords[2].parse().unwrap(),
            }
        })
        .collect();

    let mut connections: Vec<(i64, usize, usize)> = vec![];
    for i in 0..junction_boxes.len() {
        let start = &junction_boxes[i];

        for (j, end) in junction_boxes.iter().enumerate().skip(i + 1) {
            let distance = start.measure_distance(end);

            connections.push((distance, i, j));
        }
    }

    // dbg!(&connections);

    connections.sort_by_key(|a| a.0);

    let mut circuits: Vec<Vec<usize>> = vec![];
    for (_, first_idx, second_idx) in connections.iter().take(if junction_boxes.len() < 1000 {
        10
    } else {
        1000
    }) {
        let mut first_circuit: Option<usize> = None;
        let mut second_circuit: Option<usize> = None;

        // check if first or second junction are already in a circuit
        for (i, circuit) in circuits.iter().enumerate() {
            if circuit.contains(first_idx) {
                first_circuit = Some(i);
            }

            if circuit.contains(second_idx) {
                second_circuit = Some(i);
            }
        }

        match (first_circuit, second_circuit) {
            (None, None) => circuits.push(vec![*first_idx, *second_idx]),
            (Some(circuit_idx), None) => circuits[circuit_idx].push(*second_idx),
            (None, Some(circuit_idx)) => circuits[circuit_idx].push(*first_idx),
            (Some(a), Some(b)) => {
                if a == b {
                    continue;
                }

                if a < b {
                    let mut other = circuits.remove(b);
                    circuits[a].append(&mut other);
                } else {
                    let mut other = circuits.remove(a);
                    circuits[b].append(&mut other);
                }
            }
        }
    }

    circuits.sort_by_key(|a| std::cmp::Reverse(a.len()));
    // dbg!(&circuits);

    Some(
        circuits
            .split_at(3)
            .0
            .iter()
            .map(|circuit| circuit.len() as u64)
            .product(),
    )
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
