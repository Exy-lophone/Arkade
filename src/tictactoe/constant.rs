//Bitboard cell representation
pub const TOP_L: u16 = 0b100_000_000;
pub const TOP_M: u16 = 0b010_000_000;
pub const TOP_R: u16 = 0b001_000_000;
pub const MID_L: u16 = 0b000_100_000;
pub const MID_M: u16 = 0b000_010_000;
pub const MID_R: u16 = 0b000_001_000;
pub const BOT_L: u16 = 0b000_000_100;
pub const BOT_M: u16 = 0b000_000_010;
pub const BOT_R: u16 = 0b000_000_001;

//Wining conditions
pub const WIN_STATES: [u16; 8] = [
    0b111_000_000,
    0b000_111_000,
    0b000_000_111,
    0b100_100_100,
    0b010_010_010,
    0b001_001_001,
    0b001_010_100,
    0b100_010_001,
];

//Game assets
pub const CROSS_LINE_0: &str = "██      ██";
pub const CROSS_LINE_1: &str = "  ██  ██  ";
pub const CROSS_LINE_2: &str = "    ██    ";
pub const CIRCLE_LINE_0: &str = "  ██████  ";
pub const CIRCLE_LINE_1: &str = "██      ██";
pub const SEP_H_LINE: &str = "𜴳𜴳𜴳𜴳𜴳𜴳𜴳𜴳𜴳𜴳𜴳█𜴳𜴳𜴳𜴳𜴳𜴳𜴳𜴳𜴳𜴳𜴳𜴳█𜴳𜴳𜴳𜴳𜴳𜴳𜴳𜴳𜴳𜴳𜴳";
pub const SEP_V_LINE: &str = "█";

//Board and cell dimension
pub const BOARD_DIM: (u16, u16) = (36, 17);
pub const CELL_DIM: (u16, u16) = (10, 5);
