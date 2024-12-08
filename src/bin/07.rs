use nom::character::complete::{char, digit1, space1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::IResult;
use std::str::FromStr;

advent_of_code::solution!(7);

#[derive(Debug, PartialEq)]
struct Test {
    test_value: i64,
    numbers: Vec<i64>,
}

impl FromStr for Test {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, test) = parse_line(s).map_err(|_| ())?;
        Ok(test)
    }
}

impl Test {}

fn validate(current: i64, numbers: &[i64], include_concat: bool) -> bool {
    if numbers.is_empty() {
        return current == 0;
    }

    if current < 0 {
        return false;
    }

    let next_number = numbers[numbers.len() - 1];

    let m = 10_i64.pow(next_number.ilog10() + 1);

    (include_concat
        && current % m == next_number
        && validate(current / m, &numbers[..&numbers.len() - 1], include_concat))
        || (current % next_number == 0
            && validate(
                current / next_number,
                &numbers[..numbers.len() - 1],
                include_concat,
            ))
        || validate(
            current - next_number,
            &numbers[..numbers.len() - 1],
            include_concat,
        )
}

// example line:  3267: 81 40 27
fn parse_line(input: &str) -> IResult<&str, Test> {
    let (input, test_value) = map_res(digit1, str::parse)(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = space1(input)?;
    let (input, numbers) = separated_list1(space1, map_res(digit1, str::parse))(input)?;

    Ok((
        input,
        Test {
            test_value,
            numbers,
        },
    ))
}

pub fn part_one(input: &str) -> Option<u32> {
    let tests = input
        .lines()
        .map(|line| line.parse::<Test>().unwrap())
        .collect::<Vec<_>>();

    let mut result: i64 = 0;
    for test in tests {
        if validate(test.test_value, &test.numbers, false) {
            result += test.test_value;
        }
    }
    dbg!(result);
    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let tests = input
        .lines()
        .map(|line| line.parse::<Test>().unwrap())
        .collect::<Vec<_>>();

    let mut result: i64 = 0;
    for test in tests {
        if validate(test.test_value, &test.numbers, true) {
            result += test.test_value;
        }
    }
    dbg!(result);
    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
