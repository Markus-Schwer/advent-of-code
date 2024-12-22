use std::process::exit;

const INPUT: &str = include_str!("../../resources/2024_14.txt");

fn print_grid(grid: &Vec<Vec<i32>>) {
    for col in grid {
        for &c in col {
            if c == 0 {
                print!(".");
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
}

fn main() {
    let re = regex::Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

    const ROWS: usize = 103;
    const COLUMNS: usize = 101;
    let mut grid = vec![vec![0; COLUMNS]; ROWS];
    let mut robots = Vec::new();

    for (_, [px, py, vx, vy]) in re.captures_iter(INPUT).map(|c| c.extract()) {
        let px: i32 = px.parse().unwrap();
        let py: i32 = py.parse().unwrap();
        let vx: i32 = vx.parse().unwrap();
        let vy: i32 = vy.parse().unwrap();

        robots.push(((px, py), (vx, vy)));
    }

    for iteration in 0..1000000 {
        for i in 0..robots.len() {
            let (pos, v) = robots[i];
            let mut x = pos.0 + v.0;
            let mut y = pos.1 + v.1;

            if x < 0 {
                x = COLUMNS as i32 + x;
            }
            if y < 0 {
                y = ROWS as i32 + y;
            }
            if x >= COLUMNS as i32 {
                x = x - COLUMNS as i32;
            }
            if y >= ROWS as i32 {
                y = y - ROWS as i32;
            }

            robots[i] = ((x, y), v);
        }

        grid = vec![vec![0; COLUMNS]; ROWS];
        for &((x, y), _) in &robots {
            grid[y as usize][x as usize] += 1;
        }


        let mut count = 0;
        for x in 0..COLUMNS {
            let mut sum = 0;
            for y in 0..ROWS {
                if grid[y][x] == 0 {
                    sum = 0;
                } else {
                    sum += 1;
                    if sum > 10 {
                        count += 1;
                    }
                }
            }
        }

        if count > 0 {
            println!("{}", iteration);
            print_grid(&grid);
            break;
        }
    }

    let mut quadrants = vec![0; 4];

    for y in 0..ROWS {
        for x in 0..COLUMNS {
            if x == COLUMNS/2 || y == ROWS/2 {
                continue;
            }
            let quadrant_row = y/((ROWS/2)+1);
            let quadrant_col = x/((COLUMNS/2)+1);
            let quadrant_index = 2 * quadrant_row + quadrant_col;
            quadrants[quadrant_index] += grid[y][x];
        }
    }

    let mut safety_factor = 1;
    for &quadrant in &quadrants {
        safety_factor *= quadrant;
    }

    println!("{:?}", quadrants);
    println!("sum: {}", safety_factor);
}
