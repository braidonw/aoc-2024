use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(12);

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

type Map = Vec<Vec<char>>;
type Region = HashSet<(i32, i32)>;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_neighboring_plots(
    position: (i32, i32),
    map: &Vec<Vec<char>>,
    current_plant: char,
) -> Vec<(i32, i32)> {
    let mut neighboring_plots = Vec::new();

    for dir in DIRECTIONS {
        let neighboring_position = (position.0 + dir.0, position.1 + dir.1);
        if (neighboring_position.0 as usize) < map.len()
            && (neighboring_position.1 as usize) < map[0].len()
        {
            if map[neighboring_position.0 as usize][neighboring_position.1 as usize]
                == current_plant
            {
                neighboring_plots.push(neighboring_position);
            }
        }
    }

    neighboring_plots
}

fn do_bfs(position: (i32, i32), map: &Map, visited: &mut Region) -> (Region, usize) {
    // Init vars
    let mut area = HashSet::new();
    let mut perimeter = 0;

    let mut queue = VecDeque::new();
    let current_plant = map[position.0 as usize][position.1 as usize];

    visited.insert(position);
    queue.push_back(position);

    while !queue.is_empty() {
        let current_position = queue.pop_front().unwrap();
        area.insert(current_position);

        let neighboring_plots = get_neighboring_plots(current_position, map, current_plant);
        perimeter += 4 - neighboring_plots.len();

        for plot in neighboring_plots {
            if !visited.contains(&plot) {
                visited.insert(plot);
                queue.push_back(plot);
            }
        }
    }

    (area, perimeter)
}

fn count_sides(region: &Region) -> usize {
    let mut count = 0;

    for direction in DIRECTIONS {
        let mut sides = HashSet::new();
        for plot in region {
            let temp = (plot.0 + direction.0, plot.1 + direction.1);
            if !region.contains(&temp) {
                sides.insert(temp);
            }
        }
        let mut remove = HashSet::new();
        for side in &sides {
            let mut temp = (side.0 + direction.1, side.1 + direction.0);
            while sides.contains(&temp) {
                remove.insert(temp);
                temp = (temp.0 + direction.1, temp.1 + direction.0);
            }
        }

        count += sides.len() - remove.len();
    }

    count
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_input(input);
    let mut visited = HashSet::new();

    let mut price = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if !visited.contains(&(i as i32, j as i32)) {
                let (area, perimeter) = do_bfs((i as i32, j as i32), &map, &mut visited);
                price += (area.len() * perimeter) as u32;
            }
        }
    }
    Some(price)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_input(input);
    let mut visited = HashSet::new();

    let mut price = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if !visited.contains(&(i as i32, j as i32)) {
                let (area, perimeter) = do_bfs((i as i32, j as i32), &map, &mut visited);
                let side_count = count_sides(&area);
                price += (area.len() * side_count) as u32;
            }
        }
    }
    Some(price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
