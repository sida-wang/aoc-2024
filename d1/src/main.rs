use std::fs;
use std::iter;

fn main() {
    let contents =
        fs::read_to_string("./src/data.txt").expect("Should have been able to read file");

    let mut left_ids: Vec<i32> = vec![];
    let mut right_ids: Vec<i32> = vec![];

    for line in contents.split("\r\n") {
        if !line.is_empty() {
            let (left, right) = line.split_once("   ").unwrap();
            left_ids.push(left.parse().unwrap());
            right_ids.push(right.parse().unwrap());
        }
    }

    println!("sorting");
    left_ids.sort();
    right_ids.sort();

    let mut sum = 0;

    //Part 1
    for (left, right) in iter::zip(left_ids.clone(), right_ids.clone()) {
        sum += left.abs_diff(right);
    }

    println!("Sum for Part 1 is {sum}");

    let mut sum = 0;

    //Part 2
    for left in left_ids {
        let mut right_copy = right_ids.clone();
        right_copy.retain(|&x| x == left);
        sum += (left as usize) * right_copy.len();
    }

    println!("Sum for Part 2 is {sum}");
}
