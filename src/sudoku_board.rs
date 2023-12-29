use std::{collections::HashSet};

pub type SudokuBoardValues = [[SudokuCell;9];9];
pub type AllowedCellValue = u8;

#[derive(Debug)]
pub struct SudokuCell {
    pub filled: bool,
    pub value: AllowedCellValue,
}

pub struct EditableSudokuCell {
	pub row_index: AllowedCellValue,
	pub col_index: AllowedCellValue,

    pub filled: bool,
    pub value: AllowedCellValue,
	
	pub possible_values: HashSet<AllowedCellValue>
}

impl EditableSudokuCell {
	pub fn new() -> EditableSudokuCell {
		EditableSudokuCell { 
			col_index: 0,
			row_index: 0,
			filled: false,
			possible_values: Default::default(),
			value: 0
		}
	}
}
impl Default for EditableSudokuCell {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SudokuSolver {
	pub initial_board: SudokuBoardValues,
	pub working_board: [[EditableSudokuCell; 9]; 9],
	
	// pub 
}

// impl SudokuSolver {
//     fn new -> SudokuSolver {
//         SudokuSolver { initial_board: [[alive, ..GRID_SIZE], ..GRID_SIZE] }
//     }
// }
