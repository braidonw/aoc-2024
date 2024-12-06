use std::str::FromStr;

advent_of_code::solution!(4);

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug)]
struct Grid {
    line_length: i32,
    line_count: i32,
    lines: Vec<Vec<char>>,
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();

        let line_length = lines[0].len() as i32;
        let line_count = lines.len() as i32;
        Ok(Grid {
            lines,
            line_length,
            line_count,
        })
    }
}

impl Grid {
    fn find_xmas_words(&self) -> u32 {
        let mut count = 0;
        for i in 0..self.line_count {
            for j in 0..self.line_length {
                if self.lines[i as usize][j as usize] == 'X' {
                    for direction in DIRECTIONS.iter() {
                        if let Some(_) = self.find_next_xmas_word_character(1, *direction, i, j) {
                            count += 1;
                        }
                    }
                }
            }
        }

        count
    }

    fn find_next_xmas_word_character(
        &self,
        distance: i32,
        direction: (i32, i32),
        i: i32,
        j: i32,
    ) -> Option<usize> {
        let new_i = i + direction.0;
        let new_j = j + direction.1;
        if new_i < 0 || new_i >= self.line_count || new_j < 0 || new_j >= self.line_length {
            return None;
        }

        match (distance, self.lines[new_i as usize][new_j as usize]) {
            (1, 'M') | (2, 'A') => {
                self.find_next_xmas_word_character(distance + 1, direction, new_i, new_j)
            }
            (3, 'S') => return Some(1),
            _ => return None,
        }
    }

    fn find_x_mas_words(&self) -> u32 {
        let mut count = 0;
        for i in 1..self.line_count - 1 {
            for j in 1..self.line_length - 1 {
                if self.lines[i as usize][j as usize] == 'A' {
                    let top_left = self.lines[i as usize - 1][j as usize - 1];
                    let top_right = self.lines[i as usize - 1][j as usize + 1];
                    let bottom_left = self.lines[i as usize + 1][j as usize - 1];
                    let bottom_right = self.lines[i as usize + 1][j as usize + 1];

                    let diag_1 = (top_left, bottom_right);
                    let diag_2 = (bottom_left, top_right);

                    let diag_1_valid = diag_1 == ('S', 'M') || diag_1 == ('M', 'S');
                    let diag_2_valid = diag_2 == ('S', 'M') || diag_2 == ('M', 'S');

                    if diag_1_valid && diag_2_valid {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from_str(input).unwrap();
    let result = grid.find_xmas_words();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::from_str(input).unwrap();
    let result = grid.find_x_mas_words();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
