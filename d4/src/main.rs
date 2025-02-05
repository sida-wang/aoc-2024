use std::fs;

fn main() {
    let contents = fs::read_to_string("./src/data.txt").unwrap();
    let contents = contents.trim();
    const M: usize = 140;
    const TARGET: &str = "XMAS";
    let mut count = 0;

    let mut wordsearch = [['.'; M]; M];

    for (i, line) in contents.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            wordsearch[i][j] = c;
        }
    }

    fn dfs(
        i: i32,
        j: i32,
        target: &str,
        count: &mut i32,
        wordsearch: &[[char; M]; M],
        diff: Option<(i8, i8)>,
    ) {
        if i < 0 || i >= (M as i32) || j < 0 || j >= (M as i32) {
            return;
        }
        if wordsearch[i as usize][j as usize] != target.chars().next().unwrap() {
            return;
        }
        let new_target = &target[1..];
        if new_target.is_empty() {
            *count += 1;
            return;
        }

        match diff {
            Some((a, b)) => dfs(
                i + (a as i32),
                j + (b as i32),
                new_target,
                count,
                wordsearch,
                Some((a, b)),
            ),
            None => {
                dfs(i - 1, j - 1, new_target, count, wordsearch, Some((-1, -1)));
                dfs(i, j - 1, new_target, count, wordsearch, Some((0, -1)));
                dfs(i + 1, j - 1, new_target, count, wordsearch, Some((1, -1)));
                dfs(i + 1, j, new_target, count, wordsearch, Some((1, 0)));
                dfs(i + 1, j + 1, new_target, count, wordsearch, Some((1, 1)));
                dfs(i, j + 1, new_target, count, wordsearch, Some((0, 1)));
                dfs(i - 1, j + 1, new_target, count, wordsearch, Some((-1, 1)));
                dfs(i - 1, j, new_target, count, wordsearch, Some((-1, 0)));
            }
        }
    }

    for i in 0..(M as i32) {
        for j in 0..(M as i32) {
            dfs(i, j, TARGET, &mut count, &wordsearch, None);
        }
    }

    println!("Answer to part 1 is {count}");

    let mut count2 = 0;

    fn is_xmas(i: i32, j: i32, wordsearch: &[[char; M]; M]) -> bool {
        const VALID_DIAG: [char; 2] = ['M', 'S'];
        let i = i as usize;
        let j = j as usize;

        if wordsearch[i][j] != 'A' {
            return false;
        }

        if !VALID_DIAG.contains(&wordsearch[i - 1][j - 1]) {
            return false;
        }

        if !VALID_DIAG.contains(&wordsearch[i - 1][j + 1]) {
            return false;
        }

        if !VALID_DIAG.contains(&wordsearch[i + 1][j - 1]) {
            return false;
        }

        if !VALID_DIAG.contains(&wordsearch[i + 1][j + 1]) {
            return false;
        }

        if wordsearch[i - 1][j - 1] == wordsearch[i + 1][j + 1] {
            return false;
        }

        if wordsearch[i + 1][j - 1] == wordsearch[i - 1][j + 1] {
            return false;
        }

        true
    }

    for i in 1..(M as i32 - 1) {
        for j in 1..(M as i32 - 1) {
            if is_xmas(i, j, &wordsearch) {
                count2 += 1;
            }
        }
    }
    println!("Answer to part 2 is {count2}");
}
