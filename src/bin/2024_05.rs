use std::collections::HashMap;

use regex::Regex;
use colored::Colorize;

const INPUT: &str = include_str!("../../resources/2024_05.txt");

fn topological_sort(page_number: i32, update: &Vec<i32>, applicable_rules: &Vec<(i32, i32)>, visited: &mut HashMap<i32, bool>, stack: &mut Vec<i32>) {
    visited.insert(page_number, true);

    let applicable_xes: Vec<i32> = applicable_rules.iter()
        .filter(|rule| rule.1 == page_number)
        .map(|&(x, _)| x)
        .collect();

    for x in applicable_xes {
        if !visited.get(&x).unwrap_or(&false) {
            topological_sort(x, update, applicable_rules, visited, stack);
        }
    }

    stack.push(page_number);
}

fn main() {
    let re = Regex::new(r"([0-9]+)\|([0-9]+)|([0-9]+,?)+").unwrap();

    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    for caps in re.captures_iter(INPUT) {
        if caps.get(1).is_some() && caps.get(2).is_some() {
            let x = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let y = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
            rules.push((x, y));
        } else if caps.get(3).is_some() {
            let update: Vec<i32> = caps.get(0).unwrap().as_str().split(",").map(|s| s.parse::<i32>().unwrap()).collect();
            updates.push(update);
        }
    }

    let mut sum = 0;
    let mut sum_reordered = 0;

    for update in updates {
        let update_len = update.len();

        let mut correct = true;

        let mut applicable_rules = Vec::new();

        for &rule in &rules {
            let x_index = update.iter().enumerate().find(|(_, &page_number)| page_number == rule.0).map(|(i, _)| i);
            let y_index = update.iter().enumerate().find(|(_, &page_number)| page_number == rule.1).map(|(i, _)| i);

            let (Some(x_index), Some(y_index)) = (x_index, y_index) else {
                continue;
            };

            applicable_rules.push(rule);

            if x_index > y_index {
                correct = false;
            }
        }

        let reordered_update: Vec<i32>;

        if !correct {
            let mut stack: Vec<i32> = Vec::new();
            let mut visited: HashMap<i32, bool> = HashMap::new();
            for &page_number in &update {
                if !visited.get(&page_number).unwrap_or(&false) {
                    topological_sort(page_number, &update, &applicable_rules, &mut visited, &mut stack);
                }
            }

            reordered_update = stack;
            if update_len != reordered_update.len() {
                println!("lenghts do not match");
            }
        } else {
            reordered_update = update;
        }

        for (i, page_number) in reordered_update.iter().enumerate() {
            if i == update_len / 2 && correct {
                sum += page_number;
                print!("{},", page_number.to_string().green());
            } else if i == update_len / 2 && !correct {
                sum_reordered += page_number;
                print!("{},", page_number.to_string().red());
            } else if i == update_len - 1 {
                println!("{}", page_number);
            } else {
                print!("{},", page_number);
            }
        }
    }

    println!("Sum: {}", sum);
    println!("Sum re-ordered: {}", sum_reordered);
}
