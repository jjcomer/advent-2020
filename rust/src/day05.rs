use std::collections::HashSet;

pub struct BoardingPass {
    rows: String,
    columns: String,
}

#[aoc_generator(day5)]
pub fn parse_input(input: &str) -> Vec<BoardingPass> {
    input
        .lines()
        .map(|l| BoardingPass {
            rows: l[0..7].to_owned(),
            columns: l[7..].to_owned(),
        })
        .collect()
}

fn find_location(map: &str, max: i32, up: char) -> i32 {
    let mut min: i32 = 0;
    let mut max: i32 = max;

    for c in map.chars() {
        //println!("{} {} {}", c, min, max);
        let offset = (max + min) / 2;
        if c == up {
            max = offset - 1;
        } else {
            min = offset + 1;
        }
    }
    min
}

fn find_seat(pass: &BoardingPass) -> i32 {
    let row = find_location(&pass.rows, 127, 'F');
    let column = find_location(&pass.columns, 7, 'L');

    //println!("Row {}, Column {}", row, column);

    (row * 8) + column
}

#[aoc(day5, part1)]
fn solve_part1(input: &Vec<BoardingPass>) -> i32 {
    input.iter().map(find_seat).max().unwrap()
}

#[aoc(day5, part2)]
fn solve_part2(input: &Vec<BoardingPass>) -> i32 {
    let seats = input.iter().map(find_seat).collect::<HashSet<i32>>();

    for seat in seats.iter() {
        if seats.contains(&(seat + 2)) && !seats.contains(&(seat + 1)) {
            return seat + 1;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rows() {
        let test = parse_input(&"FBFBBFFRLR");
        assert_eq!(357, solve_part1(&test));
    }
}
