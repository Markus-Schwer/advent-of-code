use itertools::Itertools;

const INPUT: &str = include_str!("../../resources/2024_07.txt");

const POSSIBLE_OPERATORS: [char; 3] = ['+', '*', '|'];

fn main() {
    let mut sum = 0;

    for line in INPUT.lines() {
        let mut split_iter = line.split(':');
        let test_value = split_iter.next().unwrap().parse::<u64>().unwrap();
        let numbers: Vec<u64> = split_iter.next().unwrap().split(' ').filter_map(|s| s.parse::<u64>().ok()).collect();

        let combinations = (0..numbers.len()-1).map(|_| POSSIBLE_OPERATORS.iter()).multi_cartesian_product().collect_vec();
        for operators in combinations {
            let mut working_numbers = numbers.clone();
            for i in 0..operators.len() {
                match operators[i] {
                    '+' => working_numbers[i+1] = working_numbers[i] + working_numbers[i + 1],
                    '*' => working_numbers[i+1] = working_numbers[i] * working_numbers[i + 1],
                    '|' => working_numbers[i+1] = working_numbers[i] * u64::pow(10, (working_numbers[i + 1].to_string().len()).try_into().unwrap()) + working_numbers[i + 1],
                    _ => panic!("Invalid operator"),

                }
            }

            if test_value == working_numbers[working_numbers.len() - 1] {
                for i in 0..operators.len() {
                    print!("{} {} ", numbers[i], operators[i]);
                }
                println!("{} = {} matches", numbers[operators.len()], working_numbers[operators.len()]);
                sum += test_value;
                // one combination matches, so no need to test the others
                break;
            }
        }
    }

    println!("sum {}", sum);
}
