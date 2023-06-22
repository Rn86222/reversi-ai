use std::env;
use std::io;
mod ai;
mod reversi;
mod util;
use ai::ai::*;
use reversi::reversi::*;
use std::time::Duration;
use std::time::Instant;
use util::util::*;

fn main() {
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

    let player_turn;

    let args: Vec<String> = env::args().collect();

    let depth = 7;

    let mut black_duration_sum = Duration::from_secs(0);
    let mut white_duration_sum = Duration::from_secs(0);

    if args.len() == 3 {
        while board_state(&board) == 0 {
            if board.turn {
                let (pos, duration) = ai_pos(&mut board, depth, args[1].clone());
                if pos == 0 {
                    board.no_legal_command += 1;
                    println!("no legal command, skip");
                    board.turn = !board.turn;
                } else {
                    black_duration_sum += duration;
                    println!("{}", pos_to_cmd(&pos));
                    board = execute_cmd(&mut board, pos_to_cmd(&pos));
                }
            } else {
                let (pos, duration) = ai_pos(&mut board, 10, args[2].clone());
                if pos == 0 {
                    board.no_legal_command += 1;
                    println!("no legal command, skip");
                    board.turn = !board.turn;
                } else {
                    white_duration_sum += duration;
                    println!("{}", pos_to_cmd(&pos));
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
        assert_eq!(args.len(), 4);
        player_turn = if args[2] == "s" { BLACK } else { WHITE };
        while board_state(&board) == 0 {
            if board.turn == player_turn {
                let start_time = Instant::now();
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
                        let duration = start_time.elapsed();
                        if player_turn {
                            black_duration_sum += duration;
                        } else {
                            white_duration_sum += duration;
                        }
                        board = execute_cmd(&mut board, pos_to_cmd(&pos));
                    } else {
                        println!("illegal command");
                        continue;
                    }
                }
            } else {
                let (pos, duration) = ai_pos(&mut board, depth, args[1].clone());
                if pos == 0 {
                    board.no_legal_command += 1;
                    println!("no legal command, skip");
                    board.turn = !board.turn;
                } else {
                    if player_turn {
                        white_duration_sum += duration;
                    } else {
                        black_duration_sum += duration;
                    }
                    println!("{}", pos_to_cmd(&pos));
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
    println!(
        "Usage time  {}: {:.2?}  {}: {:.2?}",
        BLACK_STONE, black_duration_sum, WHITE_STONE, white_duration_sum
    );
}
