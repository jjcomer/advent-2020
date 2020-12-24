use im::HashMap;
use im::Vector;
use itertools::Itertools;
use itertools::MinMaxResult::{MinMax, NoElements, OneElement};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Direction {
    NE,
    NW,
    SE,
    SW,
    E,
    W,
}

type Tiles = HashMap<(i32, i32), bool>;

#[aoc_generator(day24)]
pub fn parse_input(input: &str) -> Vector<Vector<Direction>> {
    input
        .lines()
        .map(|l| {
            let mut c = l.trim().chars().collect::<Vector<char>>();
            let mut d = Vector::new();
            loop {
                if let Some(ch) = c.pop_front() {
                    match ch {
                        'n' => match c.pop_front() {
                            Some('e') => d.push_back(Direction::NE),
                            Some('w') => d.push_back(Direction::NW),
                            x => panic!("Unknown direction {} {:?}", ch, x),
                        },
                        's' => match c.pop_front() {
                            Some('e') => d.push_back(Direction::SE),
                            Some('w') => d.push_back(Direction::SW),
                            x => panic!("Unknown direction {} {:?}", ch, x),
                        },
                        'e' => d.push_back(Direction::E),
                        'w' => d.push_back(Direction::W),
                        x => panic!("Unknown direction {:?}", x),
                    }
                } else {
                    break;
                }
            }
            d
        })
        .collect()
}

fn count_neighbours(tiles: &Tiles, (x, y): (i32, i32)) -> i32 {
    let to_check = if y % 2 == 0 {
        vec![
            (x, y + 1),
            (x - 1, y + 1),
            (x, y - 1),
            (x - 1, y - 1),
            (x - 1, y),
            (x + 1, y),
        ]
    } else {
        vec![
            (x + 1, y + 1),
            (x, y + 1),
            (x + 1, y - 1),
            (x, y - 1),
            (x - 1, y),
            (x + 1, y),
        ]
    };

    to_check
        .iter()
        .filter(|c| *tiles.get(c).unwrap_or(&false))
        .count() as i32
}

fn find_coord(directions: Vector<Direction>) -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;

    for d in directions {
        match d {
            Direction::NE => {
                if y % 2 == 0 {
                    y += 1
                } else {
                    x += 1;
                    y += 1;
                }
            }
            Direction::NW => {
                if y % 2 == 0 {
                    x -= 1;
                    y += 1;
                } else {
                    y += 1;
                }
            }
            Direction::SE => {
                if y % 2 == 0 {
                    y -= 1;
                } else {
                    x += 1;
                    y -= 1;
                }
            }
            Direction::SW => {
                if y % 2 == 0 {
                    x -= 1;
                    y -= 1;
                } else {
                    y -= 1;
                }
            }
            Direction::E => x += 1,
            Direction::W => x -= 1,
        };
    }

    (x, y)
}

fn flip_tiles(directions: Vector<Vector<Direction>>) -> Tiles {
    let mut tiles = HashMap::new();
    for direction in directions {
        let coords = find_coord(direction);
        if *tiles.get(&coords).unwrap_or(&false) {
            //println!("Flip {:?} to white", coords);
            tiles.insert(coords, false);
        } else {
            //println!("Flip {:?} to black", coords);
            tiles.insert(coords, true);
        }
    }
    tiles
}

fn find_bounds(tiles: &Tiles) -> (i32, i32, i32, i32) {
    let x = match tiles.keys().map(|(x, _)| *x).minmax() {
        MinMax(min, max) => (min, max),
        _ => panic!("No x range"),
    };
    let y = match tiles.keys().map(|(_, y)| *y).minmax() {
        MinMax(min, max) => (min, max),
        _ => panic!("No y range"),
    };
    (x.0 - 1, x.1 + 1, y.0 - 1, y.1 + 1)
}

fn perform_day(tiles: Tiles) -> Tiles {
    let (minx, maxx, miny, maxy) = find_bounds(&tiles);
    let mut new_tiles = tiles.clone();
    for x in minx..=maxx {
        for y in miny..=maxy {
            let neighbours = count_neighbours(&tiles, (x, y));
            if *tiles.get(&(x, y)).unwrap_or(&false) {
                //black
                if neighbours == 0 || neighbours > 2 {
                    new_tiles.insert((x, y), false);
                }
            } else {
                //white
                if neighbours == 2 {
                    new_tiles.insert((x, y), true);
                }
            }
        }
    }
    new_tiles
}

#[aoc(day24, part1)]
pub fn solve_part1(input: &Vector<Vector<Direction>>) -> usize {
    let tiles = flip_tiles(input.clone());
    tiles.iter().filter(|(_, black)| **black).count()
}

#[aoc(day24, part2)]
pub fn solve_part2(input: &Vector<Vector<Direction>>) -> usize {
    let mut tiles = flip_tiles(input.clone());

    for i in 0..100 {
        tiles = perform_day(tiles);
        // println!(
        //     "Day {}, Count {}",
        //     i,
        //     tiles.iter().filter(|(_, black)| **black).count()
        // );
    }
    tiles.iter().filter(|(_, black)| **black).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = parse_input(
            "sesenwnenenewseeswwswswwnenewsewsw
        neeenesenwnwwswnenewnwwsewnenwseswesw
        seswneswswsenwwnwse
        nwnwneseeswswnenewneswwnewseswneseene
        swweswneswnenwsewnwneneseenw
        eesenwseswswnenwswnwnwsewwnwsene
        sewnenenenesenwsewnenwwwse
        wenwwweseeeweswwwnwwe
        wsweesenenewnwwnwsenewsenwwsesesenwne
        neeswseenwwswnwswswnw
        nenwswwsewswnenenewsenwsenwnesesenew
        enewnwewneswsewnwswenweswnenwsenwsw
        sweneswneswneneenwnewenewwneswswnese
        swwesenesewenwneswnwwneseswwne
        enesenwswwswneneswsenwnewswseenwsese
        wnwnesenesenenwwnenwsewesewsesesew
        nenewswnwewswnenesenwnesewesw
        eneswnwswnwsenenwnwnwwseeswneewsenese
        neswnwewnwnwseenwseesewsenwsweewe
        wseweeenwnesenwwwswnew",
        );

        assert_eq!(10, solve_part1(&input));
    }

    #[test]
    fn part2() {
        let input = parse_input(
            "sesenwnenenewseeswwswswwnenewsewsw
        neeenesenwnwwswnenewnwwsewnenwseswesw
        seswneswswsenwwnwse
        nwnwneseeswswnenewneswwnewseswneseene
        swweswneswnenwsewnwneneseenw
        eesenwseswswnenwswnwnwsewwnwsene
        sewnenenenesenwsewnenwwwse
        wenwwweseeeweswwwnwwe
        wsweesenenewnwwnwsenewsenwwsesesenwne
        neeswseenwwswnwswswnw
        nenwswwsewswnenenewsenwsenwnesesenew
        enewnwewneswsewnwswenweswnenwsenwsw
        sweneswneswneneenwnewenewwneswswnese
        swwesenesewenwneswnwwneseswwne
        enesenwswwswneneswsenwnewswseenwsese
        wnwnesenesenenwwnenwsewesewsesesew
        nenewswnwewswnenesenwnesewesw
        eneswnwswnwsenenwnwnwwseeswneewsenese
        neswnwewnwnwseenwseesewsenwsweewe
        wseweeenwnesenwwwswnew",
        );

        assert_eq!(2208, solve_part2(&input));
    }
}
