use itertools::Itertools;

#[allow(dead_code)]
pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

pub fn grid(input: &str) -> Vec<Vec<char>> {
    return input.lines().map(|l| l.chars().collect_vec()).collect_vec();
}

#[allow(dead_code)]
pub fn get_neighbor(x: usize, y: usize, dx: isize, dy: isize, width: usize, height: usize) -> Option<(usize, usize)> {
    let nx = x.wrapping_add_signed(dx);
    let ny = y.wrapping_add_signed(dy);

    if nx < width && ny < height {
        Some((nx, ny))
    } else {
        None
    }
}

#[allow(dead_code)]
pub fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        println!("{}", row.iter().collect::<String>())
    }
}

#[allow(dead_code)]
pub fn lcm(first: i64, second: i64) -> i64 {
    first * second / gcd(first, second)
}

#[allow(dead_code)]
pub fn gcd(first: i64, second: i64) -> i64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    #[allow(dead_code)]
    pub fn is_valid(&self, max_x: usize, max_y: usize) -> bool {
        let p = self;
        p.x >= 0 && p.y >= 0 && p.x < max_x as i32 && p.y < max_y as i32
    }

    #[allow(dead_code)]
    pub fn up(&self) -> Position {
        Position { x: self.x, y: self.y - 1 }
    }
    #[allow(dead_code)]
    pub fn down(&self) -> Position {
        Position { x: self.x, y: self.y + 1 }
    }
    #[allow(dead_code)]
    pub fn left(&self) -> Position {
        Position { x: self.x - 1, y: self.y }
    }
    #[allow(dead_code)]
    pub fn right(&self) -> Position {
        Position { x: self.x + 1, y: self.y }
    }

    #[allow(dead_code)]
    pub fn go(&self, d: Direction) -> Position {
        match d {
            Direction::North => self.up(),
            Direction::East => self.right(),
            Direction::South => self.down(),
            Direction::West => self.left()
        }
    }
}
