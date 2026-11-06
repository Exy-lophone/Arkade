use ratatui::layout::Rect;
use ratatui::prelude;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Widget};

use crate::tictactoe::constant::*;

enum CellType {
    Cross,
    Circle,
    Empty,
}

pub struct Board {
    bitboard_cross: u16,
    bitboard_circle: u16,
}

impl Board {
    pub fn new(bitboard_cross: u16, bitboard_circle: u16) -> Self {
        Board {
            bitboard_cross,
            bitboard_circle,
        }
    }

    fn get_cell_type(&self, pos: BoardPosition) -> CellType {
        let pos = pos as u16;
        if self.bitboard_cross & pos == pos {
            CellType::Cross
        } else if self.bitboard_circle & pos == pos {
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
        //Render block
        Block::new()
            .borders(Borders::ALL)
            .title("<Tic-Tac-Toe>")
            .render(area, buf);

        //Calculate position of every cells
        let top = area.top() + (area.bottom() / 2) - (BOARD_DIM.1 / 2);
        let left = area.left() + (area.right() / 2) - (BOARD_DIM.0 / 2);
        let cell_positions: [(u16, u16, BoardPosition); 9] = [
            (left, top, BoardPosition::TOP_L),
            (left + CELL_DIM.0 + 3, top, BoardPosition::TOP_M),
            (left + (CELL_DIM.0 << 1) + 6, top, BoardPosition::TOP_R),
            (left, top + CELL_DIM.1 + 1, BoardPosition::MID_L),
            (
                left + CELL_DIM.0 + 3,
                top + CELL_DIM.1 + 1,
                BoardPosition::MID_M,
            ),
            (
                left + (CELL_DIM.0 << 1) + 6,
                top + CELL_DIM.1 + 1,
                BoardPosition::MID_R,
            ),
            (left, top + (CELL_DIM.1 << 1) + 2, BoardPosition::BOT_L),
            (
                left + CELL_DIM.0 + 3,
                top + (CELL_DIM.1 << 1) + 2,
                BoardPosition::BOT_M,
            ),
            (
                left + (CELL_DIM.0 << 1) + 6,
                top + (CELL_DIM.1 << 1) + 2,
                BoardPosition::BOT_R,
            ),
        ];

        //Draw the full board
        self.draw_v_separator(left + CELL_DIM.0 + 1, top, buf);
        self.draw_v_separator(left + (CELL_DIM.0 << 1) + 4, top, buf);
        self.draw_h_separator(left, top + CELL_DIM.1, buf);
        self.draw_h_separator(left, top + (CELL_DIM.1 << 1) + 1, buf);
        for (x, y, pos) in cell_positions {
            match self.get_cell_type(pos) {
                CellType::Cross => self.draw_cross(x, y, buf),
                CellType::Circle => self.draw_circle(x, y, buf),
                CellType::Empty => {}
            }
        }
    }
}
