pub type SudokuBoardValues = Vec<Vec<SudokuCell>>;

pub struct SudokuCell {
    pub filled: bool,
    pub value: u16,
}

