use miette::Error;

advent_of_code::solution!(9);

struct Blocks(Vec<Option<File>>);
struct File(u64);

impl Blocks {
    fn checksum(&self) -> Option<u64> {
        let mut checksum: u64 = 0;
        for (i, block) in self.0.iter().enumerate() {
            if let Some(File(id)) = block {
                checksum += (i as u64) * (*id as u64);
            }
        }

        Some(checksum)
    }
}

impl std::fmt::Debug for Blocks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let block_strings: Vec<String> = self
            .0
            .iter()
            .map(|block| match block {
                Some(File(id)) => id.to_string(),
                None => ".".to_string(),
            })
            .collect();
        write!(f, "{}", block_strings.join(" "))
    }
}

fn parse_input(input: &str) -> Blocks {
    let mut blocks = Vec::new();

    let mut parsing_file = true;
    let mut current_file_id = 0;
    for c in input.chars() {
        let c_num = c.to_digit(10).map(|d| d as usize).expect("Failed to parse");
        if parsing_file {
            for _ in 0..c_num {
                blocks.push(Some(File(current_file_id)));
            }

            current_file_id += 1;
        } else {
            for _ in 0..c_num {
                blocks.push(None);
            }
        }

        parsing_file = !parsing_file;
    }

    Blocks(blocks)
}

#[derive(Debug, Copy, Clone)]
struct FileBlock {
    file_id: usize,
    start_position: usize,
    length: usize,
}

#[derive(Debug, Eq, PartialEq)]
struct BlankSpace {
    start_position: usize,
    length: usize,
}

fn parse_files_and_blanks(input: &str) -> (Vec<FileBlock>, Vec<BlankSpace>) {
    let mut files = Vec::new();
    let mut blanks = Vec::new();

    let mut parsing_file = true;
    let mut current_file_id = 0;
    let mut position = 0;

    for c in input.chars() {
        let c_num = c.to_digit(10).map(|d| d as usize).expect("Failed to parse");
        if parsing_file {
            let file = FileBlock {
                file_id: current_file_id,
                start_position: position,
                length: c_num,
            };
            files.push(file);
            current_file_id += 1;
        } else {
            let blank = BlankSpace {
                start_position: position,
                length: c_num,
            };
            blanks.push(blank);
        }
        parsing_file = !parsing_file;
        position += c_num;
    }

    (files, blanks)
}

fn compress(blocks: &mut Blocks) -> Result<(), Error> {
    let mut front_idx = 0;
    let mut back_idx = blocks.0.len() - 1;

    while front_idx < back_idx {
        while front_idx < back_idx && blocks.0[front_idx].is_some() {
            front_idx += 1;
        }

        while back_idx > front_idx && blocks.0[back_idx].is_none() {
            back_idx -= 1;
        }

        if front_idx < back_idx {
            blocks.0.swap(front_idx, back_idx);
        }
    }

    Ok(())
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut blocks = parse_input(input);
    let _ = compress(&mut blocks);

    let checksum = blocks.checksum().expect("Failed to calculate checksum");
    dbg!(&checksum);

    Some(checksum as u32)
}

fn compress_p2(
    files: &mut Vec<FileBlock>,
    blanks: &mut Vec<BlankSpace>,
) -> Result<Vec<FileBlock>, Error> {
    // Start from the last file
    let mut file_idx = files.len();

    while file_idx > 0 {
        file_idx -= 1;
        let file = files.get_mut(file_idx).unwrap();
        let mut blank_idx = 0;
        let mut should_remove_blank = false;

        loop {
            if blank_idx > blanks.len() {
                break;
            }
            let blank = blanks.get_mut(blank_idx).expect("Failed to get blank");
            blank_idx += 1;

            if blank.start_position >= file.start_position {
                blanks.truncate(blank_idx);
                break;
            }

            if file.length <= blank.length {
                file.start_position = blank.start_position;

                if file.length == blank.length {
                    should_remove_blank = true;
                } else {
                    blank.start_position += file.length;
                    blank.length -= file.length;
                }
                break;
            }
        }

        if should_remove_blank {
            blanks.remove(blank_idx - 1);
        }
    }
    Ok(files.clone())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut files, mut blanks) = parse_files_and_blanks(input);
    let compressed_files = compress_p2(&mut files, &mut blanks).expect("Failed to compress");

    let mut total: u64 = 0;
    for file in compressed_files.iter() {
        for x in file.start_position..(file.start_position + file.length) {
            total += (x as u64) * (file.file_id as u64);
        }
    }
    dbg!(&total);

    Some(total as u32)
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
        assert_eq!(result, Some(2858));
    }
}
