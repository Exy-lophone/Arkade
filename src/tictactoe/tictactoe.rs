use crate::tictactoe::constant::*;
use crate::tictactoe::event_handler::EventHandler;
use crate::tictactoe::{board::Board, tictactoe::GameState::Playing};
use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::text::Line;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
};
use std::io;
use std::io::ErrorKind;
use std::sync::mpsc::TryRecvError;

enum Player {
    Cross,
    Circle,
}

enum GameState {
    Playing(Player),
    Win(Player),
    Draw,
}

pub struct TicTacToe {
    exit: bool,
    bitboard_cross: u16,
    bitboard_circle: u16,
    event_handler: EventHandler,
    game_state: GameState,
}

impl TicTacToe {
    pub fn new() -> Self {
        TicTacToe {
            exit: false,
            bitboard_circle: 0,
            bitboard_cross: 0,
            event_handler: EventHandler::new(),
            game_state: GameState::Playing(Player::Cross),
        }
    }

    pub fn from(bitboard_cross: u16, bitboard_circle: u16) -> Self {
        TicTacToe {
            exit: false,
            bitboard_circle,
            bitboard_cross,
            event_handler: EventHandler::new(),
            game_state: GameState::Playing(Player::Cross),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            self.handle_events()?;
            terminal.draw(|frame| {
                let layouts = self.get_layouts(frame.area());
                self.render_board(frame, layouts.0);
                self.render_history(frame, layouts.1);
                self.render_tips(frame, layouts.2);
            })?;
        }
        Ok(())
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
            Paragraph::new(vec![
                Line::from(""),
                Line::from(""),
                Line::from("Use numepad key to place X/O"),
                Line::from(""),
                Line::from("         7 | 8 | 9          "),
                Line::from("        ---+---+---         "),
                Line::from("         4 | 5 | 6          "),
                Line::from("        ---+---+---         "),
                Line::from("         1 | 2 | 3          "),
            ])
            .block(Block::new().title(" <Tips> ").borders(Borders::ALL)),
            area,
        );
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match self.event_handler.rx.try_recv() {
            Ok(Event::Key(key_event)) => {
                self.handle_key_events(key_event);
                Ok(())
            }
            Err(TryRecvError::Empty) => Ok(()),
            Err(TryRecvError::Disconnected) => {
                Err(io::Error::new(ErrorKind::BrokenPipe, "channel disconected"))
            }
            _ => Ok(()),
        }
    }

    fn handle_key_events(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') if key_event.is_press() => self.exit = true,
            KeyCode::Char('1') if key_event.is_press() => self.place(BOT_L),
            KeyCode::Char('2') if key_event.is_press() => self.place(BOT_M),
            KeyCode::Char('3') if key_event.is_press() => self.place(BOT_R),
            KeyCode::Char('4') if key_event.is_press() => self.place(MID_L),
            KeyCode::Char('5') if key_event.is_press() => self.place(MID_M),
            KeyCode::Char('6') if key_event.is_press() => self.place(MID_R),
            KeyCode::Char('7') if key_event.is_press() => self.place(TOP_L),
            KeyCode::Char('8') if key_event.is_press() => self.place(TOP_M),
            KeyCode::Char('9') if key_event.is_press() => self.place(TOP_R),
            _ => {}
        }
    }

    fn place(&mut self, position: u16) {
        let GameState::Playing(player) = &self.game_state else {
            return;
        };

        if !(self.bitboard_circle & position == 0 && self.bitboard_cross & position == 0) {
            return;
        }

        match player {
            Player::Circle => {
                self.bitboard_circle += position;
                self.game_state = Playing(Player::Cross)
            }
            Player::Cross => {
                self.bitboard_cross += position;
                self.game_state = Playing(Player::Circle)
            }
        }
    }
}
