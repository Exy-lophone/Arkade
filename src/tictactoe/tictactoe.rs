use crate::tictactoe::board::Board;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
};
use std::io;
use std::time::Duration;

pub enum CellType {
    Cross,
    Circle,
    Empty,
}

pub struct TicTacToe {
    running: bool,
    bitboard_cross: u16,
    bitboard_circle: u16,
}

impl TicTacToe {
    pub fn new() -> Self {
        TicTacToe {
            running: true,
            bitboard_circle: 0,
            bitboard_cross: 0,
        }
    }

    pub fn from(bitboard_cross: u16, bitboard_circle: u16) -> Self {
        TicTacToe {
            running: true,
            bitboard_circle,
            bitboard_cross,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while self.running {
            self.handle_events()?;
            terminal.draw(|frame| {
                let layouts = self.get_layouts(frame.area());
                self.render_board(frame, layouts.0);
                self.render_history(frame, layouts.1);
                self.render_helper(frame, layouts.2);
            })?;
        }
        Ok(())
    }

    fn get_cell_type(&self, id: u16) -> CellType {
        if self.bitboard_cross & id == id {
            CellType::Cross
        } else if self.bitboard_circle & id == id {
            CellType::Circle
        } else {
            CellType::Empty
        }
    }

    fn get_layouts(&self, area: Rect) -> (Rect, Rect, Rect) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(75), Constraint::Percentage(25)])
            .split(area);
        let layout_top = layout[0];
        let layout_bot = layout[1];
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(75), Constraint::Percentage(25)])
            .split(layout_top);
        (layout[0], layout[1], layout_bot)
    }

    fn render_board(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(Board::new(self.bitboard_cross, self.bitboard_circle), area);
    }

    fn render_history(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Paragraph::new("(X => 0)")
                .block(Block::new().title(" <History> ").borders(Borders::ALL)),
            area,
        );
    }

    fn render_helper(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Paragraph::new("Use the numpad key to place a X/O")
                .block(Block::new().title(" <Tips> ").borders(Borders::ALL)),
            area,
        );
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    match key_event.code {
                        KeyCode::Char('q') => self.running = false,
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}
