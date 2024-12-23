use core::panic;
use std::{collections::HashMap, fmt::Display};

use colored::Colorize;

const INPUT: &str = include_str!("../../resources/2024_15.txt");

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

    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::West => Direction::East,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
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

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Direction::North,
            '>' => Direction::East,
            'v' => Direction::South,
            '<' => Direction::West,
            _ => panic!("Invalid direction {}", value),
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

fn print_grid(grid: &Vec<Vec<char>>) {
    for col in grid {
        for &c in col {
            if vec!['<','^','>','v'].contains(&c) {
                print!("{}", c.to_string().red());
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
}

fn get_movable_obstacle_block(grid: &Vec<Vec<char>>, rows: usize, columns: usize, direction: &Direction, pos: (i32, i32), visited: &HashMap<(i32, i32), char>) -> Option<HashMap<(i32, i32), char>> {
    let mut block: HashMap<(i32, i32), char> = HashMap::new();
    let mut curr = pos;
    loop {
        if !is_in_bounds(curr, rows, columns) {
            return None;
        }

        let obstacle = grid[curr.1 as usize][curr.0 as usize];
        if obstacle == '#' {
            // if we hit a wall, we can't move
            return None;
        }

        if obstacle == '.' {
            // if there is a free space, the block ends
            break;
        }

        if obstacle == 'O' {
            block.insert(curr, obstacle);
            curr = apply_direction_offset(curr, &direction);
        } else if obstacle == '[' || obstacle == ']' {
            // part 2
            if direction == &Direction::West || direction == &Direction::East {
                // when we want to move a box horizontally, we can just proceed as normal
                block.insert(curr, obstacle);
                curr = apply_direction_offset(curr, &direction);
                continue;
            }

            block.insert(curr, obstacle);

            // when we want to move a box vertically, we need to check if the other part of the box
            // is also free to move and if it also pushes other boxes
            let other_box_part = if obstacle == '[' { (curr.0+1, curr.1) } else { (curr.0-1, curr.1) };
            if visited.contains_key(&other_box_part) {
                curr = apply_direction_offset(curr, &direction);
                continue;
            }

            let Some(affected_block) = get_movable_obstacle_block(grid, rows, columns, direction, other_box_part, &block) else {
                return None;
            };

            block.extend(affected_block);
            curr = apply_direction_offset(curr, &direction);
        } else {
            panic!("unknown obstacle {}", obstacle);
        }
    }

    return Some(block);
}

fn walk_robot(grid: &mut Vec<Vec<char>>, rows: usize, cols: usize, instructions: &str, robot: &mut (i32, i32)) -> usize {
    for instruction in instructions.chars().filter(|&c| c != '\n') {
        let direction = Direction::from(instruction);
        let (x, y) = apply_direction_offset(*robot, &direction);
        let Some(block) = get_movable_obstacle_block(&grid, rows, cols, &direction, (x, y), &HashMap::new()) else {
            continue;
        };

        for (&obstacle, &c) in block.iter() {
            let new_obstacle = apply_direction_offset(obstacle, &direction);
            if let Some(&other_obstacle) = block.get(&apply_direction_offset(obstacle, &direction.opposite())) {
                grid[obstacle.1 as usize][obstacle.0 as usize] = other_obstacle;
            } else {
                grid[obstacle.1 as usize][obstacle.0 as usize] = '.';
            }
            grid[new_obstacle.1 as usize][new_obstacle.0 as usize] = c;
        }

        // move robot
        grid[robot.1 as usize][robot.0 as usize] = '.';
        grid[y as usize][x as usize] = '@';
        *robot = (x, y);
    }

    let mut sum = 0;
    for y in 0..rows {
        for x in 0..cols {
            if grid[y][x] == 'O' || grid[y][x] == '[' {
                sum += 100 * y + x;
            }
        }
    }

    return sum;
}

fn main() {
    let parts: Vec<&str> = INPUT.splitn(2, "\n\n").collect();
    let fields = parts[0];
    let instructions = parts[1];

    let mut robot: (i32, i32) = (0, 0);
    let mut resized_robot: (i32, i32) = (0, 0);

    let rows = fields.lines().count();
    let cols = fields.lines().next().unwrap().len();
    let mut grid = vec![vec!['.'; cols]; rows];

    let resized_rows = rows;
    let resized_cols = cols * 2;
    let mut resized_grid = vec![vec!['.'; resized_cols]; rows];

    for (y, line) in fields.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let resized_x = x * 2;
            let resized_y = y;
            match c {
                '@' => {
                    resized_robot = (resized_x as i32, resized_y as i32);
                    resized_grid[resized_y][resized_x] = '@';
                    resized_grid[resized_y][resized_x+1] = '.';
                },
                'O' => {
                    resized_grid[resized_y][resized_x] = '[';
                    resized_grid[resized_y][resized_x+1] = ']';
                },
                '#' | '.' => {
                    resized_grid[resized_y][resized_x] = c;
                    resized_grid[resized_y][resized_x+1] = c;
                },
                _ => panic!("Invalid char {}", c),
            }

            if c == '@' {
                robot = (x as i32, y as i32);
            }

            grid[y][x] = c;
        }
    }

    // part 1
    let part1 = walk_robot(&mut grid, rows, cols, instructions, &mut robot);
    print_grid(&grid);
    println!("Sum: {}", part1);

    // part 2
    let part2 = walk_robot(&mut resized_grid, resized_rows, resized_cols, instructions, &mut resized_robot);
    print_grid(&resized_grid);
    println!("Part 2: {}", part2);
}
