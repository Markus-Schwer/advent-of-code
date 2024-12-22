use std::fmt::Display;

const INPUT: &str = include_str!("../../resources/2024_12.txt");

const CORNER_VECTORS: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

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

fn print_grid(grid: &Vec<Vec<char>>) {
    for col in grid {
        for &c in col {
            print!("{}", c);
        }
        println!();
    }
}

fn find_region(grid: &Vec<Vec<char>>, rows: usize, columns: usize, start: (i32, i32)) -> Vec<(i32, i32)> {
    let plant = grid[start.1 as usize][start.0 as usize];

    let mut queue = Vec::new();
    let mut visited = Vec::new();
    visited.push(start);
    queue.push(start);

    while let Some(pos) = queue.pop() {
        for direction in DIRECTIONS {
            let next_pos = apply_direction_offset(pos, &direction);
            if !is_in_bounds(next_pos, rows, columns) {
                continue;
            }

            if grid[next_pos.1 as usize][next_pos.0 as usize] != plant {
                continue;
            }

            if !visited.contains(&next_pos) {
                visited.push(next_pos);
                queue.push(next_pos);
            }
        }
    }

    visited
}

fn main() {
    let columns = INPUT.find('\n').unwrap();
    let rows = INPUT.lines().count();

    let grid = INPUT.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let mut regions: Vec<Vec<(i32, i32)>> = Vec::new();
    for y in 0..rows {
        for x in 0..columns {
            let pos = (x as i32, y as i32);
            if regions.iter().any(|region| region.contains(&pos)) {
                continue;
            }

            regions.push(find_region(&grid, rows, columns, pos));
        }
    }

    let mut total_price = 0;
    let mut total_price_p2 = 0;
    for region in regions {
        let area = region.len();
        let mut perimeter = 0;
        let start = region[0];
        let plant = grid[start.1 as usize][start.0 as usize];

        let mut debug_grid = vec![vec!['.'; columns+2]; rows+2];

        let mut edges: Vec<((i32, i32), (i32, i32))> = Vec::new();
        for &pos in &region {
            debug_grid[pos.1 as usize + 1][pos.0 as usize + 1] = plant;
            for direction in DIRECTIONS {
                let next_pos = apply_direction_offset(pos, &direction);
                if !is_in_bounds(next_pos, rows, columns) || grid[next_pos.1 as usize][next_pos.0 as usize] != plant {
                    edges.push((pos, next_pos));
                    perimeter += 1;

                    let edge_char = match direction {
                        Direction::North | Direction::South=> '-',
                        Direction::East | Direction::West => '|',
                    };

                    debug_grid[next_pos.1 as usize + 1][next_pos.0 as usize + 1] = edge_char;
                }
            }
        }

        let mut outer_corners: Vec<(i32, i32)> = Vec::new();
        let mut inner_corners: Vec<(i32, i32)> = Vec::new();
        for e1 in &edges {
            for e2 in &edges {
                if e1 == e2 {
                    continue;
                }

                let (pos1, edge1) = e1;
                let (pos2, edge2) = e2;
                let v1 = (edge1.0 - pos1.0, edge1.1 - pos1.1);
                let v2 = (edge2.0 - pos2.0, edge2.1 - pos2.1);
                let v_corner = (v1.0 + v2.0, v1.1 + v2.1);
                let outer_corner = (pos1.0 + v_corner.0, pos1.1 + v_corner.1);
                let inner_v1 = (pos1.0 - edge1.0, pos1.1 - edge1.1);
                let inner_v2 = (pos2.0 - edge2.0, pos2.1 - edge2.1);
                let inner_v_corner = (inner_v1.0 + inner_v2.0, inner_v1.1 + inner_v2.1);
                let inner_corner = (edge1.0 + inner_v_corner.0, edge1.1 + inner_v_corner.1);

                if edge1 == edge2 && CORNER_VECTORS.contains(&inner_v_corner) {
                    // inner edge
                    inner_corners.push(inner_corner);

                    debug_grid[edge1.1 as usize + 1][edge1.0 as usize + 1] = '+';
                }

                if pos1 == pos2 && CORNER_VECTORS.contains(&v_corner) && !region.contains(&outer_corner) {
                    // outer edge
                    outer_corners.push(outer_corner);

                    debug_grid[outer_corner.1 as usize + 1][outer_corner.0 as usize + 1] = '+';
                }
            }
        }

        print_grid(&debug_grid);

        let price = area * perimeter;
        let sides = inner_corners.len() / 2 + outer_corners.len() / 2;
        let price_p2 = area * sides;
        total_price += price;
        total_price_p2 += price_p2;
        println!("{}: {}", plant, sides);
    }

    print_grid(&grid);

    println!("total_price: {}", total_price);
    println!("price part 2: {}", total_price_p2);
}
