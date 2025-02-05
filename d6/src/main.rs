use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("./src/data.txt").unwrap();
    let mut matrix = Vec::new();
    let mut starting_position: Option<(usize, usize)> = None;
    let mut direction = Direction::North;

    for (i, line) in contents.lines().enumerate() {
        let mut row_vec = Vec::new();
        for (j, c) in line.chars().enumerate() {
            if c == '^' {
                starting_position = Some((i, j));
            }
            row_vec.push(Cell {
                cell_type: CellType::from_char(&c),
                traversed_directions: HashSet::new(),
            });
        }
        matrix.push(row_vec);
    }

    let num_rows = matrix.len();
    let num_cols = matrix[0].len();

    for row in &matrix {
        assert_eq!(row.len(), num_cols);
    }
    assert!(starting_position.is_some());

    println!("Matrix is {num_rows} by {num_cols}");
    println!("Starting position is {starting_position:?}");

    // let mut matrix_pt2 = Vec::new();
    // for row in matrix.iter() {
    //     let mut row_vec = Vec::new();
    //     for entry in row.iter() {
    //         row_vec.push((*entry, HashSet::<Direction>::new()));
    //     }
    //     matrix_pt2.push(row_vec);
    // }

    let mut position: Option<(usize, usize)> = starting_position;

    while let Some(loop_position) = position {
        let cell = &mut matrix[loop_position.0][loop_position.1];
        cell.cell_type = CellType::Traversed;
        cell.traversed_directions.insert(direction);
        let new_position = update_position(loop_position, &direction, (num_rows, num_cols));
        if let Some(new_position) = new_position {
            if matrix[new_position.0][new_position.1].cell_type == CellType::Obstacle {
                direction = direction.next();
            } else {
                position = Some(new_position);
            }
        } else {
            break;
        }
    }

    println!(
        "The number of squares accessed by the guard is {}",
        count_traversed(&matrix)
    );

    fn count_traversed(matrix: &Vec<Vec<Cell>>) -> usize {
        let mut count: usize = 0;

        for row in matrix {
            for c in row {
                if c.cell_type == CellType::Traversed {
                    count += 1;
                }
            }
        }

        count
    }

    fn update_position(
        position: (usize, usize),
        direction: &Direction,
        limits: (usize, usize),
    ) -> Option<(usize, usize)> {
        let step_offset = direction.step_offset();
        let i = position.0 as isize + step_offset.0;
        let j = position.1 as isize + step_offset.1;
        if !(0..limits.0 as isize).contains(&i) {
            return None;
        }
        if !(0..limits.1 as isize).contains(&j) {
            return None;
        }
        Some((i as usize, j as usize))
    }
}

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn next(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn step_offset(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }
}

struct Cell {
    cell_type: CellType,
    traversed_directions: HashSet<Direction>,
}

#[derive(PartialEq)]
enum CellType {
    Traversed,
    Untraversed,
    Obstacle,
}

impl CellType {
    fn from_char(c: &char) -> Self {
        match c {
            '#' => Self::Obstacle,
            _ => Self::Untraversed,
        }
    }
}
