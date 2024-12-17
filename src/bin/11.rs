use std::collections::HashMap;
use std::mem;

use nom::character::complete;
use nom::{character::complete::space1, multi::separated_list1, IResult};

advent_of_code::solution!(11);

#[derive(Debug, Hash, Eq, Copy, Clone)]
struct Stone(u64);

impl PartialEq for Stone {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Stone {
    fn apply_blink(&self) -> (Stone, Option<Stone>) {
        if self.0 == 0 {
            return (Stone(1), None);
        }

        let num_digits = { (self.0 as f64).log10().floor() as u64 + 1 };

        if num_digits % 2 == 0 {
            let (left, right) = split_number(self.0);
            return (Stone(left), Some(Stone(right)));
        }

        let new_stone_value = self.0 * 2024;
        (Stone(new_stone_value), None)
    }
}

fn split_number(n: u64) -> (u64, u64) {
    let digit_count = (n as f64).log10().floor() as u32 + 1;
    assert!(
        digit_count % 2 == 0,
        "Number must have even number of digits"
    );

    let half_digits = digit_count / 2;
    let divisor = 10_u32.pow(half_digits);

    let right_half = n % divisor as u64;
    let left_half = n / divisor as u64;

    (left_half, right_half)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Stone>> {
    let (input, numbers) = separated_list1(space1, complete::u64)(input)?;

    Ok((input, numbers.into_iter().map(|n| Stone(n)).collect()))
}

// pub fn part_one(input: &str) -> Option<u64> {
//     let (_, mut stones) = parse_input(input).unwrap();
//     let mut new_stones: Vec<Stone> = Vec::new();

//     // cache
//     let mut seen: HashMap<Stone, (Stone, Option<Stone>)> = HashMap::new();

//     for i in 0..25 {
//         new_stones.clear();
//         for stone in &stones {
//             if let Some((new_stone, new_stone_right)) = seen.get(stone) {
//                 new_stones.push(*new_stone);
//                 if let Some(right) = new_stone_right {
//                     new_stones.push(*right);
//                 }
//                 continue;
//             }

//             let (new_stone, new_stone_right) = stone.apply_blink();
//             seen.insert(*stone, (new_stone, new_stone_right));
//             new_stones.push(new_stone);
//             if let Some(right) = new_stone_right {
//                 new_stones.push(right);
//             }
//         }
//         mem::swap(&mut stones, &mut new_stones);
//     }
//     Some(stones.len() as u64)
// }
pub fn part_one(input: &str) -> Option<u64> {
    let (_, stones) = parse_input(input).unwrap();

    // cache
    let mut cache: HashMap<(Stone, usize), u64> = HashMap::new();

    let result = stones
        .iter()
        .map(|stone| count(stone, 25, &mut cache))
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, stones) = parse_input(input).unwrap();

    // cache
    let mut cache: HashMap<(Stone, usize), u64> = HashMap::new();

    let result = stones
        .iter()
        .map(|stone| count(stone, 75, &mut cache))
        .sum();

    Some(result)
}

fn count(stone: &Stone, steps_remaining: usize, cache: &mut HashMap<(Stone, usize), u64>) -> u64 {
    if let Some(&result) = cache.get(&(*stone, steps_remaining)) {
        return result;
    }

    if steps_remaining == 0 {
        return 1;
    }

    if stone == &Stone(0) {
        return count(&Stone(1), steps_remaining - 1, cache);
    }

    let num_digits = { (stone.0 as f64).log10().floor() as u64 + 1 };

    if num_digits % 2 == 0 {
        let (left, right) = split_number(stone.0);

        let result = count(&Stone(left), steps_remaining - 1, cache)
            + count(&Stone(right), steps_remaining - 1, cache);

        cache.insert((*stone, steps_remaining), result);
        return result;
    }

    let result = count(&Stone(stone.0 * 2024), steps_remaining - 1, cache);
    cache.insert((*stone, steps_remaining), result);
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_split_number() {
        assert_eq!(split_number(123456), (123, 456));
        assert_eq!(split_number(12345678), (1234, 5678));
    }
}
