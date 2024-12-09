use std::collections::HashMap;

use itertools::Itertools;

const INPUT: &str = include_str!("../../resources/2024_08.txt");

fn print_grid(grid: &Vec<Vec<char>>) {
    for col in grid {
        for &c in col {
            print!("{}", c);
        }
        println!();
    }
}

fn main() {
    let rows = INPUT.lines().count();
    let columns = INPUT.lines().next().unwrap().len();

    let mut grid = vec![vec!['.'; columns]; rows];
    let mut frequencies: HashMap<char, Vec<(i32, i32)>> = HashMap::<char, Vec<(i32, i32)>>::new();

    for (x, line) in INPUT.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            grid[x][y] = c;

            if c >= '0' && c <= '9' || c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' {
                let antennas = frequencies.entry(c).or_insert(Vec::new());
                antennas.push((x as i32, y as i32));
            }
        }
    }

    let mut antinodes: Vec<(i32, i32)> = Vec::new();
    for (frequency, antennas) in frequencies.iter() {
        for (first, second) in antennas.iter().tuple_combinations() {
            let (x0, y0) = (first.0 as f32, first.1 as f32);
            let (x1, y1) = (second.0 as f32, second.1 as f32);

            let pair_distance = f32::sqrt((x1 - x0).powi(2) + (y1 - y0).powi(2) as f32);
            let (ux, uy) = ((x1 - x0) / pair_distance, (y1 - y0) / pair_distance);

            let first_antinode = ((x0 - pair_distance * ux).round_ties_even() as i32, (y0 - pair_distance * uy).round_ties_even() as i32);
            let second_antinode = ((x1 + pair_distance * ux).round_ties_even() as i32, (y1 + pair_distance * uy).round_ties_even() as i32);

            if first_antinode.0 >= 0 && first_antinode.0 < columns as i32 && first_antinode.1 >= 0 && first_antinode.1 < rows as i32 {
                antinodes.push(first_antinode);
                if grid[first_antinode.0 as usize][first_antinode.1 as usize] == '.' {
                    grid[first_antinode.0 as usize][first_antinode.1 as usize] = '#';
                }
            }

            if second_antinode.0 >= 0 && second_antinode.0 < columns as i32 && second_antinode.1 >= 0 && second_antinode.1 < rows as i32 {
                antinodes.push(second_antinode);
                if grid[second_antinode.0 as usize][second_antinode.1 as usize] == '.' {
                    grid[second_antinode.0 as usize][second_antinode.1 as usize] = '#';
                }
            }
        }
    }

    antinodes.sort();
    antinodes.dedup();
    let sum = antinodes.len();

    print_grid(&grid);
    println!("sum: {}", sum);
}
