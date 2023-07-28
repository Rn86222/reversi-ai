use crate::util::util::*;
use std::hash::{Hash, Hasher};
pub const BLACK: bool = true;
pub const WHITE: bool = false;
pub const BLACK_STONE: &str = "\x1b[31mo\x1b[0m";
pub const WHITE_STONE: &str = "\x1b[34mx\x1b[0m";
pub const CELL: u32 = 8;
pub const CORNER_BIT: u64 = 1 | 1 << 7 | 1 << 56 | 1 << 63;
pub const WALL_BIT: u64 = 1 << 1
    // | 1 << 2
    | 1 << 3
    | 1 << 4
    | 1 << 5
    // | 1 << 6
    // | 1 << 8
    // | 1 << 15
    | 1 << 16
    | 1 << 23
    | 1 << 24
    | 1 << 31
    | 1 << 32
    | 1 << 39
    | 1 << 40
    | 1 << 47
    | 1 << 48
    | 1 << 55
    // | 1 << 57
    | 1 << 58
    | 1 << 59
    | 1 << 60
    | 1 << 61;
// | 1 << 62;

#[derive(Clone, Copy)]
pub struct Board {
    pub black_board: u64,
    pub white_board: u64,
    pub turn: bool,
    pub no_legal_command: i32,
    pub value: i32,
    pub before_pos: u64,
}

impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.black_board.hash(state);
        self.white_board.hash(state);
        self.turn.hash(state);
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.black_board == other.black_board
            && self.white_board == other.white_board
            && self.turn == other.turn
    }
}

impl Eq for Board {}

enum LineDirection {
    Vertical,
    Horizontal,
    Diagonal1,
    Diagonal2,
}

impl LineDirection {
    #[inline]
    const fn consts(self) -> (u64, u32) {
        match self {
            Self::Vertical => (0x00ffffffffffff00, 8),
            Self::Horizontal => (0x7e7e7e7e7e7e7e7e, 1),
            Self::Diagonal1 => (0x007e7e7e7e7e7e00, 9),
            Self::Diagonal2 => (0x007e7e7e7e7e7e00, 7),
        }
    }
}

macro_rules! line {
    ($start:expr, $data:expr, $shift:ident, $n:expr) => {{
        let mut result = $data & $shift($start, $n);
        result |= $data & $shift(result, $n);
        result |= $data & $shift(result, $n);
        result |= $data & $shift(result, $n);
        result |= $data & $shift(result, $n);
        result |= $data & $shift(result, $n);
        result
    }};
}

#[inline]
const fn shl(a: u64, b: u32) -> u64 {
    a << b
}

#[inline]
const fn shr(a: u64, b: u32) -> u64 {
    a >> b
}

#[inline]
pub const fn legal(board: Board) -> u64 {
    #[inline]
    const fn calc(board: Board, direction: LineDirection) -> u64 {
        let consts = direction.consts();
        if board.turn {
            let mask = board.white_board & consts.0;
            shl(line!(board.black_board, mask, shl, consts.1), consts.1)
                | shr(line!(board.black_board, mask, shr, consts.1), consts.1)
        } else {
            let mask = board.black_board & consts.0;
            shl(line!(board.white_board, mask, shl, consts.1), consts.1)
                | shr(line!(board.white_board, mask, shr, consts.1), consts.1)
        }
    }

    let mut result = 0;
    result |= calc(board, LineDirection::Vertical);
    result |= calc(board, LineDirection::Horizontal);
    result |= calc(board, LineDirection::Diagonal1);
    result |= calc(board, LineDirection::Diagonal2);
    result & !(board.black_board | board.white_board)
}

#[inline]
const fn reverse(board: Board, pos: u64) -> u64 {
    #[inline]
    const fn calc(board: Board, pos: u64, direction: LineDirection) -> u64 {
        let consts = direction.consts();
        if board.turn {
            let mask = board.white_board & consts.0;
            (line!(pos, mask, shl, consts.1) & line!(board.black_board, mask, shr, consts.1))
                | (line!(pos, mask, shr, consts.1) & line!(board.black_board, mask, shl, consts.1))
        } else {
            let mask = board.black_board & consts.0;
            (line!(pos, mask, shl, consts.1) & line!(board.white_board, mask, shr, consts.1))
                | (line!(pos, mask, shr, consts.1) & line!(board.white_board, mask, shl, consts.1))
        }
    }

    let mut result = 0;
    result |= calc(board, pos, LineDirection::Vertical);
    result |= calc(board, pos, LineDirection::Horizontal);
    result |= calc(board, pos, LineDirection::Diagonal1);
    result |= calc(board, pos, LineDirection::Diagonal2);
    result
}

