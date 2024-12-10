use std::{collections::HashSet, fmt::Display};

const INPUT: &str = include_str!("../../resources/2024_10.txt");

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn get_offset(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::North => write!(f, "^"),
            Direction::East => write!(f, ">"),
            Direction::South => write!(f, "v"),
            Direction::West => write!(f, "<"),
        }
    }
}

fn is_in_bounds((x, y): (i32, i32), rows: usize, columns: usize) -> bool {
    x >= 0 && y >= 0 && x < columns as i32 && y < rows as i32
}

fn apply_direction_offset(position: (i32, i32), direction: &Direction) -> (i32, i32) {
    let offset = direction.get_offset();
    (position.0 + offset.0, position.1 + offset.1)
}

fn count_trails(pos: (i32, i32), coming_from: (i32, i32), grid: &Vec<Vec<char>>, rows: usize, columns: usize) -> i32 {
    let current_c = grid[pos.0 as usize][pos.1 as usize];
    let mut res = 0;

    for direction in DIRECTIONS {
        let new_pos = apply_direction_offset(pos, &direction);
        if new_pos == coming_from || !is_in_bounds(new_pos, rows, columns) {
            continue;
        }

        let next_c = grid[new_pos.0 as usize][new_pos.1 as usize];
        let is_step_possible = next_c == (current_c as u8 + 1) as char;
        if is_step_possible && next_c == '9' {
            res += 1;
        } else if is_step_possible {
            res += count_trails(new_pos, pos, grid, rows, columns);
        }
    }
    res
}

fn get_reachable_ends(pos: (i32, i32), coming_from: (i32, i32), grid: &Vec<Vec<char>>, rows: usize, columns: usize, ends: &mut HashSet<(i32, i32)>) {
    let current_c = grid[pos.0 as usize][pos.1 as usize];

    for direction in DIRECTIONS {
        let new_pos = apply_direction_offset(pos, &direction);
        if new_pos == coming_from || !is_in_bounds(new_pos, rows, columns) {
            continue;
        }

        let next_c = grid[new_pos.0 as usize][new_pos.1 as usize];
        let is_step_possible = next_c == (current_c as u8 + 1) as char;
        if is_step_possible && next_c == '9' {
            ends.insert(new_pos);
        } else if is_step_possible {
            get_reachable_ends(new_pos, pos, grid, rows, columns, ends);
        }
    }
}

fn main() {
    let columns = INPUT.find('\n').unwrap();
    let rows = INPUT.lines().count();

    let mut grid = vec![vec!['.'; rows]; columns];
    let mut trailheads = Vec::new();

    for (y, line) in INPUT.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[x][y] = c;

            if c == '0' {
                // trailhead
                trailheads.push((x as i32, y as i32));
            }
        }
    }

    // part 1
    let mut count = 0;
    for &trailhead in &trailheads {
        let mut ends = HashSet::new();
        get_reachable_ends(trailhead, trailhead, &grid, rows, columns, &mut ends);
        count += ends.len();
    }

    println!("count: {}", count);

    // part 2
    let mut distinct_trails = 0;
    for &trailhead in &trailheads {
        distinct_trails += count_trails(trailhead, trailhead, &grid, rows, columns);
    }

    println!("distinct trails: {}", distinct_trails);
}
