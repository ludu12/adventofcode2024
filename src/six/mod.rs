use std::collections::HashSet;
use crate::utils::{get_bounds, grid, Direction, Position};

pub fn run() {
    let input = include_str!("input.txt");
    let part1 = process(input, false);
    println!("Part1: {}", part1.to_string());
    let part2 = process(input, true);
    println!("Part2: {}", part2.to_string());
}

fn get_start(g: &Vec<Vec<char>>) -> Position {
    for (y, line) in g.into_iter().enumerate() {
        for (x, char) in line.into_iter().enumerate() {
            match char {
                '^' => {
                    return Position {
                        x: x as i32,
                        y: y as i32,
                        dir: Direction::North,
                    }
                }
                _ => {}
            }
        }
    }
    panic!("Could not find starting point ^");
}

fn process(input: &str, _part2: bool) -> usize {
    let g = grid(input);
    let (x, y) = get_bounds(&g);
    let mut position = get_start(&g);

    let mut distinct = HashSet::new();
    while position.is_valid(x, y) {
        distinct.insert((position.x, position.y));
        let next = position.go(position.dir);
        if next.is_valid(x, y) && g[next.y as usize][next.x as usize] == '#' {
            position.dir = position.dir.turn(90);
        }
        else {
            position = next;
        }
    }

    distinct.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(41, process(input, false));
    }

    #[test]
    fn part2() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(0, process(input, true));
    }
}
