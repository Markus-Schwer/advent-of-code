const INPUT: &str = include_str!("../../resources/2024_02.txt");

fn is_report_safe(levels: Vec<i32>, index: usize, previous_diff: i32, tolerations: i32) -> bool {
    if tolerations > 1 {
        return false;
    }

    if index == levels.len()-1 {
        return true;
    }

    let diff = levels[index] - levels[index+1];
    let abs_diff = diff.abs();

    if abs_diff < 1 || abs_diff > 3 || (diff < 0 && previous_diff > 0) || (diff > 0 && previous_diff < 0) {
        for index_to_remove in 0..levels.len() {
            let alternative_levels = [&levels[0..index_to_remove], &levels[index_to_remove+1..levels.len()]].concat();
            if is_report_safe(alternative_levels, 0, 0, tolerations+1) {
                return true;
            }
        }

        return false;
    }

    return is_report_safe(levels, index+1, diff, tolerations);
}

fn count_safe_reports(input: &str, tolerations: i32) -> i32 {
    let lines = input.lines();
    let mut count_safe_reports = 0;

    for line in lines {
        let levels = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let safe = is_report_safe(levels, 0, 0, tolerations);

        if safe {
            count_safe_reports += 1;
        }
    }

    return count_safe_reports;
}

fn main() {
    println!("Count of safe reports: {}", count_safe_reports(INPUT, 1));
    println!("Count of safe reports with Problem Dampener: {}", count_safe_reports(INPUT, 0));
}
