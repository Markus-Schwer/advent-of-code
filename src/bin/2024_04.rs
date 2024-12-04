use regex::Regex;

const INPUT: &str = include_str!("../../resources/2024_04.txt");

fn print_field(rows: usize, columns: usize, field: Vec<char>) {
    for y in 0..rows {
        for x in 0..columns {
            print!("{}", field[(columns*y)+x]);
        }
        println!();
    }
}

fn part_one() {
    let re = Regex::new(r"XMAS").unwrap();

    let columns = INPUT.find('\n').unwrap();
    let rows = INPUT.lines().count();

    let chars = INPUT.chars().filter(|&c| c != '\n').collect::<Vec<char>>();
    let mut output = vec!['.'; columns * rows];

    let left_to_right_mapping: Vec<Vec<usize>> = (0..rows).map(|row| {
        let row_offset = row * columns;
        (row_offset..row_offset+columns).collect()
    }).collect();

    let right_to_left_mapping: Vec<Vec<usize>> = (0..rows).map(|row| {
        let row_offset = row * columns;
        (row_offset..row_offset+columns).rev().collect()
    }).collect();

    let top_to_bottom_mapping: Vec<Vec<usize>> = (0..columns).map(|col| {
        (0..rows).map(|row| row * columns + col).collect()
    }).collect();

    let bottom_to_top_mapping: Vec<Vec<usize>> = (0..columns).map(|col| {
        (0..rows).map(|row| row * columns + col).rev().collect()
    }).collect();

    let mut top_left_to_bottom_right: Vec<Vec<usize>> = Vec::new();
    let mut bottom_right_to_top_left: Vec<Vec<usize>> = Vec::new();
    let mut top_right_to_bottom_left: Vec<Vec<usize>> = Vec::new();
    let mut bottom_left_to_top_right: Vec<Vec<usize>> = Vec::new();

    for diagonal_index in 1..rows+columns+1 {
        let mut top_left_to_bottom_right_diagonal = Vec::new();
        let mut top_right_to_bottom_left_diagonal = Vec::new();

        let diagonal_index = diagonal_index as i32;
        let rows = rows as i32;
        let columns = columns as i32;

        let start_col = 0.max(diagonal_index - rows);
        let diagonal_len = diagonal_index.min(columns - start_col).min(rows);

        for i in 0..diagonal_len {
            let x = rows.min(diagonal_index) - i - 1;
            let y = start_col + i;

            let x_rev = columns - rows.min(diagonal_index) + i;
            let y_rev = start_col + i;

            let index: usize = (y * columns + x).try_into().unwrap();
            let index_rev: usize = (y_rev * columns + x_rev).try_into().unwrap();

            top_left_to_bottom_right_diagonal.push(index);
            top_right_to_bottom_left_diagonal.push(index_rev);
        }

        top_left_to_bottom_right.push(top_left_to_bottom_right_diagonal.clone());
        bottom_right_to_top_left.push(top_left_to_bottom_right_diagonal.iter().rev().cloned().collect());
        top_right_to_bottom_left.push(top_right_to_bottom_left_diagonal.clone());
        bottom_left_to_top_right.push(top_right_to_bottom_left_diagonal.iter().rev().cloned().collect());
    }

    let all_directions = vec![
        left_to_right_mapping,
        right_to_left_mapping,
        top_to_bottom_mapping,
        bottom_to_top_mapping,
        top_right_to_bottom_left,
        bottom_left_to_top_right,
        top_left_to_bottom_right,
        bottom_right_to_top_left,
    ];

    let mut xmas_count = 0;

    for row_mapping in all_directions {
        row_mapping.iter().map(|orig_indexes| {
            let row_str = orig_indexes.iter().map(|&i| chars[i]).collect::<String>();
            (orig_indexes, row_str)
        }).for_each(|(orig_indexes, row_str)| {
            for m in re.find_iter(row_str.as_str()) {
                xmas_count += 1;
                for i in m.start()..m.end() {
                    output[orig_indexes[i]] = chars[orig_indexes[i]];
                }
            }
        });
    }

    println!("output:");
    print_field(rows, columns, output);

    println!();
    println!("XMAS count: {}", xmas_count);
}

fn get_index_from_pos(columns: usize, x: usize, y: usize) -> usize {
    return columns * y + x;
}

fn part_two() {
    const KERNELS: [[[char; 3]; 3]; 4] = [
        [
            ['M', '.', 'S'],
            ['.', 'A', '.'],
            ['M', '.', 'S'],
        ],
        [
            ['S', '.', 'M'],
            ['.', 'A', '.'],
            ['S', '.', 'M'],
        ],
        [
            ['M', '.', 'M'],
            ['.', 'A', '.'],
            ['S', '.', 'S'],
        ],
        [
            ['S', '.', 'S'],
            ['.', 'A', '.'],
            ['M', '.', 'M'],
        ],
    ];

    let columns = INPUT.find('\n').unwrap();
    let rows = INPUT.lines().count();

    let chars = INPUT.chars().filter(|&c| c != '\n').collect::<Vec<char>>();
    let mut output = vec!['.'; columns * rows];

    let mut xmas_count = 0;

    for y in 1..rows-1 {
        for x in 1..columns-1 {
            let sample = [
                [chars[get_index_from_pos(columns, x-1, y-1)], '.', chars[get_index_from_pos(columns, x+1, y-1)]],
                ['.', chars[get_index_from_pos(columns, x, y)], '.'],
                [chars[get_index_from_pos(columns, x-1, y+1)], '.', chars[get_index_from_pos(columns, x+1, y+1)]],
            ];

            if KERNELS.contains(&sample) {
                xmas_count += 1;
                for i in 0..3 {
                    for j in 0..3 {
                        if sample[i][j] != '.' {
                            output[get_index_from_pos(columns, x+i-1, y+j-1)] = chars[get_index_from_pos(columns, x+i-1, y+j-1)];
                        }
                    }
                }
            }
        }
    }

    print_field(rows, columns, output);

    println!();
    println!("X-MAS count: {}", xmas_count);
}

fn main() {
    println!("Part 1:");
    part_one();
    println!("Part 2:");
    part_two();
}

