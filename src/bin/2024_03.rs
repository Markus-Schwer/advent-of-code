use regex::Regex;

const INPUT: &str = include_str!("../../resources/2024_03.txt");

fn main() {
    let re = Regex::new(r"(?s)(?:don't\(\)(?:.*?do\(\)|.*?$))|mul\(([0-9]{0,3}),([0-9]{0,3})\)").unwrap();
    let mut sum = 0;

    for captures in re.captures_iter(INPUT) {
        let x = captures.get(1).map(|x| x.as_str());
        let y = captures.get(2).map(|y| y.as_str());

        if x.is_some() && y.is_some() {
            let x = x.unwrap().parse::<i32>().unwrap();
            let y = y.unwrap().parse::<i32>().unwrap();
            println!("{} * {} = {}", x, y, x * y);
            sum += x * y;
        }
    }

    println!("sum: {}", sum);
}
