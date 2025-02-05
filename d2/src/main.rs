use std::fs;
use std::iter::zip;

fn main() {
    let contents =
        fs::read_to_string("./src/data.txt").expect("Should have been able to read file");

    let mut safe = 0;
    let mut dampened_safe = 0;

    'report_loop: for line in contents.split("\r\n") {
        if !line.is_empty() {
            let report_values: Vec<i32> =
                line.split(' ').map(|x| x.parse::<i32>().unwrap()).collect();
            if check_report(&report_values) {
                safe += 1;
            } else {
                for i in 0..report_values.len() {
                    let mut modified_report_values = report_values.clone();
                    modified_report_values.remove(i);
                    if check_report(&modified_report_values) {
                        dampened_safe += 1;
                        continue 'report_loop;
                    }
                }
            }
        }
    }

    println!("Safe levels = {safe}");
    println!("Safe dampened levels = {}", safe + dampened_safe);
}

fn check_report(values: &[i32]) -> bool {
    let positive_sign = values[1] - values[0] > 0;
    for (val1, val2) in zip(values[..values.len()].iter(), values[1..].iter()) {
        let abs_diff = val1.abs_diff(*val2);
        if !(1..=3).contains(&abs_diff) {
            return false;
        }
        if (val2 - val1 > 0) != positive_sign {
            return false;
        }
    }
    true
}
