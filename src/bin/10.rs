use std::{collections::HashSet, fmt::Debug};

advent_of_code::solution!(10);

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    row: i32,
    col: i32,
    height: usize,
}

impl Position {
    fn new(row: usize, col: usize, height: usize) -> Self {
        Self {
            row: row as i32,
            col: col as i32,
            height,
        }
    }
}

struct TrailMap {
    positions: Vec<Vec<Position>>,
    trailheads: Vec<Position>,
}

impl Debug for TrailMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TrailMap:\n")?;
        for row in &self.positions {
            for pos in row {
                write!(f, "{}", pos.height)?;
            }
            writeln!(f)?;
        }

        write!(f, "\nTrailheads:\n")?;
        for trailhead in &self.trailheads {
            write!(f, "\t{:?}\n", trailhead)?;
        }
        Ok(())
    }
}

fn parse_input(input: &str) -> TrailMap {
    let mut positions: Vec<Vec<Position>> = Vec::new();
    let mut trailheads: Vec<Position> = Vec::new();
    for (row_idx, row) in input.lines().enumerate() {
        let mut col_positions = Vec::new();
        for (col_idx, col) in row.chars().enumerate() {
            let height = col.to_digit(10).unwrap() as usize;
            let position = Position::new(row_idx, col_idx, height);
            col_positions.push(position);
            if height == 0 {
                trailheads.push(position);
            }
        }
        positions.push(col_positions);
    }

    TrailMap {
        positions,
        trailheads,
    }
}

impl TrailMap {
    fn get_position(&self, row: i32, col: i32) -> Option<Position> {
        if row < 0 || col < 0 {
            return None;
        }

        self.positions
            .get(row as usize)
            .and_then(|r| r.get(col as usize))
            .copied()
    }

    fn move_in(&self, position: &Position, direction: Direction) -> Option<Position> {
        let new_position = match direction {
            Direction::North => self.get_position(position.row - 1, position.col),
            Direction::East => self.get_position(position.row, position.col + 1),
            Direction::South => self.get_position(position.row + 1, position.col),
            Direction::West => self.get_position(position.row, position.col - 1),
        };
        new_position
    }

    fn score_trailhead(&self, trailhead: &Position) -> usize {
        let mut trail_ends: HashSet<Position> = HashSet::new();

        let mut working_positions: Vec<Position> = Vec::new();
        working_positions.push(*trailhead);

        while let Some(pos) = working_positions.pop() {
            for direction in DIRECTIONS {
                if let Some(next) = self.move_in(&pos, direction) {
                    match next.height {
                        9 if pos.height == 8 => {
                            trail_ends.insert(next);
                        }
                        height if height > 0 && height - 1 == pos.height => {
                            working_positions.push(next);
                        }
                        _ => {}
                    }
                }
            }
        }

        trail_ends.len()
    }

    fn find_distinct_trails(&self, trailhead: &Position) -> u32 {
        let mut trails: HashSet<Vec<Position>> = HashSet::new();

        let mut stack: Vec<Vec<Position>> = Vec::new();
        stack.push(vec![*trailhead]);

        while let Some(current_path) = stack.pop() {
            let current_pos = *current_path.last().unwrap();

            if current_pos.height == 9 {
                trails.insert(current_path.clone());
                continue;
            }

            for direction in DIRECTIONS {
                if let Some(next) = self.move_in(&current_pos, direction) {
                    if next.height > current_pos.height && next.height - 1 == current_pos.height {
                        let mut new_path = current_path.clone();
                        new_path.push(next);
                        stack.push(new_path);
                    }
                }
            }
        }

        trails.len() as u32
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let trailmap = parse_input(input);

    let score: u32 = trailmap
        .trailheads
        .iter()
        .map(|trailhead| trailmap.score_trailhead(trailhead) as u32)
        .sum();

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let trailmap = parse_input(input);
    let score: u32 = trailmap
        .trailheads
        .iter()
        .map(|trailhead| trailmap.find_distinct_trails(trailhead) as u32)
        .sum();

    Some(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
