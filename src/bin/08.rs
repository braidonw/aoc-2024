use std::collections::HashMap;

use miette::Error;

advent_of_code::solution!(8);

#[derive(Debug, PartialEq, Eq)]
enum Node {
    Node,
    Antinode,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Frequency(char);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Antenna {
    kind: Frequency,
    row: i64,
    col: i64,
}

#[derive(Debug)]
struct Grid {
    nodes: Vec<Vec<Node>>,
    antennas: HashMap<Frequency, Vec<Antenna>>,
}

fn parse_input(input: &str) -> Grid {
    let mut nodes = Vec::new();
    let mut antennas = HashMap::new();

    for (row, line) in input.lines().enumerate() {
        let mut node_row = Vec::new();

        for (col, c) in line.chars().enumerate() {
            node_row.push(Node::Node);
            if c != '.' {
                antennas
                    .entry(Frequency(c))
                    .or_insert(Vec::new())
                    .push(Antenna {
                        kind: Frequency(c),
                        row: row as i64,
                        col: col as i64,
                    });
            };
        }

        nodes.push(node_row);
    }

    Grid { nodes, antennas }
}

impl Grid {
    fn contains(&self, row: usize, col: usize) -> bool {
        row < self.nodes.len() && col < self.nodes[0].len()
    }

    fn find_resonant_frequency_antinodes(&mut self, frequency: Frequency) -> Result<(), Error> {
        if self.antennas.get(&frequency).is_none() {
            return Err(miette::miette!("No antennas with that frequency"));
        }

        if self.antennas.get(&frequency).unwrap().len() < 2 {
            return Err(miette::miette!("Not enough antennas with that frequency"));
        }

        for antenna_1 in self.antennas.get(&frequency).unwrap() {
            for antenna_2 in self.antennas.get(&frequency).unwrap() {
                if antenna_1 == antenna_2 {
                    continue;
                }

                let row_diff = antenna_2.row as i64 - antenna_1.row as i64;
                let col_diff = antenna_2.col as i64 - antenna_1.col as i64;

                let mut antinode = (antenna_2.row, antenna_2.col);

                loop {
                    self.nodes[antinode.0 as usize][antinode.1 as usize] = Node::Antinode;
                    antinode = (antinode.0 + row_diff, antinode.1 + col_diff);
                    if !self.contains(antinode.0 as usize, antinode.1 as usize) {
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    fn find_antinodes(&mut self, frequency: Frequency) -> Result<(), Error> {
        if self.antennas.get(&frequency).is_none() {
            return Err(miette::miette!("No antennas with that frequency"));
        }

        if self.antennas.get(&frequency).unwrap().len() < 2 {
            return Err(miette::miette!("Not enough antennas with that frequency"));
        }

        for antenna_1 in self.antennas.get(&frequency).unwrap() {
            for antenna_2 in self.antennas.get(&frequency).unwrap() {
                if antenna_1 == antenna_2 {
                    continue;
                }

                let row_diff = antenna_2.row as i64 - antenna_1.row as i64;
                let col_diff = antenna_2.col as i64 - antenna_1.col as i64;

                let (possible_row, possible_col) =
                    (antenna_2.row + row_diff, antenna_2.col + col_diff);

                if possible_row < 0 || possible_col < 0 {
                    continue;
                }

                if self.contains(possible_row as usize, possible_col as usize) {
                    self.nodes[possible_row as usize][possible_col as usize] = Node::Antinode;
                }
            }
        }

        Ok(())
    }

    fn count_antinodes(&self) -> usize {
        self.nodes
            .iter()
            .flatten()
            .filter(|&n| *n == Node::Antinode)
            .count()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = parse_input(input);
    let frequencies: Vec<_> = grid.antennas.keys().cloned().collect();
    for frequency in frequencies {
        grid.find_antinodes(frequency).unwrap();
    }
    Some(grid.count_antinodes() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = parse_input(input);
    let frequencies: Vec<_> = grid.antennas.keys().cloned().collect();
    for frequency in frequencies {
        grid.find_resonant_frequency_antinodes(frequency).unwrap();
    }
    Some(grid.count_antinodes() as u32)
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
        assert_eq!(result, Some(34));
    }
}
