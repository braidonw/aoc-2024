advent_of_code::solution!(1);

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in input.lines() {
        let result: Vec<i32> = line
            .split_whitespace()
            .filter_map(|digit| digit.parse::<i32>().ok())
            .collect();

        if result.len() == 2 {
            left.push(result[0]);
            right.push(result[1]);
        }
    }

    (left, right)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = parse(input);

    left.sort();
    right.sort();

    let result: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut left, mut right) = parse(input);

    left.sort();
    right.sort();

    let mut frequency_counts: Vec<usize> = Vec::new();

    for l in left.iter() {
        let right_frequency = right.iter().filter(|r| *r == l).count();
        frequency_counts.push(right_frequency);
    }

    let result = left
        .iter()
        .zip(frequency_counts.iter())
        .map(|(l, r)| *l * (*r as i32))
        .sum::<i32>();

    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
