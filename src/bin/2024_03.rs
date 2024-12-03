use regex::Regex;

const INPUT: &str = include_str!("../../resources/2024_03.txt");

fn main() {
    let re = Regex::new(r"(don't\(\))|(do\(\))|(mul\(([0-9]{1,3}),([0-9]{1,3})\))").unwrap();
    let mut sum = 0;
    let mut enabled = true;

    for captures in re.captures_iter(INPUT) {
        let operator = captures.get(0).unwrap().as_str();
        let x = captures.get(4).map(|x| x.as_str());
        let y = captures.get(5).map(|y| y.as_str());

        match operator {
            "don't()" => {
                enabled = false;
            }
            "do()" => {
                enabled = true;
            }
            _ => {
                if enabled && x.is_some() && y.is_some() {
                    let x = x.unwrap().parse::<i32>().unwrap();
                    let y = y.unwrap().parse::<i32>().unwrap();
                    println!("{} * {} = {}", x, y, x * y);
                    sum += x * y;
                }
            }
        }
    }

    println!("sum: {}", sum);
}
