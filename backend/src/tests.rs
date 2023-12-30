// mod sudoku_board;

#[cfg(test)]
mod tests {
    use crate::utils::{create_initial_board, is_valid, solve};

    #[test]
    fn test_empty_board_invalid() {
        assert_eq!(
            is_valid(&create_initial_board("test-boards/empty.txt")),
            false
        );
    }
	
	
	#[test]
	fn test_num_rows_invalid() {
        assert_eq!(
            is_valid(&create_initial_board("test-boards/invalid-num-rows.txt")),
            false
        );
    }

    #[test]
    #[should_panic]
    fn test_empty_board() {
        solve("test-boards/empty.txt");
    }

    #[test]
    #[should_panic]
    fn test_invalid_num_rows() {
        solve("test-boards/invalid-num-rows.txt");
    }
}
