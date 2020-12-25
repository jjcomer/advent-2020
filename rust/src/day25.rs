#[aoc_generator(day25)]
pub fn parse_input(input: &str) -> (i64, i64) {
    let mut l = input.trim().lines();
    (
        l.next().unwrap().parse().unwrap(),
        l.next().unwrap().parse().unwrap(),
    )
}

fn find_loop_size(key: i64) -> i64 {
    let mut loop_counter = 0;
    let mut value = 1;

    loop {
        value = (value * 7) % 20201227;
        loop_counter += 1;
        if value == key {
            return loop_counter;
        }
    }
}

fn find_private_key(key: i64, loop_size: i64) -> i64 {
    let mut value = 1;
    for _ in 0..loop_size {
        value = (value * key) % 20201227;
    }
    value
}

#[aoc(day25, part1)]
fn solve_part1(input: &(i64, i64)) -> i64 {
    let (door, card) = *input;
    let door_loop_size = find_loop_size(door);
    find_private_key(card, door_loop_size)
}
