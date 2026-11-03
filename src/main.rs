mod tictactoe;

use crate::tictactoe::tictactoe::TicTacToe;
use std::io;

fn main() -> io::Result<()> {
    // let mut tictactoe = TicTacToe::from(0b011_100_001, 0b100_011_110);
    let mut tictactoe = TicTacToe::new();
    ratatui::run(|terminal| tictactoe.run(terminal))?;
    Ok(())
}
