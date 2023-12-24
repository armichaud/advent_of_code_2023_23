use std::{io::{BufReader, BufRead}, fs::File, collections::{HashSet, HashMap}};
use nalgebra::DMatrix;

const FOREST: char = '#';
const PATH: char = '.';
const SLOPE_UP: char = '^';
const SLOPE_DOWN: char = 'v';
const SLOPE_LEFT: char = '<';
const SLOPE_RIGHT: char = '>';

#[derive(Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn can_descend_slope(&self, slope: char) -> bool {
        match self {
            Direction::Up => slope != SLOPE_DOWN,
            Direction::Down => slope != SLOPE_UP,
            Direction::Left => slope != SLOPE_RIGHT,
            Direction::Right => slope != SLOPE_LEFT
        }
    }
}

fn get_matrix(file: &str) -> DMatrix<char> {
    let file = File::open(file).unwrap();
    let lines = BufReader::new(file).lines();
    let mut nrows = 0;
    let mut data = Vec::<char>::new();
    for line in lines {
        data.extend(line.unwrap().chars());
        nrows += 1;
    }
    DMatrix::from_row_slice(nrows, data.len() / nrows, &data)
}

fn get_start(matrix: &DMatrix<char>) -> Option<(usize, usize)> {
    let first_row = matrix.row(0);
    for col in 0..matrix.ncols() {
        if first_row[col] == PATH {
            return Some((0, col));
        }
    }
    None
}

fn longest_path(matrix: &DMatrix<char>, current: (usize, usize), visited: HashSet<(usize, usize)>) -> Option<usize> {
    if current.0 == matrix.nrows() - 1 {
        return Some(0);
    }
    let mut visited = visited;
    visited.insert(current);
    match matrix[current] {
        SLOPE_UP => {
            if let Some(n) = longest_path(matrix, (current.0 - 1, current.1), visited.clone()) {
                return Some(n + 1);
            }
        }
        SLOPE_DOWN => {
            if let Some(n) = longest_path(matrix, (current.0 + 1, current.1), visited.clone()) {
                return Some(n + 1);
            }
        }
        SLOPE_LEFT => {
            if let Some(n) = longest_path(matrix, (current.0, current.1 - 1), visited.clone()) {
                return Some(n + 1);
            }
        }
        SLOPE_RIGHT => {
            if let Some(n) =longest_path(matrix, (current.0, current.1 + 1), visited.clone()) {
                return Some(n + 1);
            }
        }
        _ => {
            let neighbors = HashMap::from([
                (Direction::Up, (current.0 as i32 - 1, current.1 as i32)), 
                (Direction::Down, (current.0 as i32 + 1, current.1 as i32)), 
                (Direction::Left, (current.0 as i32, current.1 as i32 - 1)), 
                (Direction::Right, (current.0 as i32, current.1 as i32 + 1))
            ]);
            let mut longest: Option<usize> = None;
            for (direction, neighbor_i32) in neighbors {
                if neighbor_i32.0 < 0 || neighbor_i32.0 > matrix.nrows() as i32 - 1 || neighbor_i32.1 < 0 || neighbor_i32.1 > matrix.ncols() as i32 - 1 {
                    continue;
                }
                let neighbor_usize = (neighbor_i32.0 as usize, neighbor_i32.1 as usize);
                let neighbor = matrix[neighbor_usize];
                if neighbor != FOREST && direction.can_descend_slope(neighbor) && !visited.contains(&neighbor_usize) {
                    if let Some(n) = longest_path(matrix, neighbor_usize, visited.clone()) {
                        longest = Some(longest.unwrap_or(0).max(n));
                    }
                }
            }
            if let Some(longest) = longest {
                return Some(longest + 1)
            }
        }
    }
    None
}

fn solution(file: &str) -> usize {
    let matrix = get_matrix(file);
    longest_path(&matrix, get_start(&matrix).expect("First row does not contain a path"), HashSet::new()).expect("No path found")
}

fn main() {
    assert_eq!(solution("example.txt"), 94);
    assert_eq!(solution("input.txt"), 2170);
}
