use std::str::FromStr;

advent_of_code::solution!(2);

#[derive(Debug, Clone)]
struct Line {
    reports: Vec<i32>,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let reports = s
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i32>>();

        Ok(Self { reports })
    }
}

fn is_safe(line: &Line, can_remove: bool) -> bool {
    let direction = (line.reports[1] - line.reports[0]).signum();

    for i in 0..line.reports.len() - 1 {
        let mut diff = line.reports[i + 1] - line.reports[i];
        let diff_sign = diff.signum();
        diff = diff.abs();

        if direction == 0 || diff_sign != direction || diff > 3 || diff < 1 {
            if can_remove {
                return line
                    .reports
                    .iter()
                    .enumerate()
                    .any(|(j, _)| is_safe(&remove_at(line, j), false));
            }
            return false;
        }
    }

    return true;
}

fn remove_at(line: &Line, index: usize) -> Line {
    let mut new_line = line.reports.clone();
    new_line.remove(index);
    Line { reports: new_line }
}

fn parse_input(input: &str) -> Vec<Line> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = parse_input(input);
    let result = lines.iter().filter(|line| is_safe(line, false)).count();

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = parse_input(input);
    let result = lines.iter().filter(|line| is_safe(line, true)).count();
    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
