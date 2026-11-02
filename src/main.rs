use std::io;
use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::{self, Constraint, Direction, Layout, Position, Rect};
use ratatui::style::{Color, Style};
use ratatui::symbols::shade::EMPTY;
use ratatui::widgets::{Block, Borders, Paragraph, Widget};
use ratatui::{DefaultTerminal, prelude};

const TOP_L: u16 = 0b100_000_000;
const TOP_M: u16 = 0b010_000_000;
const TOP_R: u16 = 0b001_000_000;
const MID_L: u16 = 0b000_100_000;
const MID_M: u16 = 0b000_010_000;
const MID_R: u16 = 0b000_001_000;
const BOT_L: u16 = 0b000_000_100;
const BOT_M: u16 = 0b000_000_010;
const BOT_R: u16 = 0b000_000_001;

const WIN_TOP_ROW: u16 = 0b111_000_000;
const WIN_MID_ROW: u16 = 0b000_111_000;
const WIN_BOT_ROW: u16 = 0b000_000_111;
const WIN_L_COL: u16 = 0b100_100_100;
const WIN_M_COL: u16 = 0b010_010_010;
const WIN_R_COL: u16 = 0b001_001_001;
const WIN_DIAG_BOTTOP: u16 = 0b001_010_100;
const WIN_DIAG_TOPBOT: u16 = 0b100_010_001;

const CROSS_LINE_0: &str = "██      ██";
const CROSS_LINE_1: &str = "  ██  ██  ";
const CROSS_LINE_2: &str = "    ██    ";

const CIRCLE_LINE_0: &str = "  ██████  ";
const CIRCLE_LINE_1: &str = "██      ██";

const SEP_H_LINE: &str = "𜴳𜴳𜴳𜴳𜴳𜴳𜴳𜴳𜴳𜴳𜴳█𜴳𜴳𜴳𜴳𜴳𜴳𜴳𜴳𜴳𜴳𜴳𜴳█𜴳𜴳𜴳𜴳𜴳𜴳𜴳𜴳𜴳𜴳𜴳";
const SEP_V_LINE: &str = "█";

const SPRITE_DIM: (u16, u16) = (10, 5);

#[derive(PartialEq)]
enum CellType {
    Cross,
    Circle,
    Empty,
}

struct TicTacToe {
    running: bool,
    bitboard_cross: u16,
    bitboard_circle: u16,
}

struct Board {
    bitboard_cross: u16,
    bitboard_circle: u16,
}

impl Board {
    fn new(bitboard_cross: u16, bitboard_circle: u16) -> Self {
        Board {
            bitboard_cross,
            bitboard_circle,
        }
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

    fn draw_cross(&self, x: u16, y: u16, buf: &mut prelude::Buffer) {
        buf.set_string(x, y, CROSS_LINE_0, Style::new().fg(Color::Red));
        buf.set_string(x, y + 1, CROSS_LINE_1, Style::new().fg(Color::Red));
        buf.set_string(x, y + 2, CROSS_LINE_2, Style::new().fg(Color::Red));
        buf.set_string(x, y + 3, CROSS_LINE_1, Style::new().fg(Color::Red));
        buf.set_string(x, y + 4, CROSS_LINE_0, Style::new().fg(Color::Red));
    }

    fn draw_circle(&self, x: u16, y: u16, buf: &mut prelude::Buffer) {
        buf.set_string(x, y, CIRCLE_LINE_0, Style::new().fg(Color::Blue));
        buf.set_string(x, y + 1, CIRCLE_LINE_1, Style::new().fg(Color::Blue));
        buf.set_string(x, y + 2, CIRCLE_LINE_1, Style::new().fg(Color::Blue));
        buf.set_string(x, y + 3, CIRCLE_LINE_1, Style::new().fg(Color::Blue));
        buf.set_string(x, y + 4, CIRCLE_LINE_0, Style::new().fg(Color::Blue));
    }

    fn draw_v_separator(&self, x: u16, y: u16, buf: &mut prelude::Buffer) {
        for i in 0..17 {
            buf.set_string(x, y + i, SEP_V_LINE, Style::new().fg(Color::White));
        }
    }

    fn draw_h_separator(&self, x: u16, y: u16, buf: &mut prelude::Buffer) {
        buf.set_string(x, y, SEP_H_LINE, Style::new().fg(Color::White));
    }
}

impl Widget for Board {
    fn render(self, area: Rect, buf: &mut prelude::Buffer)
    where
        Self: Sized,
    {
        let top = area.top();
        let left = area.left();
        let cell_positions: [(u16, u16, u16); 9] = [
            (left, top, TOP_L),
            (left + SPRITE_DIM.0 + 3, top, TOP_M),
            (left + (SPRITE_DIM.0 << 1) + 6, top, TOP_R),
            (left, top + SPRITE_DIM.1 + 1, MID_L),
            (left + SPRITE_DIM.0 + 3, top + SPRITE_DIM.1 + 1, MID_M),
            (
                left + (SPRITE_DIM.0 << 1) + 6,
                top + SPRITE_DIM.1 + 1,
                MID_R,
            ),
            (left, top + (SPRITE_DIM.1 << 1) + 2, BOT_L),
            (
                left + SPRITE_DIM.0 + 3,
                top + (SPRITE_DIM.1 << 1) + 2,
                BOT_M,
            ),
            (
                left + (SPRITE_DIM.0 << 1) + 6,
                top + (SPRITE_DIM.1 << 1) + 2,
                BOT_R,
            ),
        ];

        self.draw_v_separator(left + SPRITE_DIM.0 + 1, top, buf);
        self.draw_v_separator(left + (SPRITE_DIM.0 << 1) + 4, top, buf);
        self.draw_h_separator(left, top + SPRITE_DIM.1, buf);
        self.draw_h_separator(left, top + (SPRITE_DIM.1 << 1) + 1, buf);
        for (x, y, pos) in cell_positions {
            match self.get_cell_type(pos) {
                CellType::Cross => self.draw_cross(x, y, buf),
                CellType::Circle => self.draw_circle(x, y, buf),
                CellType::Empty => {}
            }
        }
    }
}

impl TicTacToe {
    fn new() -> Self {
        TicTacToe {
            running: true,
            bitboard_circle: 0,
            bitboard_cross: 0,
        }
    }

    fn from(bitboard_cross: u16, bitboard_circle: u16) -> Self {
        TicTacToe {
            running: true,
            bitboard_circle,
            bitboard_cross,
        }
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

    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while self.running {
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
            let board = Board::new(self.bitboard_cross, self.bitboard_circle);
            terminal.draw(|frame| {
                frame.render_widget(board, frame.area());
            })?;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut tictactoe = TicTacToe::from(0b011_100_001, 0b100_011_110);
    ratatui::run(|terminal| tictactoe.run(terminal))?;
    Ok(())
}
