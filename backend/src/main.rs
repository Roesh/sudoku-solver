mod utils;
mod sudoku_board;
mod tests;

//https://github.com/snapview/tokio-tungstenite/blob/master/examples/echo-server.rs
fn main() {
	utils::solve("test-boards/valid-final.txt");
}

