use regex::Regex;

const INPUT: &str = include_str!("../../resources/2024_01.txt");

fn main() {
    let mut first_list = Vec::<i32>::new();
    let mut second_list = Vec::<i32>::new();

    let re = Regex::new(r"([0-9]+) +([0-9]+)").unwrap();

    let mut count = 0;
    let lines = INPUT.lines();
    for line in lines {
        let Some(caps) = re.captures(line) else {
            println!("Could not parse line: {}", line);
            return;
        };

        let first = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let second = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();

        first_list.push(first);
        second_list.push(second);
        count += 1;
    }

    first_list.sort();
    second_list.sort();

    // part one
    let sum_distances: i32 = (0..count).into_iter().map(|i| (first_list[i] - second_list[i]).abs()).sum();
    println!("Sum of distances: {}", sum_distances);

    // part two
    let mut total_similarity = 0;
    for first in &first_list {
        let mut occasions = 0;
        for second in &second_list {
            if first == second {
                occasions += 1;
            }
        }

        let similarity = first * occasions;
        total_similarity += similarity;
    }

    println!("Total similarity score: {}", total_similarity);
}
