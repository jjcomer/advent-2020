use std::collections::HashMap;

use itertools::iproduct;

type Coord = (i64, i64, i64);
type Coord2 = (i64, i64, i64, i64);
type Grid = HashMap<Coord, char>;
type Grid2 = HashMap<Coord2, char>;

#[aoc_generator(day17)]
pub fn parse_input(input: &str) -> Grid {
    let mut grid = HashMap::new();
    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            grid.insert((x as i64, y as i64, 0), c);
        }
    }

    grid
}

fn new_char(current_state: char, active_neighbours: i64) -> char {
    match current_state {
        '#' => {
            if active_neighbours == 2 || active_neighbours == 3 {
                '#'
            } else {
                '.'
            }
        }
        '.' => {
            if active_neighbours == 3 {
                '#'
            } else {
                '.'
            }
        }
        _ => panic!("Unknown character {}", current_state),
    }
}

fn check_neighbours(grid: &Grid, coord: &Coord) -> char {
    let current_state = grid.get(coord).unwrap_or(&'.');
    let mut active_neighbours = 0;
    let (x, y, z) = *coord;
    for to_check in iproduct!(
        x - 1..=x + 1 as i64,
        y - 1..=y + 1 as i64,
        z - 1..=z + 1 as i64
    ) {
        if to_check == *coord {
            continue;
        }
        if *grid.get(&to_check).unwrap_or(&'.') == '#' {
            active_neighbours += 1;
        }
    }

    new_char(*current_state, active_neighbours)
}

fn check_neighbours2(grid: &Grid2, coord: &Coord2) -> char {
    let current_state = grid.get(coord).unwrap_or(&'.');
    let mut active_neighbours = 0;
    let (x, y, z, w) = *coord;
    for to_check in iproduct!(
        x - 1..=x + 1 as i64,
        y - 1..=y + 1 as i64,
        z - 1..=z + 1 as i64,
        w - 1..=w + 1 as i64
    ) {
        if to_check == *coord {
            continue;
        }
        if *grid.get(&to_check).unwrap_or(&'.') == '#' {
            active_neighbours += 1;
        }
    }

    new_char(*current_state, active_neighbours)
}

fn find_search_space(grid: &Grid) -> ((i64, i64), (i64, i64), (i64, i64)) {
    (
        (
            *(grid.keys().map(|(x, _, _)| x).min().unwrap()) - 1,
            *(grid.keys().map(|(x, _, _)| x).max().unwrap()) + 1,
        ),
        (
            *(grid.keys().map(|(_, y, _)| y).min().unwrap()) - 1,
            *(grid.keys().map(|(_, y, _)| y).max().unwrap()) + 1,
        ),
        (
            *(grid.keys().map(|(_, _, z)| z).min().unwrap()) - 1,
            *(grid.keys().map(|(_, _, z)| z).max().unwrap()) + 1,
        ),
    )
}

fn find_search_space2(grid: &Grid2) -> ((i64, i64), (i64, i64), (i64, i64), (i64, i64)) {
    (
        (
            *(grid.keys().map(|(x, _, _, _)| x).min().unwrap()) - 1,
            *(grid.keys().map(|(x, _, _, _)| x).max().unwrap()) + 1,
        ),
        (
            *(grid.keys().map(|(_, y, _, _)| y).min().unwrap()) - 1,
            *(grid.keys().map(|(_, y, _, _)| y).max().unwrap()) + 1,
        ),
        (
            *(grid.keys().map(|(_, _, z, _)| z).min().unwrap()) - 1,
            *(grid.keys().map(|(_, _, z, _)| z).max().unwrap()) + 1,
        ),
        (
            *(grid.keys().map(|(_, _, _, w)| w).min().unwrap()) - 1,
            *(grid.keys().map(|(_, _, _, w)| w).max().unwrap()) + 1,
        ),
    )
}

fn compute_cycle(grid: &Grid) -> Grid {
    let mut new_grid = HashMap::new();
    let ((x_min, x_max), (y_min, y_max), (z_min, z_max)) = find_search_space(grid);
    for coord in iproduct!(x_min..=x_max, y_min..=y_max, z_min..=z_max) {
        new_grid.insert(coord, check_neighbours(grid, &coord));
    }
    new_grid
}

fn compute_cycle2(grid: &Grid2) -> Grid2 {
    let mut new_grid = HashMap::new();
    let ((x_min, x_max), (y_min, y_max), (z_min, z_max), (w_min, w_max)) = find_search_space2(grid);
    for coord in iproduct!(x_min..=x_max, y_min..=y_max, z_min..=z_max, w_min..=w_max) {
        new_grid.insert(coord, check_neighbours2(grid, &coord));
    }
    new_grid
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &Grid) -> usize {
    let mut the_grid = input.to_owned();
    for _ in 0..6 {
        the_grid = compute_cycle(&the_grid);
    }

    the_grid.values().filter(|&c| *c == '#').count()
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &Grid) -> usize {
    let mut the_grid = HashMap::new();
    for (&(x, y, z), &v) in input {
        the_grid.insert((x, y, z, 0), v);
    }
    for _ in 0..6 {
        the_grid = compute_cycle2(&the_grid);
    }

    the_grid.values().filter(|&c| *c == '#').count()
}
