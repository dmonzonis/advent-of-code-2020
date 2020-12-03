use std::env;
use std::fs;
use std::io;

/// Parses the file in the current directory file into a string
fn file_to_string(filename: &str) -> Result<String, io::Error> {
    let mut file_path = env::current_dir()?;
    file_path.push(&filename);
    fs::read_to_string(file_path)
}

fn build_map(s: &str) -> Vec<Vec<bool>> {
    let mut vec: Vec<Vec<bool>> = Vec::new();
    for line in s.lines() {
        let mut row: Vec<bool> = Vec::new();
        for c in line.chars() {
            row.push(c == '#');
        }
        vec.push(row);
    }
    vec
}

fn traverse_count(map: &Vec<Vec<bool>>, slope_x: usize, slope_y: usize) -> u64 {
    let mut count = 0;
    // if the data is correctly parsed, we can assume that the length of all rows is the same
    let width = map[0].len();
    let mut x = 0;
    let mut y = 0;
    while y + slope_y < map.len() {
        y += slope_y;
        x += slope_x;
        x %= width;
        if map[y][x] {
            count += 1;
        }
    }
    count
}

fn main() {
    let map = build_map(&file_to_string("input.txt").unwrap());
    println!("Part 1: {}", traverse_count(&map, 3, 1));
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut total = 1;
    for (slope_x, slope_y) in slopes.iter() {
        total *= traverse_count(&map, *slope_x as usize, *slope_y as usize);
    }
    println!("Part 2: {}", total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let map = build_map(&file_to_string("test.txt").unwrap());
        assert_eq!(traverse_count(&map, 3, 1), 7);
    }

    #[test]
    fn test_part_2() {
        let map = build_map(&file_to_string("test.txt").unwrap());
        let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let mut total = 1;
        for (slope_x, slope_y) in slopes.iter() {
            total *= traverse_count(&map, *slope_x as usize, *slope_y as usize);
        }
        assert_eq!(total, 336);
    }
}
