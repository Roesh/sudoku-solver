use std::{collections::HashSet, fs};
use crate::sudoku_board::*;

pub fn solve(path_to_board: &str) {
    let initial_board = create_initial_board(path_to_board);
    // initial_board[0][0];
    if !is_valid(&initial_board) {
        panic!("Board is invalid, exiting")
    }
    print_board(&initial_board);
}

fn create_initial_board(file_path: &str) -> SudokuBoardValues {
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let initial_board: SudokuBoardValues = contents
        .split('\n')
        .map(|line| {
            println!("Parsing line: {}", line);

            // unbound_board.push()
            let new_row = line
                .split('|')
                .map(|val| {
                    // println!("Parsing value: {}", val);
                    if val.trim() == "" {
                        return SudokuCell {
                            filled: false,
                            value: 0,
                        };
                    }
                    let parsed = val
                        .trim()
                        .parse::<u16>()
                        .expect("Unable to parse numeric digit");
                    if parsed > 9 {
                        panic!("Value was greater than 9");
                    }
                    SudokuCell {
                        filled: true,
                        value: parsed,
                    }
                })
                .collect();

            new_row
        })
        .collect();

    initial_board
}

fn print_board(board: &SudokuBoardValues) {
    board.iter().for_each(|row| {
        row.iter().for_each(|cell| {
            if cell.filled {
                print!("{}\t|", cell.value);
            } else {
                print!("[ ]\t|")
            }
        });
        println!();
    })
}

fn is_valid(board: &SudokuBoardValues) -> bool {
    // Board Size valid
    if board.len() != 9 {
        println!("Invalid number of rows. Row count is {}", board.len());
        return false;
    }
    let mut all_rows_valid = true;

    board.iter().enumerate().for_each(|(row_index, row)| {
        if row.len() != 9 {
            println!("Row {} has invalid number of entries", row_index + 1);
            all_rows_valid = false;
        }
    });

    if !all_rows_valid {
        return false;
    }

    // Rows valid
    let mut row_values_map: HashSet<u16> = HashSet::with_capacity(9);
    let mut column_values_map: HashSet<u16> = HashSet::with_capacity(9);

    for row in 0..=9 {
        print!("curr row {}", row);
        let mut row_sum = 0;
        let mut column_sum = 0;

        row_values_map.clear();
        column_values_map.clear();

        for column in 0..=9 {
            let row_value = board[row][column].value;
            let column_value = board[column][row].value;

            row_sum += row_value;
            column_sum += column_value;

            if row_values_map.contains(&row_value) {
                println!("Duplicate value {row_value} found in row {row}");
                return false;
            }
            row_values_map.insert(row_value);

            if column_values_map.contains(&column_value) {
                println!("Duplicate value {column_value} found in column {row}");
                return false;
            }
            column_values_map.insert(column_value);
        }

        if row_sum > 45 || column_sum > 45 {
            if row_sum > 45 {
                println!("Row sum at {} exceeds limits", row)
            }
            if column_sum > 45 {
                println!("Column sum at {} exceeds limits", row)
            }
            // println!()
            return false;
        }
    }
    // let mut row_sum = 0;
    // let mut column_sum = 0;
    // [0..10].iter().for_each(|column:&Range<usize>| {
    // 	row_sum += board[row as usize][column];
    // 	column_sum += board[column as usize][row]
    // })
    // })
    // Columns valid

    // Subgrids valid

    true
}
