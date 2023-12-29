use crate::sudoku_board::*;
use std::{collections::HashSet, fs};

pub fn solve(path_to_board: &str) {
    let initial_board = create_initial_board(path_to_board);
    // initial_board[0][0];
    if !is_valid(&initial_board) {
        panic!("Board is invalid, exiting");
    }
    print_board(&initial_board);
}

pub fn create_initial_board(file_path: &str) -> SudokuBoardValues {
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let initial_board: SudokuBoardValues = contents
        .split('\n')
        .map(|line| {
            // println!("Parsing line: {}", line);

            let new_row: [SudokuCell; 9] = line
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
                        .parse::<AllowedCellValue>()
                        .expect("Unable to parse numeric digit");
                    if parsed > 9 {
                        panic!("Value was greater than 9");
                    }
                    SudokuCell {
                        filled: true,
                        value: parsed,
                    }
                })
                .collect::<Vec<SudokuCell>>() //::<Vec<SudokuCell>>()
                .try_into()
                .unwrap();

            new_row
        })
        .collect::<Vec<[SudokuCell; 9]>>()
        .try_into()
        .unwrap();

    initial_board
}

fn print_board(board: &SudokuBoardValues) {
    println!("\n================= BOARD ================");
    board.iter().enumerate().for_each(|(row_ind, row)| {
        row.iter().enumerate().for_each(|(cell_ind, cell)| {
            if cell.filled {
                print!("{}\t|", cell.value);
            } else {
                print!("[ ]\t|");
            }
            if (cell_ind + 1) % 3 == 0 {
                print!("|\t");
            }
        });
        println!();
        if (row_ind + 1) % 3 == 0 {
            println!();
        }
    })
}

pub fn is_valid(board: &SudokuBoardValues) -> bool {
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

    // Rows and columns valid
    let mut row_values_map: HashSet<AllowedCellValue> = HashSet::with_capacity(9);
    let mut column_values_map: HashSet<AllowedCellValue> = HashSet::with_capacity(9);

    for row in 0..9 {
        // print!("curr row {}", row);
        let mut row_sum = 0;
        let mut column_sum = 0;

        row_values_map.clear();
        column_values_map.clear();

        for column in 0..9 {
            let row_value = board[row][column].value;
            let column_value = board[column][row].value;

            row_sum += row_value;
            column_sum += column_value;

            if board[row][column].filled && row_values_map.contains(&row_value) {
                println!("Duplicate value {row_value} found in row {row}");
                return false;
            }
            row_values_map.insert(row_value);

            if board[column][row].filled && column_values_map.contains(&column_value) {
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

    let mut grid_values_map: HashSet<AllowedCellValue> = HashSet::with_capacity(9);

    // Subgrids valid
    for sub_grid in 0..9 {
        let x_start_index = (sub_grid % 3) * 3;
        let x_end = x_start_index + 3;

        let y_start_index = (sub_grid / 3) * 3;
        let y_end = y_start_index + 3;

        let mut grid_sum = 0;
        grid_values_map.clear();

        for x in x_start_index..x_end {
            for y in y_start_index..y_end {
                grid_sum += board[x][y].value;
                if board[x][y].filled && grid_values_map.contains(&board[x][y].value) {
                    println!(
                        "Duplicate value {} found in grid {}",
                        board[x][y].value,
                        sub_grid + 1
                    );
                    return false;
                }
            }
        }

        if grid_sum > 45 {
            println!("Grid sum in grid {} exceeds limit", sub_grid + 1);
            return false;
        }
        // Debug subgrid
        // println!(
        //     "At subgrid {}, X start is {}, end is {}; Y start is {}, end is {}",
        //     sub_grid + 1, x_start_index, x_end, y_start_index, y_end
        // );
        // for y in 0..9 {
        //     for x in 0..9 {
        //         if x >= x_start_index && x < x_end && y >= y_start_index && y < y_end {
        //             print!("o\t");
        //         } else {
        //             print!("x\t");
        //         }
        //     }
        //     println!();
        // }
    }

    true
}

fn create_solver(initial_board: SudokuBoardValues) -> SudokuSolver {
    let mut working_board: [[EditableSudokuCell; 9]; 9] = Default::default();

    for row in 0..9 {
        for col in 0..9 {
            let modified_cell = &mut working_board[row as usize][col as usize];
            modified_cell.row_index = row;
            modified_cell.col_index = col;
            modified_cell.filled = initial_board[row as usize][col as usize].filled;
            modified_cell.value = initial_board[row as usize][col as usize].value;
            modified_cell.possible_values = HashSet::with_capacity(9);
        }
    }

    let mut solver = SudokuSolver {
        initial_board,
        working_board,
    };

    solver
}
