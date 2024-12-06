#![allow(dead_code)]

use std::{fmt::Debug, str::FromStr};

use miette::Error;
advent_of_code::solution!(6);

#[derive(Debug, Copy, Clone)]
enum Position {
    Blocked,
    Empty,
    Guard,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

// # = Blocked, . = Empty
impl FromStr for Position {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(Position::Blocked),
            "." => Ok(Position::Empty),
            "^" => Ok(Position::Guard),
            _ => Err(format!("Invalid position: {}", s)),
        }
    }
}

#[derive(Clone)]
struct Map {
    positions: Vec<Vec<Position>>,
    visited_positions: Vec<Vec<bool>>,
    guard_position: (usize, usize),
    guard_direction: Direction,
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut guard_position = (0, 0);
        let positions: Vec<Vec<Position>> = s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == '^' {
                            guard_position = (x, y);
                            Position::Empty
                        } else {
                            c.to_string().parse::<Position>().unwrap()
                        }
                    })
                    .collect()
            })
            .collect();

        Ok(Map {
            visited_positions: vec![vec![false; positions[0].len()]; positions.len()],
            positions,
            guard_position,
            guard_direction: Direction::North,
        })
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for (y, row) in self.positions.iter().enumerate() {
            for (x, pos) in row.iter().enumerate() {
                if (x, y) == self.guard_position {
                    write!(f, "^")?;
                    continue;
                }
                match pos {
                    Position::Blocked => write!(f, "#")?,
                    Position::Empty => write!(f, ".")?,
                    Position::Guard => write!(f, "^")?,
                }
            }
            writeln!(f)?;
        }

        writeln!(f)?;
        // for row in self.visited_positions.iter() {
        //     for &visited in row.iter() {
        //         write!(f, "{}", if visited { "X" } else { "." })?;
        //     }
        //     writeln!(f)?;
        // }

        writeln!(f, "Guard position: {:?}", self.guard_position)?;
        writeln!(f, "Guard direction: {:?}", self.guard_direction)?;
        Ok(())
    }
}

impl Map {
    fn tick(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn turn_right(&mut self) {
        self.guard_direction = match self.guard_direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn simulate_plain(&mut self) -> Result<(), Error> {
        loop {
            self.visited_positions[self.guard_position.1][self.guard_position.0] = true;
            match self.position_in_front_of_guard() {
                None => break,
                Some(Position::Blocked) => {
                    self.turn_right();
                }
                Some(Position::Empty) => {
                    self.guard_position = match self.guard_direction {
                        Direction::North => (self.guard_position.0, self.guard_position.1 - 1),
                        Direction::East => (self.guard_position.0 + 1, self.guard_position.1),
                        Direction::South => (self.guard_position.0, self.guard_position.1 + 1),
                        Direction::West => (self.guard_position.0 - 1, self.guard_position.1),
                    };
                }
                Some(Position::Guard) => {
                    panic!("Guard should never be in front of guard");
                }
            }
        }
        Ok(())
    }

    fn position_in_front_of_guard(&self) -> Option<Position> {
        let (x, y) = self.guard_position;
        match self.guard_direction {
            Direction::North => {
                if y == 0 {
                    None
                } else {
                    Some(self.positions[y - 1][x])
                }
            }
            Direction::East => {
                if x == self.positions[0].len() - 1 {
                    None
                } else {
                    Some(self.positions[y][x + 1])
                }
            }
            Direction::South => {
                if y == self.positions.len() - 1 {
                    None
                } else {
                    Some(self.positions[y + 1][x])
                }
            }
            Direction::West => {
                if x == 0 {
                    None
                } else {
                    Some(self.positions[y][x - 1])
                }
            }
        }
    }

    fn count_visited_positions(&self) -> usize {
        self.visited_positions
            .iter()
            .flatten()
            .filter(|&&visited| visited)
            .count()
    }

    // Insert a blocked position into the provided position, then run the simulation
    // If we end up in a square we've been to before, then return true
    // If we end up going off the map, then false
    fn will_loop(&mut self) -> bool {
        let mut seen: Vec<((usize, usize), Direction)> = Vec::new();
        loop {
            // check if we've been here before
            if seen.contains(&(self.guard_position, self.guard_direction)) {
                return true;
            }
            match self.position_in_front_of_guard() {
                None => break,
                Some(Position::Blocked) => {
                    seen.push((self.guard_position, self.guard_direction));
                    self.turn_right();
                }
                Some(Position::Empty) => {
                    self.guard_position = match self.guard_direction {
                        Direction::North => (self.guard_position.0, self.guard_position.1 - 1),
                        Direction::East => (self.guard_position.0 + 1, self.guard_position.1),
                        Direction::South => (self.guard_position.0, self.guard_position.1 + 1),
                        Direction::West => (self.guard_position.0 - 1, self.guard_position.1),
                    };
                }
                Some(Position::Guard) => {
                    panic!("Guard should never be in front of guard");
                }
            }
        }

        false
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = input.parse::<Map>().unwrap();
    map.simulate_plain().unwrap();
    Some(map.count_visited_positions() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = input.parse::<Map>().unwrap();
    map.simulate_plain().unwrap();

    let mut looping_count = 0;

    let mut maps_to_test: Vec<Map> = Vec::with_capacity(map.count_visited_positions());

    for (j, line) in map.visited_positions.iter().enumerate() {
        for (i, pos) in line.iter().enumerate() {
            if !pos {
                continue;
            }

            let mut test_map = input.parse::<Map>().unwrap();
            test_map.positions[j][i] = Position::Blocked;
            maps_to_test.push(test_map);
        }
    }

    for mut test_map in maps_to_test {
        if test_map.will_loop() {
            looping_count += 1;
        }
    }

    Some(looping_count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
