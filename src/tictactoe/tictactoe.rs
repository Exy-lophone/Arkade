use crate::tictactoe::constant::{self, *};
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

#[derive(Clone, Copy)]
enum Player {
    Cross,
    Circle,
}

#[derive(Clone, Copy)]
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
    history: Vec<(Player, BoardPosition, GameState)>,
}

impl TicTacToe {
    pub fn new() -> Self {
        TicTacToe {
            exit: false,
            bitboard_circle: 0,
            bitboard_cross: 0,
            event_handler: EventHandler::new(),
            game_state: GameState::Playing(Player::Cross),
            history: Vec::with_capacity(9),
        }
    }

    pub fn _from(bitboard_cross: u16, bitboard_circle: u16) -> Self {
        TicTacToe {
            exit: false,
            bitboard_circle,
            bitboard_cross,
            event_handler: EventHandler::new(),
            game_state: GameState::Playing(Player::Cross),
            history: Vec::with_capacity(9),
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
        let mut lines: Vec<Line> = Vec::with_capacity(9);
        for plays in &self.history {
            let mut str = String::new();
            match plays.0 {
                Player::Cross => str.push_str("X => "),
                Player::Circle => str.push_str("O => "),
            }
            match plays.1 {
                BoardPosition::TOP_L => str.push_str("7"),
                BoardPosition::TOP_M => str.push_str("8"),
                BoardPosition::TOP_R => str.push_str("9"),
                BoardPosition::MID_L => str.push_str("4"),
                BoardPosition::MID_M => str.push_str("5"),
                BoardPosition::MID_R => str.push_str("6"),
                BoardPosition::BOT_L => str.push_str("1"),
                BoardPosition::BOT_M => str.push_str("2"),
                BoardPosition::BOT_R => str.push_str("3"),
            }
            match plays.2 {
                GameState::Draw => str.push_str(" | Draw !"),
                GameState::Win(player) => match player {
                    Player::Cross => str.push_str(" | X WIN !"),
                    Player::Circle => str.push_str(" | O WIN !"),
                },
                _ => {}
            }
            lines.push(Line::from(str));
        }

        frame.render_widget(
            Paragraph::new(lines).block(Block::new().title(" <History> ").borders(Borders::ALL)),
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
            KeyCode::Char('1') if key_event.is_press() => self.place(BoardPosition::BOT_L),
            KeyCode::Char('2') if key_event.is_press() => self.place(BoardPosition::BOT_M),
            KeyCode::Char('3') if key_event.is_press() => self.place(BoardPosition::BOT_R),
            KeyCode::Char('4') if key_event.is_press() => self.place(BoardPosition::MID_L),
            KeyCode::Char('5') if key_event.is_press() => self.place(BoardPosition::MID_M),
            KeyCode::Char('6') if key_event.is_press() => self.place(BoardPosition::MID_R),
            KeyCode::Char('7') if key_event.is_press() => self.place(BoardPosition::TOP_L),
            KeyCode::Char('8') if key_event.is_press() => self.place(BoardPosition::TOP_M),
            KeyCode::Char('9') if key_event.is_press() => self.place(BoardPosition::TOP_R),
            KeyCode::Char('r') if key_event.is_press() => self.restart(),
            _ => {}
        }
    }

    fn restart(&mut self) {
        self.bitboard_circle = 0;
        self.bitboard_cross = 0;
        self.game_state = GameState::Playing(Player::Cross);
        self.history.clear();
    }

    fn place(&mut self, board_position: BoardPosition) {
        let GameState::Playing(player) = self.game_state else {
            return;
        };
        let position = board_position as u16;

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

        //Check win
        for win in constant::WIN_STATES {
            if self.bitboard_circle & win == win {
                self.game_state = GameState::Win(Player::Circle);
                self.history.push((player, board_position, self.game_state));
                return;
            }
            if self.bitboard_cross & win == win {
                self.game_state = GameState::Win(Player::Cross);
                self.history.push((player, board_position, self.game_state));
                return;
            }
        }

        //Check draw
        if self.bitboard_circle + self.bitboard_cross == 0b111_111_111 {
            self.game_state = GameState::Draw;
        }

        self.history.push((player, board_position, self.game_state));
    }
}
