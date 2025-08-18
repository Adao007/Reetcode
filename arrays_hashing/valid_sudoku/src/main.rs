use std::collections::HashMap; 

struct Solution;
impl Solution {
    fn is_valid(sudoku: &Vec<Vec<char>>) -> bool {        
        if Self::check_rows(&sudoku) 
        && Self::check_columns(&sudoku) 
        && Self::check_subsquares(&sudoku) {
            return true;
        }

        false
    }

    fn check_rows(sudoku: &Vec<Vec<char>>) -> bool {
        for row in sudoku.iter() {
            let mut map = HashMap::new(); 
            for num in row.iter() {
                if *num == '.' { continue; }
                if map.contains_key(&num) { return false; }
                else { map.insert(num, 1); }
            }
        }

        true 
    }

    fn check_columns(sudoku: &Vec<Vec<char>>) -> bool {
        let length = sudoku[0].len();
        for column in 0..length {
            let mut map = HashMap::new(); 
            for num in 0..length {
                if sudoku[num][column] == '.' { continue; }
                if map.contains_key(&sudoku[num][column]) { return false; }
                else { map.insert(sudoku[num][column], 1); }
            }
        }

        true
    }

    fn check_subsquares(sudoku: &Vec<Vec<char>>) -> bool {
        let mut start_i = 0;
        let mut start_j = 0;

        for _ in 0..sudoku.len() {
            let i = start_i;
            let j = start_j;
            let mut map = HashMap::new();
            for row in i..(i+3) {
                for col in j..(j+3) {
                    if sudoku[row as usize][col as usize] == '.' { continue; }
                    if map.contains_key(&sudoku[row as usize][col as usize]) { return false; }
                    else { map.insert(sudoku[row as usize][col as usize], 1); }
                }
            }

            start_j += 3; 
            if start_j == 9 {
                start_i += 3;
                start_j = 0; 
            }
        }

        true
    }
}

fn main() {
    let mut sudoku: Vec<Vec<char>>  = vec![vec!['1', '2', '.', '.', '.', '.', '.', '.', '.'],
                                           vec!['4', '.', '.', '5', '.', '.', '.', '.', '.'],
                                           vec!['.', '9', '8', '.', '.', '.', '.', '.', '3'],
                                           vec!['5', '.', '.', '.', '6', '.', '.', '.', '4'],
                                           vec!['.', '.', '.', '8', '.', '3', '.', '.', '5'],
                                           vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                                           vec!['.', '.', '.', '.', '.', '.', '2', '.', '.'],
                                           vec!['.', '.', '.', '4', '1', '9', '.', '.', '8'],
                                           vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']];

    assert!(Solution::is_valid(&sudoku)); 

    sudoku = vec![vec!['1', '2', '.', '.', '3', '.', '.', '.', '.'],
                  vec!['4', '.', '.', '5', '.', '.', '.', '.', '.'],
                  vec!['.', '9', '1', '.', '.', '.', '.', '.', '3'],
                  vec!['5', '.', '.', '.', '6', '.', '.', '.', '4'],
                  vec!['.', '.', '.', '8', '.', '3', '.', '.', '5'],
                  vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                  vec!['.', '.', '.', '.', '.', '.', '2', '.', '.'],
                  vec!['.', '.', '.', '4', '1', '9', '.', '.', '8'],
                  vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']];

    assert!(!Solution::is_valid(&sudoku));

    println!("is_valid function is working as intended!");
}
