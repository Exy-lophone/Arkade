use crate::tictactoe::board::Board;
use crate::tictactoe::event_handler::EventHandler;
use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
};
use std::io;
use std::io::ErrorKind;
use std::sync::mpsc::TryRecvError;

pub enum CellType {
    Cross,
    Circle,
    Empty,
}

pub struct TicTacToe {
    exit: bool,
    bitboard_cross: u16,
    bitboard_circle: u16,
    event_handler: EventHandler,
}

impl TicTacToe {
    pub fn new() -> Self {
        TicTacToe {
            exit: false,
            bitboard_circle: 0,
            bitboard_cross: 0,
            event_handler: EventHandler::new(),
        }
    }

    pub fn from(bitboard_cross: u16, bitboard_circle: u16) -> Self {
        TicTacToe {
            exit: false,
            bitboard_circle,
            bitboard_cross,
            event_handler: EventHandler::new(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            match self.event_handler.rx.try_recv() {
                Ok(e) => {
                    self.handle_events(e);
                    Ok(())
                }
                Err(TryRecvError::Empty) => Ok(()),
                Err(TryRecvError::Disconnected) => {
                    Err(io::Error::new(ErrorKind::BrokenPipe, "channel disconected"))
                }
            }?;
            terminal.draw(|frame| {
                let layouts = self.get_layouts(frame.area());
                self.render_board(frame, layouts.0);
                self.render_history(frame, layouts.1);
                self.render_tips(frame, layouts.2);
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

    fn render_tips(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(
            Paragraph::new("Use the numpad key to place a X/O")
                .block(Block::new().title(" <Tips> ").borders(Borders::ALL)),
            area,
        );
    }

    fn handle_events(&mut self, e: Event) {
        match e {
            Event::Key(key_event) => self.handle_key_events(key_event),
            _ => {}
        }
    }

    fn handle_key_events(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') if key_event.is_press() => self.exit = true,
            _ => {}
        }
    }
}
