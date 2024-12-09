use std::collections::VecDeque;

use miette::Error;

advent_of_code::solution!(9);

struct Blocks(VecDeque<Block>);

impl Blocks {
    fn checksum(&self) -> Option<u128> {
        let mut checksum: u128 = 0;
        for (i, block) in self.0.iter().enumerate() {
            match block {
                Block::File(id) => checksum += (id * i as u64) as u128,
                Block::EmptySpace => {}
            }
        }

        Some(checksum)
    }

    fn empty_space_idxs(&self) -> Vec<usize> {
        self.0
            .iter()
            .enumerate()
            .filter(|(_idx, block)| match block {
                Block::EmptySpace => true,
                _ => false,
            })
            .map(|(idx, _)| idx)
            .collect()
    }
}

impl std::fmt::Debug for Blocks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let block_strings: Vec<String> = self
            .0
            .iter()
            .map(|block| match block {
                Block::File(id) => id.to_string(),
                Block::EmptySpace => ".".to_string(),
            })
            .collect();
        write!(f, "{}", block_strings.join(" "))
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Block {
    File(u64),
    EmptySpace,
}

fn parse_input(input: &str) -> Blocks {
    let mut blocks = VecDeque::new();

    let mut parsing_file = true;
    let mut current_file_id = 0;
    for c in input.chars() {
        let c_num = c.to_digit(10).map(|d| d as usize).expect("Failed to parse");
        if parsing_file {
            for _ in 0..c_num {
                blocks.push_back(Block::File(current_file_id));
            }

            current_file_id += 1;
        } else {
            for _ in 0..c_num {
                blocks.push_back(Block::EmptySpace);
            }
        }

        parsing_file = !parsing_file;
    }

    Blocks(blocks)
}

fn compress(blocks: &mut Blocks) -> Result<(), Error> {
    'outer: for blank_idx in dbg!(blocks.empty_space_idxs()) {
        if blank_idx >= blocks.0.len() {
            break 'outer;
        }

        // pop off the empty spaces at the back
        while let Some(Block::EmptySpace) = blocks.0.back() {
            blocks.0.pop_back();
        }
        let block_to_insert = blocks
            .0
            .pop_back()
            .expect("Failed to get a reference to the back el of the blocks");

        // insert the block at the blank index
        blocks.0.insert(blank_idx, block_to_insert);
        blocks.0.remove(blank_idx + 1);
    }

    Ok(())
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut blocks = parse_input(input);
    let _ = compress(&mut blocks);
    dbg!(&blocks);

    let checksum = blocks.checksum().expect("Failed to calculate checksum");
    dbg!(&checksum);

    Some(checksum as u32)
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
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
