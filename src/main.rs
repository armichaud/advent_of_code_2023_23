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

fn longest_path(matrix: &DMatrix<char>, start: (usize, usize), visited: HashSet<(usize, usize)>) -> Option<usize> {
    if start.0 == matrix.nrows() - 1 {
        return Some(0);
    }
    // TODO
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
