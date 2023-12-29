// mod sudoku_board;

#[cfg(test)]
mod tests {
    use crate::utils::solve;

	#[test]
	#[should_panic]
	fn test_empty_board(){
		solve("test-boards/empty.txt");
	}

	#[test]
	#[should_panic]
	fn test_invalid_num_rows(){
		solve("test-boards/invalid-num-rows.txt");
	}
}