use regex::Regex;
use std::fs;

fn main() {
    let contents = fs::read_to_string("./src/data.txt").expect("Should be able to read file");
    let contents = contents.split("\r\n").collect::<Vec<&str>>().join("");
    let condition_re = Regex::new(r"(?:^|do\(\))(.*?)(?:don't\(\)|$)").unwrap();
    let mut sum = 0;

    for (_, [section]) in condition_re.captures_iter(&contents).map(|x| x.extract()) {
        sum += compute(section)
    }

    println!("Result is {}", compute(&contents));
    println!("Condition Result is {sum}");
}

fn compute(text: &str) -> i32 {
    let re = Regex::new(r"(mul\((\d+),(\d+)\))").unwrap();
    let mut sum = 0;
    for (_, [_mult, left, right]) in re.captures_iter(text).map(|x| x.extract()) {
        let left = left.parse::<i32>().expect("Should parse without issue");
        let right = right.parse::<i32>().expect("Should parse without issue");
        sum += left * right;
    }
    sum
}
