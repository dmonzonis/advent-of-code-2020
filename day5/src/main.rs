use std::env;
use std::fs;
use std::io;

/// Parses the file in the current directory file into a string
fn file_to_string(filename: &str) -> Result<String, io::Error> {
    let mut file_path = env::current_dir()?;
    file_path.push(&filename);
    fs::read_to_string(file_path)
}

fn compute_seat_id(seat: &str) -> u32 {
    let mut front = 0;
    let mut back = 127;
    let mut left = 0;
    let mut right = 7;
    for c in seat.chars() {
        let vdist = back - front;
        let hdist = right - left;
        match c {
            'F' => back = front + vdist / 2,
            'B' => front = back - vdist / 2,
            'L' => right = left + hdist / 2,
            'R' => left = right - hdist / 2,
            _ => panic!("Unexpected character found"),
        }
    }
    if front != back || left != right {
        panic!("String {} did not converge into a seat", seat);
    }
    front * 8 + left
}

fn find_highest_seat_id(seats: Vec<String>) -> u32 {
    let mut highest = 0;
    for seat in seats {
        let seat_id = compute_seat_id(&seat);
        if seat_id > highest {
            highest = seat_id;
        }
    }
    highest
}

fn main() {
    let s = file_to_string("input.txt").unwrap();
    let seats = s.lines().map(|w| w.to_owned()).collect();
    println!("Part 1: {}", find_highest_seat_id(seats));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_seat_id() {
        assert_eq!(357, compute_seat_id("FBFBBFFRLR"));
        assert_eq!(567, compute_seat_id("BFFFBBFRRR"));
        assert_eq!(119, compute_seat_id("FFFBBBFRRR"));
        assert_eq!(820, compute_seat_id("BBFFBBFRLL"));
    }
}
