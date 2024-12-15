use std::{collections::HashMap, ops::{AddAssign, SubAssign}};

const INPUT: &str = include_str!("../../resources/2024_11.txt");

fn blink(stone_map: &mut HashMap<u64, u64>) {
    for (&stone, &count) in stone_map.clone().iter() {
        stone_map.entry(stone).or_insert(count).sub_assign(count);
        if stone == 0 {
            stone_map.entry(1).or_insert(0).add_assign(count);
            println!("0 -> 1");
            continue;
        }

        let stone_str = stone.to_string();
        let digits_len = stone_str.len();
        if digits_len % 2 == 0 {
            let (left, right) = stone_str.split_at(digits_len / 2);
            let (left, right) = (left.parse::<u64>().unwrap(), right.parse::<u64>().unwrap());

            stone_map.entry(left).or_insert(0).add_assign(count);
            stone_map.entry(right).or_insert(0).add_assign(count);
            println!("{} -> {}, {}", stone, left, right);
            continue;
        }

        stone_map.entry(stone * 2024).or_insert(0).add_assign(count);
        println!("{} -> {}", stone, stone * 2024);
    }
}

fn main() {
    let stones: Vec<u64> = INPUT.trim_end().split(' ').map(|x| x.parse::<u64>().unwrap()).collect();
    let mut stone_map: HashMap<u64, u64> = HashMap::new();

    for stone in stones {
        let stone_count = stone_map.entry(stone).or_insert(0);
        stone_count.add_assign(1);
    }

    println!("{:?}", stone_map);

    for _ in 0..75 {
        blink(&mut stone_map);
    }

    let mut sum = 0;
    for (_, &count) in stone_map.iter() {
        sum += count;
    }

    println!("sum {}", sum);
}
