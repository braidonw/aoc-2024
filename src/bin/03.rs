use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

advent_of_code::solution!(3);

#[derive(Debug, Clone)]
enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

#[derive(Debug, PartialEq, Eq)]
enum ShouldProcess {
    Do,
    Dont,
}

fn mul(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::i32, tag(","), complete::i32),
        tag(")"),
    )(input)?;

    Ok((input, Instruction::Mul(pair.0, pair.1)))
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Dont, tag("don't")),
        value(Instruction::Do, tag("do")),
        mul,
    ))(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(many_till(anychar, instruction).map(|(_discard, ins)| ins))(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_input, instructions) = match parse_input(input) {
        Ok(result) => result,
        Err(_e) => return None,
    };
    let (_, result) = instructions
        .iter()
        .fold((ShouldProcess::Do, 0), |(process, acc), ins| match ins {
            Instruction::Mul(x, y) => (process, acc + (x * y)),
            Instruction::Do => (ShouldProcess::Do, acc),
            Instruction::Dont => (ShouldProcess::Dont, acc),
        });

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_input, instructions) = match parse_input(input) {
        Ok(result) => result,
        Err(_e) => return None,
    };
    let (_, result) = instructions
        .iter()
        .fold((ShouldProcess::Do, 0), |(process, acc), ins| match ins {
            Instruction::Mul(x, y) => {
                if process == ShouldProcess::Do {
                    (process, acc + (x * y))
                } else {
                    (process, acc)
                }
            }
            Instruction::Do => (ShouldProcess::Do, acc),
            Instruction::Dont => (ShouldProcess::Dont, acc),
        });

    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
