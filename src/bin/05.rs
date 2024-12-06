#![allow(dead_code)]
use std::str::FromStr;

advent_of_code::solution!(5);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PageNumber(u32);

impl FromStr for PageNumber {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(PageNumber).map_err(|_| ())
    }
}

#[derive(Debug, Clone, Copy)]
struct Rule {
    before: PageNumber,
    after: PageNumber,
}

impl FromStr for Rule {
    type Err = ();

    // Expected input: 47|53
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (before, after) = s.split_once('|').ok_or(())?;
        let before = before.parse().map_err(|_| ())?;
        let after = after.parse().map_err(|_| ())?;
        Ok(Rule { before, after })
    }
}

// A sleigh safety manual update
#[derive(Debug)]
struct Update {
    pages: Vec<PageNumber>,
}

impl FromStr for Update {
    type Err = ();

    // Expected input: 47|53
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pages = s
            .split(',')
            .map(|page| page.parse().map_err(|_| ()))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Update { pages })
    }
}

impl Update {
    fn satisfies_rules(&self, rules: &Vec<Rule>) -> bool {
        let filtered_rules: Vec<&Rule> = rules
            .iter()
            .filter(|rule| self.pages.contains(&rule.before) && self.pages.contains(&rule.after))
            .collect();

        for rule in filtered_rules {
            let before_idx = self
                .pages
                .iter()
                .position(|&page| page == rule.before)
                .unwrap();

            let after_idx = self
                .pages
                .iter()
                .position(|&page| page == rule.after)
                .unwrap();

            if after_idx < before_idx {
                return false;
            }
        }

        true
    }

    fn middle_page(&self) -> Option<&PageNumber> {
        let num_pages = self.pages.len();
        self.pages.iter().nth(num_pages / 2)
    }
}

fn parse_input(input: &str) -> Result<(Vec<Rule>, Vec<Update>), ()> {
    let mut sections = input.split("\n\n");
    let rules = sections
        .next()
        .ok_or(())?
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<_>, _>>()?;

    let updates: Vec<Update> = sections
        .next()
        .ok_or(())?
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<_>, _>>()?;

    Ok((rules, updates))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = parse_input(input).ok()?;

    let valid_updates: Vec<&Update> = updates
        .iter()
        .filter(|update| update.satisfies_rules(&rules))
        .collect::<Vec<_>>();

    let result: u32 = valid_updates
        .iter()
        .map(|update| update.middle_page())
        .flatten()
        .map(|page| page.0)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
