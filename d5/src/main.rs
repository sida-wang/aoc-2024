use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("./src/data.txt").expect("Should have data");
    let mut lines = contents.lines();
    let mut rule_map = HashMap::new();

    for line in &mut lines {
        if line.is_empty() {
            break;
        };

        let line_values: Vec<u32> = line
            .split('|')
            .filter_map(|str| str.trim().parse::<u32>().ok())
            .collect();
        let map_entry = rule_map.entry(line_values[0]).or_insert(Vec::new());
        map_entry.push(line_values[1]);
    }

    let mut pt1_sum = 0;
    let mut pt2_sum = 0;

    for line in lines {
        let line_array: Vec<u32> = line
            .split(',')
            .filter_map(|x| x.trim().parse::<u32>().ok())
            .collect();
        if check_rules(&rule_map, &line_array) {
            pt1_sum += line_array.get(line_array.len() / 2).unwrap_or(&0)
        } else {
            let mut array = Vec::new();
            for num in line_array {
                array = place_in_order(&rule_map, array, num);
            }
            pt2_sum += array.get(array.len() / 2).unwrap_or(&0)
        }
    }

    println!("Sum of middle pages of valid manuals is {pt1_sum}");
    println!("Sum of middle pages of invalid manuals after reordering is {pt2_sum}");

    fn check_rules(rule_map: &HashMap<u32, Vec<u32>>, pages: &[u32]) -> bool {
        let mut forbidden: Vec<u32> = Vec::new();
        for num in pages.iter().rev() {
            if forbidden.contains(num) {
                return false;
            }
            if let Some(rules) = rule_map.get(num) {
                forbidden.extend(rules);
            }
        }
        true
    }

    fn place_in_order(
        rule_map: &HashMap<u32, Vec<u32>>,
        mut pages: Vec<u32>,
        value: u32,
    ) -> Vec<u32> {
        let mut index = pages.len();
        if let Some(map) = rule_map.get(&value) {
            for (i, page) in pages.iter().enumerate() {
                if map.contains(page) {
                    index = i;
                    break;
                }
            }
        }
        pages.insert(index, value);
        pages
    }

    // This should generate the manual in an order that satisfies all the rules. Worth exploring
    // how to apply this in an ordered set manner.
    // let mut full_order = Vec::new();
    // for num in rule_map.keys() {
    //     full_order = place_in_order(&rule_map, full_order, *num)
    // }
}
