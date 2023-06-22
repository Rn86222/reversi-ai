use std::collections::HashMap;
use std::env;
use std::io;
mod ai;
mod reversi;
mod util;
use ai::ai::*;
use reversi::reversi::*;
use util::util::*;

fn main() {
    let mut transpose_table: HashMap<Board, i32> = HashMap::new();
    let mut former_transpose_table: HashMap<Board, i32> = HashMap::new();
    let mut transpose_table_upper: HashMap<Board, i32> = HashMap::new();
    let mut former_transpose_table_upper: HashMap<Board, i32> = HashMap::new();
    let mut transpose_table_lower: HashMap<Board, i32> = HashMap::new();
    let mut former_transpose_table_lower: HashMap<Board, i32> = HashMap::new();
    let mut board: Board = Board {
        black_board: 0,
        white_board: 0,
        turn: BLACK,
        no_legal_command: 0,
        value: 0,
        before_pos: 0,
    };

    init_board(&mut board);
    print_board(&board);
    let mut rng = rand::thread_rng();

    let player_turn = BLACK;

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "0" {
        while board_state(&board) == 0 {
            if board.turn == player_turn {
                let pos = random_pos(&board, &mut rng);
                // let pos = alpha_beta_pos(&board, &mut rng, 7);
                // let pos = nega_alpha_transpose_pos(
                //     &board,
                //     7,
                //     &mut transpose_table,
                //     &mut former_transpose_table,
                // );
                // let pos = nega_scout_transpose_pos(
                //     &board,
                //     8,
                //     &mut transpose_table_upper,
                //     &mut transpose_table_lower,
                //     &mut former_transpose_table_upper,
                //     &mut former_transpose_table_lower,
                // );
                if pos == 0 {
                    board.no_legal_command += 1;
                    println!("no legal command, skip");
                    board.turn = !board.turn;
                } else {
                    board = execute_cmd(&mut board, pos_to_cmd(&pos));
                }
            } else {
                // let pos = random_pos(&board, &mut rng);
                let pos = alpha_beta_pos(&board, &mut rng, 7);
                // let pos = nega_alpha_transpose_pos(
                //     &board,
                //     7,
                //     &mut transpose_table,
                //     &mut former_transpose_table,
                // );
                // let pos = nega_scout_transpose_pos(
                //     &board,
                //     8,
                //     &mut transpose_table_upper,
                //     &mut transpose_table_lower,
                //     &mut former_transpose_table_upper,
                //     &mut former_transpose_table_lower,
                // );
                if pos == 0 {
                    board.no_legal_command += 1;
                    println!("no legal command, skip");
                    board.turn = !board.turn;
                } else {
                    board = execute_cmd(&mut board, pos_to_cmd(&pos));
                }
            }
            print_board(&board);
            println!(
                "{}: {}  {}: {}",
                BLACK_STONE,
                board.black_board.count_ones(),
                WHITE_STONE,
                board.white_board.count_ones()
            );
        }
    } else {
        while board_state(&board) == 0 {
            if board.turn == player_turn {
                if legal_poss(&board).len() == 0 {
                    board.no_legal_command += 1;
                    println!("no legal command, skip");
                    board.turn = !board.turn;
                } else {
                    println!("wait command...");
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                    let pos = cmd_to_pos(input);
                    if pos == 0 {
                        continue;
                    }
                    if is_legal_pos(&board, &pos) {
                        board = execute_cmd(&mut board, pos_to_cmd(&pos));
                    } else {
                        println!("illegal command");
                        continue;
                    }
                }
            } else {
                // let pos = random_pos(&board, &mut rng);
                let pos = alpha_beta_pos(&board, &mut rng, 7);
                // let pos = nega_alpha_transpose_pos(
                //     &board,
                //     7,
                //     &mut transpose_table,
                //     &mut former_transpose_table,
                // );
                // let pos = nega_scout_transpose_pos(
                //     &board,
                //     9,
                //     &mut transpose_table_upper,
                //     &mut transpose_table_lower,
                //     &mut former_transpose_table_upper,
                //     &mut former_transpose_table_lower,
                // );
                if pos == 0 {
                    board.no_legal_command += 1;
                    println!("no legal command, skip");
                    board.turn = !board.turn;
                } else {
                    board = execute_cmd(&mut board, pos_to_cmd(&pos));
                }
            }
            print_board(&board);
            println!(
                "{}: {}  {}: {}",
                BLACK_STONE,
                board.black_board.count_ones(),
                WHITE_STONE,
                board.white_board.count_ones()
            );
        }
    }
    if board_state(&board) == 1 {
        println!("{} win!", BLACK_STONE);
    } else if board_state(&board) == 2 {
        println!("{} win!", WHITE_STONE);
    } else {
        println!("draw");
    }
}
