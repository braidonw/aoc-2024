use advent_of_code::util::grid::*;
use advent_of_code::util::point::*;

advent_of_code::solution!(16);

const DIRECTIONS: [Point; 4] = [RIGHT, DOWN, LEFT, UP];

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::parse(input);

    let start = grid.find(b'S').unwrap();
    let end = grid.find(b'E').unwrap();

    // Dijkstra forwards\n
    let mut bucket_queue = vec![Vec::new(); 1001];

    let mut seen = grid.same_size_with([u32::MAX; 4]);
    let mut cost = 0;
    let mut lowest = u32::MAX;

    bucket_queue[0].push((start, 0));
    seen[start][0] = 0;

    while lowest == u32::MAX {
        let index = (cost % 1001) as usize;

        while let Some((position, direction)) = bucket_queue[index].pop() {
            if position == end {
                lowest = cost;
                break;
            }

            let left = (direction + 3) % 4;
            let right = (direction + 1) % 4;

            let next_directions = [
                (position + DIRECTIONS[direction], direction, cost + 1),
                (position, left, cost + 1000),
                (position, right, cost + 1000),
            ];

            for (next_position, next_direction, next_cost) in next_directions {
                if grid[next_position] != b'#' && next_cost < seen[next_position][next_direction] {
                    let index = (next_cost % 1001) as usize;
                    bucket_queue[index].push((next_position, next_direction));
                    seen[next_position][next_direction] = next_cost;
                }
            }
        }

        cost += 1;
    }

    Some(lowest)
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
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
