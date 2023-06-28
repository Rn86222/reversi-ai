use std::env;
use std::io;
use std::net::SocketAddr;
use std::net::ToSocketAddrs;
mod ai;
mod client;
mod reversi;
mod util;
use ai::ai::*;
use client::client::{ClientState::*, Command::*, *};
use reversi::reversi::*;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
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

    let mut player_turn;
    let mut ai_turn;

    let args: Vec<String> = env::args().collect();
    let argc = args.len();

    let depth = 9;

    let mut client_state = CardWaiting;

    if argc == 1 || args[1] != String::from("-debug") {
        let server_address;
        let name;
        if argc == 1 {
            server_address = String::from("127.0.0.1:8080");
            name = String::from("Rn86222");
        } else {
            assert!(argc > 6);
            let host = args[2].clone() + &String::from(":") + &args[4];
            let server_addresses: Vec<SocketAddr> = host.to_socket_addrs().unwrap().collect();
            assert_ne!(server_addresses.len(), 0);
            server_address = server_addresses[0].to_string();
            name = args[6].to_string();
        }
        match TcpStream::connect(server_address) {
            Ok(mut stream) => {
                println!("Connected to server.");
                let mut reader = BufReader::new(stream.try_clone().unwrap());
                let mut received_mes = String::new();

                let request = String::from("OPEN ") + &name + &String::from("\n");
                let request_mes = request.as_bytes();
                stream.write_all(request_mes).unwrap();
                print!("Sent: {}", request);

                'main: loop {
                    match client_state {
                        CardWaiting => {
                            received_mes.clear();
                            reader.read_line(&mut received_mes).unwrap();
                            print!("Recieved: {}", received_mes);

                            match mes_to_command(&received_mes) {
                                Start(wb, opponent_name, _) => {
                                    ai_turn = if wb == String::from("BLACK") {
                                        BLACK
                                    } else {
                                        WHITE
                                    };
                                    player_turn = !ai_turn;
                                    println!("Opponent name: {}", opponent_name);
                                    println!("ai_turn {}  oppoennt_turn {}", ai_turn, player_turn);
                                    client_state = if ai_turn == BLACK {
                                        MyTurn
                                    } else {
                                        OpponentTurn
                                    };
                                }
                                Bye(stat) => {
                                    println!("Stat: {}", stat);
                                    println!("Bybye.");
                                    client_state = Ended;
                                }
                                _ => {
                                    println!("Unexpected message (waiting card): {}", received_mes);
                                    panic!();
                                }
                            }
                        }
                        MyTurn => {
                            let (pos, _) = ai_pos(&mut board, depth, String::from("ns"));
                            if pos == 0 {
                                println!("No legal command");
                                board.no_legal_command += 1;
                                board.turn = !board.turn;

                                let request = String::from("MOVE PASS\n");
                                let request_mes = request.as_bytes();
                                stream.write_all(request_mes).unwrap();
                                print!("Sent: {}", request);
                            } else {
                                println!("{}", pos_to_cmd(&pos));
                                let request_string =
                                    String::from("MOVE ") + &pos_to_cmd(&pos) + &String::from("\n");
                                let request = request_string.as_bytes();
                                stream.write_all(request).unwrap();
                                board = execute_cmd(&mut board, pos_to_cmd(&pos));
                            }
                            client_state = AckWaiting;
                        }
                        AckWaiting => {
                            received_mes.clear();
                            reader.read_line(&mut received_mes).unwrap();
                            print!("Recieved: {}", received_mes);

                            match mes_to_command(&received_mes) {
                                Ack(time) => {
                                    println!("Remaining time: {}", time);
                                    client_state = OpponentTurn;
                                }
                                End(wl, n, m, reason) => {
                                    println!("{} : {}", n, m);
                                    if wl == String::from("WIN") {
                                        println!("Win!");
                                    } else if wl == String::from("LOSE") {
                                        println!("Lose...");
                                    } else {
                                        println!("Tie.");
                                    }
                                    println!("Ended reason: {}", reason);
                                    print_board(&board);
                                    init_board(&mut board);
                                    client_state = CardWaiting;
                                }
                                _ => {
                                    println!("Unexpected message (waiting ack): {}", received_mes);
                                    panic!();
                                }
                            }
                        }
                        OpponentTurn => {
                            received_mes.clear();
                            reader.read_line(&mut received_mes).unwrap();
                            print!("Recieved: {}", received_mes);

                            match mes_to_command(&received_mes) {
                                Move(move_cmd) => {
                                    if move_cmd == String::from("PASS") {
                                        board.no_legal_command += 1;
                                        board.turn = !board.turn;
                                    } else {
                                        let pos = cmd_to_pos(move_cmd);
                                        if pos == 0 {
                                            panic!();
                                        }
                                        if is_legal_pos(&board, &pos) {
                                            board = execute_cmd(&mut board, pos_to_cmd(&pos));
                                        } else {
                                            println!("illegal command");
                                            panic!();
                                        }
                                    }
                                    client_state = MyTurn;
                                }
                                End(wl, n, m, reason) => {
                                    println!("{} : {}", n, m);
                                    if wl == String::from("WIN") {
                                        println!("Win!");
                                    } else if wl == String::from("LOSE") {
                                        println!("Lose...");
                                    } else {
                                        println!("Tie.");
                                    }
                                    println!("Ended reason: {}", reason);
                                    print_board(&board);
                                    init_board(&mut board);
                                    client_state = CardWaiting;
                                }
                                _ => {
                                    println!(
                                        "Unexpected message (opponent turn): {}",
                                        received_mes
                                    );
                                    panic!();
                                }
                            }
                        }
                        Ended => {
                            println!("Ended");
                            break 'main;
                        }
                    }
                }
            }
            Err(e) => {
                println!("Connection error: {}", e);
            }
        }
    } else {
        let mut black_duration_sum = Duration::from_secs(0);
        let mut white_duration_sum = Duration::from_secs(0);
        if argc == 4 {
            while board_state(&board) == 0 {
                if board.turn {
                    let (pos, duration) = ai_pos(&mut board, depth, args[2].clone());
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
                    let (pos, duration) = ai_pos(&mut board, 10, args[3].clone());
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
            assert_eq!(argc, 5);
            player_turn = if args[3] == "s" { BLACK } else { WHITE };
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
                    let (pos, duration) = ai_pos(&mut board, depth, args[4].clone());
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
}
