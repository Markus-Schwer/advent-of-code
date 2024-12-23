use std::{collections::HashSet, fmt::Display};
use colored::Colorize;

const INPUT: &str = include_str!("../../resources/2024_16.txt");

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
            Direction::West => Direction::North,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
        }
    }

    fn rotate_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
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

fn apply_direction_offset(position: (i32, i32), direction: &Direction) -> (i32, i32) {
    let offset = direction.get_offset();
    (position.0 + offset.0, position.1 + offset.1)
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for col in grid {
        for &c in col {
            if vec!['<','^','>','v','O'].contains(&c) {
                print!("{}", c.to_string().red());
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
}

#[derive(Debug, Clone)]
struct Node {
    g: i32,
    pos: (i32, i32),
    direction: Direction,
    parent: Option<Box<Node>>,
}

impl Node {
    fn new(pos: (i32, i32), direction: Direction, parent: Option<Box<Node>>, g: i32) -> Self {
        Node { pos, direction, parent, g }
    }
}

fn djikstra(grid: &Vec<Vec<char>>, start: Node, end: &(i32, i32)) -> Vec<Node> {
    let mut open = vec![start];
    let mut visited: HashSet<((i32, i32), Direction)> = HashSet::new();
    let mut result = Vec::new();

    const MOVEMENT_COST: i32 = 1;
    const TURN_COST: i32 = 1000;

    while !open.is_empty() {
        open.sort_by(|a, b| a.g.cmp(&b.g));
        let node = open.remove(0);

        if node.pos == *end && !result.iter().any(|n: &Node| n.g < node.g) {
            result.push(node.clone());
        }

        //if visited.contains(&(node.pos, node.direction.clone())) {
        //    continue;
        //}

        visited.insert((node.pos, node.direction.clone()));

        let forward = Node::new(apply_direction_offset(node.pos, &node.direction), node.direction.clone(), Some(Box::new(node.clone())), node.g + MOVEMENT_COST);
        if grid[forward.pos.1 as usize][forward.pos.0 as usize] != '#' && !visited.contains(&(forward.pos, forward.direction.clone())) {
            open.push(forward);
        }

        let left = Node::new(node.pos, node.direction.rotate_left(), Some(Box::new(node.clone())), node.g + TURN_COST);
        if !visited.contains(&(left.pos, left.direction.clone())) {
            open.push(left);
        }

        let right = Node::new(node.pos, node.direction.rotate_right(), Some(Box::new(node.clone())), node.g + TURN_COST);
        if !visited.contains(&(right.pos, right.direction.clone())) {
            open.push(right);
        }
    }

    return result;
}

fn main() {
    let rows = INPUT.lines().count();
    let cols = INPUT.lines().next().unwrap().len();

    let mut grid = vec![vec!['.'; cols]; rows];
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, line) in INPUT.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (x as i32, y as i32);
            } else if c == 'E' {
                end = (x as i32, y as i32);
            }

            grid[y][x] = c;
        }
    }

    let start_node = Node::new(start, Direction::East, None, 0);
    let solutions = djikstra(&grid, start_node, &end);

    // part 1
    let mut grid_p1 = grid.clone();
    let solution = solutions.get(0).expect("could not find solution");
    let mut parent = &solution.parent;
    while let Some(node) = parent {
        if node.pos == start {
            break;
        }

        let (x, y) = node.pos;
        grid_p1[y as usize][x as usize] = node.direction.to_string().chars().next().unwrap();
        parent = &node.parent;
    }

    print_grid(&grid_p1);
    println!("score {}", solution.g);

    // part 2
    let mut grid_p2 = grid.clone();
    for solution in &solutions {
        let mut parent = &Some(Box::new(solution.clone()));
        while let Some(node) = parent {
            let (x, y) = node.pos;
            grid_p2[y as usize][x as usize] = 'O';
            parent = &node.parent;
        }
    }

    print_grid(&grid_p2);
    let count = grid_p2.iter().flatten().filter(|&&c| c == 'O').count();
    println!("Tiles part of best paths: {}", count);
}