#[inline]
pub const fn rotate180_pos(pos: u64) -> u64 {
    pos.reverse_bits()
}

#[inline]
pub const fn flip_diagonal_pos(pos: u64) -> u64 {
    macro_rules! calc {
        ($r:ident, $m:expr, $n:expr) => {
            let mask = $m & ($r ^ ($r << $n));
            $r ^= mask ^ (mask >> $n);
        };
    }

    let mut result = pos;
    calc!(result, 0x0f0f0f0f00000000, 28);
    calc!(result, 0x3333000033330000, 14);
    calc!(result, 0x5500550055005500, 07);
    result
}

pub fn init_board(board: &mut Board) {
    board.black_board = 1 << 28 | 1 << 35;
    board.white_board = 1 << 27 | 1 << 36;
    board.turn = BLACK;
    board.no_legal_command = 0;
    board.value = 0;
    board.before_pos = 0;
}

pub fn print_board(board: &Board) {
    let result = legal_poss(board).iter().fold(0, |acc, &x| acc | x);
    print!("    A   B   C   D   E   F   G   H");
    let line = "+---+---+---+---+---+---+---+---+";
    for i in 0..64 {
        if i % CELL == 0 {
            print!("\n  {}\n", line);
            print!("{} ", i / CELL + 1);
        }
        print!("|");
        if (board.black_board >> i) & 1 as u64 == 1 {
            print!(" {} ", BLACK_STONE);
        } else if (board.white_board >> i) & 1 as u64 == 1 {
            print!(" {} ", WHITE_STONE);
        } else if (result >> i) & 1 as u64 == 1 {
            print!("[ ]");
        } else {
            print!("   ")
        }
        if i % CELL == 7 {
            print!("|");
        }
    }
    print!("\n  {}\n", line);
    print!(
        "turn: {}\n",
        if board.turn { BLACK_STONE } else { WHITE_STONE }
    );
}

pub fn board_state(board: &Board) -> i32 {
    let black_num = board.black_board.count_ones();
    let white_num = board.white_board.count_ones();
    if black_num + white_num < CELL * CELL && board.no_legal_command < 2 {
        0
    } else if black_num > white_num {
        1
    } else if black_num < white_num {
        2
    } else {
        3
    }
}

pub fn is_legal_pos(board: &Board, pos: &u64) -> bool {
    let legal_pos = legal(*board);
    if (legal_pos & *pos) == 0 {
        false
    } else {
        true
    }
}

pub fn legal_poss(board: &Board) -> Vec<u64> {
    let mut legal_poss_vec: Vec<u64> = Vec::new();
    for i in 0..64 {
        let pos: u64 = 1 << i;
        if is_legal_pos(board, &pos) {
            legal_poss_vec.push(pos);
        }
    }
    legal_poss_vec
}

fn flip(board: &mut Board, pos: &u64) -> Board {
    let reversed_pos = reverse(*board, *pos);
    let mut new_board = *board;
    if new_board.turn {
        new_board.black_board |= reversed_pos;
        new_board.white_board &= !reversed_pos;
    } else {
        new_board.white_board |= reversed_pos;
        new_board.black_board &= !reversed_pos;
    }
    new_board
}

pub fn execute_cmd(board: &mut Board, cmd: String) -> Board {
    execute_pos(board, cmd_to_pos(cmd))
}

pub fn execute_lower_cmd(board: &mut Board, cmd: String) -> Board {
    execute_pos(board, lower_cmd_to_pos(cmd))
}

pub fn execute_pos(board: &mut Board, pos: u64) -> Board {
    if pos == 0 || !is_legal_pos(board, &pos) {
        println!("{}: illegal command", pos_to_cmd(&pos));
        return *board;
    }
    if board.turn {
        board.black_board |= pos;
    } else {
        board.white_board |= pos;
    }
    board.no_legal_command = 0;
    let mut new_board = flip(board, &pos);
    new_board.turn = !new_board.turn;
    new_board
}

#[inline]
pub fn msb(pos: u64) -> u64 {
    let mut current_pos = pos;
    current_pos |= current_pos >> 1;
    current_pos |= current_pos >> 2;
    current_pos |= current_pos >> 4;
    current_pos |= current_pos >> 8;
    current_pos |= current_pos >> 16;
    current_pos |= current_pos >> 32;
    current_pos &= !(current_pos >> 1);
    current_pos
}
