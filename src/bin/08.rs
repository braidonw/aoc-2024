use std::collections::HashMap;

use miette::Error;

advent_of_code::solution!(8);

#[derive(Debug, PartialEq, Eq)]
enum Node {
    Node,
    Antinode,
}

#[derive(Debug, PartialEq, Eq)]
struct Antenna {
    kind: char,
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct Grid {
    nodes: Vec<Vec<Node>>,
    antennas: Vec<Antenna>,
}

fn parse_input(input: &str) -> Grid {
    let mut nodes = Vec::new();
    let mut antennas = Vec::new();

    for (row, line) in input.lines().enumerate() {
        let mut node_row = Vec::new();

        for (col, c) in line.chars().enumerate() {
            node_row.push(Node::Node);
            if c != '.' {
                antennas.push(Antenna { kind: c, row, col });
            };
        }

        nodes.push(node_row);
    }

    Grid { nodes, antennas }
}

impl Grid {
    fn find_antinodes(&mut self, antenna_1: &Antenna, antenna_2: &Antenna) -> Result<(), Error> {
        let row_distance = (antenna_1.row as i32 - antenna_2.row as i32) * 2;
        let col_distance = (antenna_1.col as i32 - antenna_2.col as i32) * 2;

        let mut possible_antinodes: Vec<(usize, usize)> = Vec::new();

        Ok(())
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = parse_input(input);
    dbg!(grid);
    None
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
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
