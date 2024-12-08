use std::{fmt::Display, i32};
use colored::Colorize;

const INPUT: &str = include_str!("../../resources/2024_06.txt");

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn rotate_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

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

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Direction::North),
            '>' => Ok(Direction::East),
            'v' => Ok(Direction::South),
            '<' => Ok(Direction::West),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Guard {
    position: (i32, i32),
    direction: Direction,
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for &c in row {
            if c == 'O' {
                print!("{}", c.to_string().red());
            } else if Direction::try_from(c).is_ok() {
                print!("{}", c.to_string().green());
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
}

fn apply_direction_offset(position: (i32, i32), direction: &Direction) -> (i32, i32) {
    let offset = direction.get_offset();
    (position.0 + offset.0, position.1 + offset.1)
}

fn walk(grid: &mut Vec<Vec<char>>, rows: i32, columns: i32, guard: Guard) {
    let mut guard = guard.clone();
    loop {
        let (x, y) = guard.position;

        let (nx, ny) = apply_direction_offset(guard.position, &guard.direction);
        if nx < 0 || nx >= columns || ny < 0 || ny >= rows {
            grid[y as usize][x as usize] = 'X';
            // path leads outside of the grid, so no loop
            break;
        }

        // obstacle in front
        if grid[ny as usize][nx as usize] == '#' {
            guard.direction = guard.direction.rotate_right();
        } else {
            grid[y as usize][x as usize] = 'X';
            guard.position = (nx, ny);
        }
    }
}


fn is_loop(grid: &mut Vec<Vec<char>>, rows: i32, columns: i32, intial_guard: Guard, obstacle: (i32, i32)) -> bool {
    let mut guard = intial_guard.clone();
    let mut path = Vec::new();

    loop {
        let (nx, ny) = apply_direction_offset(guard.position, &guard.direction);
        if nx < 0 || nx >= columns || ny < 0 || ny >= rows {
            // path leads outside of the grid, so no loop
            return false;
        }

        // obstacle in front
        if grid[ny as usize][nx as usize] == '#' || (nx, ny) == obstacle {
            if path.contains(&guard) {
                return true;
            }

            path.push(guard.clone());

            guard.direction = guard.direction.rotate_right();
        } else {
            guard.position = (nx, ny);
        }
    }
}

fn count_loops(grid: &mut Vec<Vec<char>>, rows: i32, columns: i32, guard: &Guard) -> u32 {
    let mut count = 0;
    let mut guard = guard.clone();
    let mut obstacles = Vec::new();
    loop {
        let (nx, ny) = apply_direction_offset(guard.position, &guard.direction);
        if nx < 0 || nx >= columns || ny < 0 || ny >= rows {
            break;
        }

        // obstacle in front
        if grid[ny as usize][nx as usize] == '#' {
            guard.direction = guard.direction.rotate_right();
        } else {
            if !obstacles.contains(&(nx, ny)) && is_loop(grid, rows, columns, guard.clone(), (nx, ny)) {
                count += 1;
                obstacles.push((nx, ny));
                grid[ny as usize][nx as usize] = 'O';
                println!("{}", count);
            }

            guard.position = (nx, ny);
        }
    }
    count
}

fn main() {
    let columns = INPUT.find('\n').unwrap();
    let rows = INPUT.lines().count();

    let mut grid = vec![vec!['.'; columns]; rows];
    let mut guard = Guard { position: (0, 0), direction: Direction::North };

    for (y, line) in INPUT.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[y][x] = c;

            if let Ok(direction) = Direction::try_from(c) {
                guard.position = (x as i32, y as i32);
                guard.direction = direction;
            }
        }
    }

    // part 1
    let mut grid_p1 = grid.clone();
    walk(&mut grid_p1, rows as i32, columns as i32, guard.clone());
    let count = grid_p1.iter().flatten().filter(|&&c| c == 'X').count();

    print_grid(&grid_p1);
    println!("distinct positions: {}", count);

    // part 2
    let mut grid_p2 = grid.clone();
    let loop_count = count_loops(&mut grid_p2, rows as i32, columns as i32, &guard);
    let loop_count_deduped = grid_p2.iter().flatten().filter(|&&c| c == 'O').count();

    print_grid(&grid_p2);
    println!("loop count: {}", loop_count);
    println!("loop count deduplicated: {}", loop_count_deduped);

}
