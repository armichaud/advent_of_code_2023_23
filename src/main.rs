use std::{io::{BufReader, BufRead}, fs::File, collections::HashSet};
use nalgebra::DMatrix;

const FOREST: char = '#';
const PATH: char = '.';
const SLOPE_UP: char = '^';
const SLOPE_DOWN: char = 'v';
const SLOPE_LEFT: char = '<';
const SLOPE_RIGHT: char = '>';

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
        return Some(1);
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
            let neighbors: &[(i32, i32); 4] = &[
                (current.0 as i32 - 1, current.1 as i32 ), 
                (current.0 as i32 + 1, current.1 as i32), 
                (current.0 as i32, current.1 as i32 - 1), 
                (current.0 as i32, current.1 as i32 + 1)
            ];
            let mut longest = 0;
            for neighbor in neighbors {
                let neighbor_usize = (neighbor.0 as usize, neighbor.1 as usize);
                if neighbor.0 > -1 && 
                    neighbor_usize.0 < matrix.nrows() && 
                    neighbor.1 > -1 && 
                    neighbor_usize.1 < matrix.ncols() && 
                    matrix[neighbor_usize] != FOREST && 
                    !visited.contains(&neighbor_usize) {
                        longest = longest.max(longest_path(matrix, neighbor_usize, visited.clone()).unwrap_or(0));
                    }
            }
            if longest > 0 {
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
    assert_eq!(solution("input.txt"), 0);
}
