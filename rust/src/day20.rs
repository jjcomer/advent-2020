use std::{error::Error, str::FromStr};

use apply::Apply;
use im::hashset;
use im::HashMap;
use im::HashSet;
use im::Vector;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub struct Tile {
    num: usize,
    above: Option<usize>,
    below: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    raw: Vector<Vector<char>>,
}

impl Tile {
    fn get_bottom(&self) -> Vector<char> {
        self.raw.last().unwrap().clone()
    }

    fn get_top(&self) -> Vector<char> {
        self.raw.head().unwrap().clone()
    }

    fn get_left(&self) -> Vector<char> {
        self.raw
            .iter()
            .map(|l| l.head().unwrap())
            .cloned()
            .collect()
    }

    fn get_right(&self) -> Vector<char> {
        self.raw
            .iter()
            .map(|l| l.last().unwrap())
            .cloned()
            .collect()
    }

    fn all_possible_sides(&self) -> HashSet<Vector<char>> {
        let top = self.get_top();
        let bottom = self.get_bottom();
        let left = self.get_left();
        let right = self.get_right();

        hashset![
            top.iter().cloned().rev().collect(),
            top,
            bottom.iter().cloned().rev().collect(),
            bottom,
            left.iter().cloned().rev().collect(),
            left,
            right.iter().cloned().rev().collect(),
            right,
        ]
    }

    fn flip_y(&self) -> Self {
        let mut copy = self.clone();
        copy.raw = copy.raw.into_iter().rev().collect();
        copy
    }

    fn flip_x(&self) -> Self {
        let mut copy = self.clone();
        copy.raw = copy
            .raw
            .into_iter()
            .map(|l| l.into_iter().rev().collect())
            .collect();
        copy
    }

    fn rotate(&self) -> Self {
        let mut copy = self.clone();
        copy.raw = (0..copy.raw.len())
            .map(|i| copy.raw.iter().map(|l| *l.get(i).unwrap()).collect())
            .collect();
        copy
    }

    fn num_neighbours(&self) -> usize {
        let mut count = 0;
        if self.above.is_some() {
            count += 1
        };
        if self.below.is_some() {
            count += 1
        };
        if self.left.is_some() {
            count += 1
        };
        if self.right.is_some() {
            count += 1
        };
        count
    }

    fn common_side(&self, other: &Tile) -> Option<Vector<char>> {
        let common_sides = self
            .all_possible_sides()
            .intersection(other.all_possible_sides());
        match common_sides.len() {
            0 => None,
            1 => Some(common_sides.iter().next().unwrap().clone()),
            //Ignore this for now
            n => {
                println!("Matched a bunch of sides {} {} {}", self.num, other.num, n);
                Some(common_sides.iter().next().unwrap().clone())
            }
        }
    }
}

// Tile 2729:
// ###.######
// .......#.#
// #..#......
// ....#.#...
// ...#.....#
// .....#.###
// ...#.....#
// ........#.
// ..........
// #.......##

impl FromStr for Tile {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.lines();
        let num = split.next().ok_or("")?.trim()[5..9].parse()?;
        let raw: Vector<Vector<char>> = split.map(|l| l.trim().chars().collect()).collect();
        Ok(Tile {
            num,
            raw,
            ..Default::default()
        })
    }
}

#[aoc_generator(day20)]
pub fn parse_input(input: &str) -> HashMap<usize, Tile> {
    input
        .split("\n\n")
        .map(|l| {
            let t: Tile = l.parse().unwrap();
            (t.num, t)
        })
        .collect()
}

fn build_map(tiles: HashMap<usize, Tile>) -> HashMap<usize, Tile> {
    let first_tile = tiles.values().next().unwrap();
    let mut remaining_tiles = tiles
        .keys()
        .cloned()
        .collect::<HashSet<usize>>()
        .without(&first_tile.num);
    let mut exposed_sides = HashMap::new();
    exposed_sides.insert(first_tile.get_bottom(), first_tile.num);
    exposed_sides.insert(first_tile.get_top(), first_tile.num);
    exposed_sides.insert(first_tile.get_left(), first_tile.num);
    exposed_sides.insert(first_tile.get_right(), first_tile.num);

    let mut the_map = tiles.clone();

    while !remaining_tiles.is_empty() {}
    the_map
}

fn find_corners(tiles: HashMap<usize, Tile>) -> Vector<Tile> {
    let raw_tiles = tiles.values().cloned().collect::<HashSet<Tile>>();
    raw_tiles
        .iter()
        .filter(|&t| {
            raw_tiles
                .without(t)
                .iter()
                .filter_map(|other_tile| t.common_side(other_tile))
                .count()
                == 2
        })
        .cloned()
        .collect()
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &HashMap<usize, Tile>) -> usize {
    let corners = input.clone().apply(find_corners);
    println!("Found {} corners", corners.len());
    corners.iter().map(|t| t.num).product()
}
