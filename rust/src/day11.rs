use std::thread::current;

#[aoc_generator(day11)]
pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.trim().chars().collect()).collect()
}

fn get_location(map: &Vec<Vec<char>>, x: usize, y: usize) -> char {
    *map.get(y).unwrap().get(x).unwrap()
}

fn check_neighbours(map: &Vec<Vec<char>>, x: usize, y: usize) -> char {
    let current_location = get_location(map, x, y);
    if current_location == '.' {
        return '.';
    }

    let mut occupied = 0;
    let x = x as i32;
    let y = y as i32;
    let max_y = map.len() as i32;
    let max_x = map.first().unwrap().len() as i32;

    for (x1, y1) in &[
        (x + 1, y),
        (x - 1, y),
        (x, y + 1),
        (x, y - 1),
        (x + 1, y + 1),
        (x + 1, y - 1),
        (x - 1, y - 1),
        (x - 1, y + 1),
    ] {
        if *x1 >= max_x || *x1 < 0 || *y1 >= max_y || *y1 < 0 {
            continue;
        }
        if get_location(map, *x1 as usize, *y1 as usize) == '#' {
            occupied += 1;
        }
    }

    if current_location == '#' && occupied >= 4 {
        'L'
    } else if current_location == 'L' && occupied == 0 {
        '#'
    } else {
        current_location
    }
}

fn look_further(map: &Vec<Vec<char>>, x: i32, y: i32, x_delta: i32, y_delta: i32) -> bool {
    let mut x1 = x + x_delta;
    let mut y1 = y + y_delta;

    let max_y = map.len() as i32;
    let max_x = map.first().unwrap().len() as i32;

    loop {
        if x1 >= max_x || x1 < 0 || y1 >= max_y || y1 < 0 {
            break;
        }
        match get_location(map, x1 as usize, y1 as usize) {
            '#' => return true,
            'L' => return false,
            _ => (),
        };
        x1 += x_delta;
        y1 += y_delta;
    }
    false
}

fn check_neighbours2(map: &Vec<Vec<char>>, x: usize, y: usize) -> char {
    let current_location = get_location(map, x, y);
    if current_location == '.' {
        return '.';
    }
    let x = x as i32;
    let y = y as i32;
    let mut occupied = 0;

    for (x_delta, y_delta) in &[
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, -1),
        (-1, 1),
    ] {
        if look_further(map, x, y, *x_delta, *y_delta) {
            occupied += 1;
        }
    }

    if current_location == '#' && occupied >= 5 {
        'L'
    } else if current_location == 'L' && occupied == 0 {
        '#'
    } else {
        current_location
    }
}

fn compute_cycle(map: &Vec<Vec<char>>, part2: bool) -> (bool, Vec<Vec<char>>) {
    let mut new_map = Vec::new();
    let mut changed = false;
    for (y, line) in map.iter().enumerate() {
        let mut new_line = Vec::new();
        for (x, old_char) in line.iter().enumerate() {
            let new_char = if part2 {
                check_neighbours2(map, x, y)
            } else {
                check_neighbours(map, x, y)
            };
            if new_char != *old_char {
                changed = true;
            }
            new_line.push(new_char);
        }
        new_map.push(new_line);
    }
    (changed, new_map)
}

fn find_equilibrium(map: &Vec<Vec<char>>, part2: bool) -> Vec<Vec<char>> {
    let mut current_map = map.to_owned();
    loop {
        let (changed, new_map) = compute_cycle(&current_map, part2);
        current_map = new_map;
        if !changed {
            break;
        }
    }
    current_map
}

fn count_occupied(map: &Vec<Vec<char>>) -> usize {
    map.iter()
        .map(|l| l.iter().filter(|&&c| c == '#').count())
        .sum()
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &Vec<Vec<char>>) -> usize {
    let current_map = find_equilibrium(input, false);

    count_occupied(&current_map)
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &Vec<Vec<char>>) -> usize {
    let current_map = find_equilibrium(input, true);

    count_occupied(&current_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL";
        let input = parse_input(input);

        assert_eq!(37, solve_part1(&input));
    }

    #[test]
    fn part_2() {
        let input = "L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL";
        let input = parse_input(input);

        assert_eq!(26, solve_part2(&input));
    }
}
