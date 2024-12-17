advent_of_code::solution!(15);
use std::{
    fmt::{self, Debug},
    str::FromStr,
};

use miette::Error;

#[derive(Copy, Clone)]
struct Point {
    row: i32,
    col: i32,
}

impl Point {
    fn next(&self, direction: Direction) -> Point {
        match direction {
            Direction::North => Point {
                row: self.row - 1,
                col: self.col,
            },
            Direction::South => Point {
                row: self.row + 1,
                col: self.col,
            },
            Direction::East => Point {
                row: self.row,
                col: self.col + 1,
            },
            Direction::West => Point {
                row: self.row,
                col: self.col - 1,
            },
        }
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Block {
    Empty,
    Wall,
    Box,
    Robot,
}

impl Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Block::Empty => write!(f, "_"),
            Block::Wall => write!(f, "#"),
            Block::Box => write!(f, "O"),
            Block::Robot => write!(f, "@"),
        }
    }
}

impl FromStr for Block {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Block::Empty),
            "#" => Ok(Block::Wall),
            "O" => Ok(Block::Box),
            "@" => Ok(Block::Robot),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Debug for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::North => write!(f, "^"),
            Direction::South => write!(f, "v"),
            Direction::East => write!(f, ">"),
            Direction::West => write!(f, "<"),
        }
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "^" => Ok(Direction::North),
            "v" => Ok(Direction::South),
            ">" => Ok(Direction::East),
            "<" => Ok(Direction::West),
            _ => Err(()),
        }
    }
}

struct Grid {
    grid: Vec<Vec<Block>>,
}

impl Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "\n")?;
        for row in &self.grid {
            for pos in row {
                write!(f, "{:?}", pos)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Grid {
    fn robot_position(&self) -> Point {
        self.grid
            .iter()
            .enumerate()
            .find_map(|(row, line)| {
                line.iter()
                    .position(|pos| pos == &Block::Robot)
                    .map(|col| Point {
                        row: row as i32,
                        col: col as i32,
                    })
            })
            .unwrap()
    }

    fn at(&self, point: Point) -> Block {
        self.grid[point.row as usize][point.col as usize]
    }

    fn set_at(&mut self, point: Point, block: Block) -> Result<(), Error> {
        self.grid[point.row as usize][point.col as usize] = block;
        Ok(())
    }

    fn get_total_gps(&self) -> u32 {
        let mut total: u32 = 0;
        for (i, row) in self.grid.iter().enumerate() {
            for (j, pos) in row.iter().enumerate() {
                if pos == &Block::Box {
                    total += (100 * i as u32) + j as u32;
                }
            }
        }
        total
    }

    fn handle_robot_movement(&mut self, direction: Direction) -> Result<(), Error> {
        let start = self.robot_position();
        let mut targets: Vec<Point> = Vec::new();
        let mut can_move: bool = true;
        let mut next = start;
        loop {
            next = next.next(direction);

            match self.at(next) {
                Block::Wall => {
                    can_move = false;
                    break;
                }

                Block::Box => {
                    targets.push(next);
                }

                Block::Empty => {
                    break;
                }

                Block::Robot => {
                    unreachable!("Shouldn't get to the robot square.")
                }
            }
        }

        if can_move {
            for point in targets {
                self.set_at(point.next(direction), Block::Box)?;
            }

            self.set_at(start, Block::Empty)?;
            self.set_at(start.next(direction), Block::Robot)?;
        }
        Ok(())
    }
}

// Small example
// ########
// #..O.O.#
// ##@.O..#
// #...O..#
// #.#.O..#
// #...O..#
// #......#
// ########

// <^^>>>vv<v>>v<<
fn parse_input(input: &str) -> (Grid, Vec<Direction>) {
    let mut lines = input.lines();
    let mut grid_lines = Vec::new();
    let mut directions = Vec::new();

    // Parse grid
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let row: Vec<Block> = line
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();
        grid_lines.push(row);
    }

    // Parse directions
    // Flatten all lines of directions into a single Vec
    while let Some(dir_line) = lines.next() {
        if dir_line.is_empty() {
            continue;
        }
        let line_directions: Vec<Direction> = dir_line
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();
        directions.extend(line_directions);
    }

    (Grid { grid: grid_lines }, directions)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut grid, directions) = parse_input(input);

    for direction in directions {
        grid.handle_robot_movement(direction).unwrap();
    }

    Some(grid.get_total_gps())
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
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
