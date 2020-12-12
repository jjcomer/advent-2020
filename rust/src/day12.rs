#[aoc_generator(day12)]
pub fn parse_input(input: &str) -> Vec<(char, i64)> {
    input
        .lines()
        .map(|l| {
            let l = l.trim();
            (
                *l.as_bytes().first().unwrap() as char,
                l.get(1..).unwrap().parse().unwrap(),
            )
        })
        .collect()
}

fn turn_left(direction: char, degree: i64) -> char {
    if degree == 0 {
        return direction;
    }
    turn_left(
        match direction {
            'N' => 'W',
            'E' => 'N',
            'S' => 'E',
            'W' => 'S',
            _ => direction,
        },
        degree - 90,
    )
}

fn turn_right(direction: char, degree: i64) -> char {
    if degree == 0 {
        return direction;
    }

    turn_right(
        match direction {
            'N' => 'E',
            'E' => 'S',
            'S' => 'W',
            'W' => 'N',
            _ => direction,
        },
        degree - 90,
    )
}

fn move_ship(x: i64, y: i64, facing: char, direction: char, distance: i64) -> (i64, i64) {
    match direction {
        'N' => (x, y + distance),
        'E' => (x + distance, y),
        'S' => (x, y - distance),
        'W' => (x - distance, y),
        'F' => move_ship(x, y, facing, facing, distance),
        _ => (x, y),
    }
}

fn compute_map(map: &Vec<(char, i64)>) -> (i64, i64) {
    let mut x = 0;
    let mut y = 0;
    let mut facing = 'E';
    for (direction, distance) in map {
        //println!("{} {} {} -- {} {}", x, y, facing, direction, distance);
        match *direction {
            'L' => facing = turn_left(facing, *distance),
            'R' => facing = turn_right(facing, *distance),
            _ => {
                let (x1, y1) = move_ship(x, y, facing, *direction, *distance);
                x = x1;
                y = y1;
            }
        };
    }
    (x, y)
}

fn compute_distance(x: i64, y: i64) -> i64 {
    x.abs() + y.abs()
}

#[aoc(day12, part1)]
fn solve_part1(input: &Vec<(char, i64)>) -> i64 {
    let (x, y) = compute_map(&input);
    compute_distance(x, y)
}

fn turn_waypoint_left(x: i64, y: i64, degree: i64) -> (i64, i64) {
    match degree {
        90 => (-y, x),
        180 => (-x, -y),
        270 => (y, -x),
        _ => (x, y),
    }
}

fn turn_waypoint_right(x: i64, y: i64, degree: i64) -> (i64, i64) {
    match degree {
        90 => (y, -x),
        180 => (-x, -y),
        270 => (-y, x),
        _ => (x, y),
    }
}

fn compute_map_2(map: &Vec<(char, i64)>) -> (i64, i64) {
    let mut ship_x = 0;
    let mut ship_y = 0;
    let mut waypoint_x = 10;
    let mut waypoint_y = 1;

    for (direction, distance) in map {
        // println!(
        //     "{} {} {} {} -> {} {}",
        //     waypoint_x, waypoint_y, ship_x, ship_y, direction, distance
        // );
        match *direction {
            'N' => waypoint_y += *distance,
            'S' => waypoint_y -= *distance,
            'E' => waypoint_x += *distance,
            'W' => waypoint_x -= *distance,
            'L' => {
                let (x1, y1) = turn_waypoint_left(waypoint_x, waypoint_y, *distance);
                waypoint_x = x1;
                waypoint_y = y1;
            }
            'R' => {
                let (x1, y1) = turn_waypoint_right(waypoint_x, waypoint_y, *distance);
                waypoint_x = x1;
                waypoint_y = y1;
            }
            'F' => {
                ship_x = ship_x + (*distance * waypoint_x);
                ship_y = ship_y + (*distance * waypoint_y);
            }
            _ => (),
        };
    }
    (ship_x, ship_y)
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &Vec<(char, i64)>) -> i64 {
    let (x, y) = compute_map_2(input);
    compute_distance(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "F10
        N3
        F7
        R90
        F11";

        let input = parse_input(input);
        assert_eq!(25, solve_part1(&input));
    }

    #[test]
    fn part2() {
        let input = "F10
        N3
        F7
        R90
        F11";

        let input = parse_input(input);
        assert_eq!(286, solve_part2(&input));
    }
}
