use crate::reversi::reversi::*;
use crate::util::util::*;
use rand::seq::SliceRandom;
use rand::{rngs::ThreadRng, Rng};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::option::Option;
use std::time::Duration;
use std::time::Instant;

const MAX_SCORE: i32 = 10000;

fn evaluate_board(board: &Board) -> i32 {
    let mut black_score: u32 = 0;
    let mut white_score: u32 = 0;

    // count
    let count = board.black_board.count_ones() + board.white_board.count_ones();

    // corner
    let corner_score;
    if count <= 48 {
        corner_score = 20;
    } else {
        corner_score = 10;
    }
    black_score += (board.black_board & CORNER_BIT).count_ones() * corner_score;
    white_score += (board.white_board & CORNER_BIT).count_ones() * corner_score;

    // alongside the walls
    let wall_score = 5;
    black_score += (board.black_board & WALL_BIT).count_ones() * wall_score;
    white_score += (board.white_board & WALL_BIT).count_ones() * wall_score;

    let legal_score;
    if count <= 48 {
        legal_score = 5;
    } else {
        legal_score = 10;
    }

    if board.turn {
        black_score as i32 - white_score as i32 + (legal(*board).count_ones() * legal_score) as i32
    } else {
        white_score as i32 - black_score as i32 + (legal(*board).count_ones() * legal_score) as i32
    }
}

pub fn random_pos(board: &Board) -> u64 {
    let legal_poss_vec: Vec<u64> = legal_poss(&board);
    let len = legal_poss_vec.len();
    let mut rng = rand::thread_rng();
    if len == 0 {
        0
    } else {
        let random_index = rng.gen_range(0..len);
        legal_poss_vec[random_index]
    }
}

fn check_end_score(board: &Board) -> Option<(i32, i32)> {
    let state = board_state(board);
    if state == 1 {
        if board.turn {
            Some((1, MAX_SCORE))
        } else {
            Some((1, -MAX_SCORE))
        }
    } else if state == 2 {
        if board.turn {
            Some((1, -MAX_SCORE))
        } else {
            Some((1, MAX_SCORE))
        }
    } else if state == 3 {
        Some((1, 0))
    } else {
        None
    }
}

fn alpha_beta(
    board: &mut Board,
    rng: &mut ThreadRng,
    mut alpha: i32,
    beta: i32,
    depth: i32,
) -> (i32, i32) {
    if let Some((count, score)) = check_end_score(board) {
        return (count, score);
    } else if depth <= 0 {
        return (1, evaluate_board(board));
    }
    let legal_poss_vec = legal_poss(board);
    if legal_poss_vec.len() == 0 {
        board.turn = !board.turn;
        board.no_legal_command += 1;
        let (count, score) = alpha_beta(board, rng, -beta, -alpha, depth);
        return (count, -score);
    }

    let mut choices: Vec<usize> = (0..legal_poss_vec.len()).collect();
    choices.shuffle(rng);
    let mut count_sum = 0;
    for i in choices {
        let mut new_board = *board;
        let pos = legal_poss_vec[i];
        new_board = execute_pos(&mut new_board, pos);
        let (count, mut score) = alpha_beta(&mut new_board, rng, -beta, -alpha, depth - 1);
        count_sum += count;
        score = -score;
        if score > alpha {
            alpha = score;
        }
        if alpha >= beta {
            return (count_sum, alpha);
        }
    }
    (count_sum, alpha)
}

pub fn alpha_beta_pos(board: &Board, depth: i32) -> u64 {
    let legal_poss_vec = legal_poss(board);
    let mut best_pos;
    let mut alpha = std::i32::MIN + 1;
    if legal_poss_vec.len() == 0 {
        return 0;
    }
    let mut rng = rand::thread_rng();
    let mut choices: Vec<usize> = (0..legal_poss_vec.len()).collect();
    choices.shuffle(&mut rng);
    best_pos = legal_poss_vec[0];
    let mut count_sum = 0;
    for i in choices {
        let mut new_board = *board;
        new_board = execute_pos(&mut new_board, legal_poss_vec[i]);
        let (count, mut score) = alpha_beta(
            &mut new_board,
            &mut rng,
            std::i32::MIN + 1,
            -alpha,
            depth - 1,
        );
        count_sum += count;
        score = -score;
        if score >= MAX_SCORE {
            println!("Complete");
            return legal_poss_vec[i];
        }
        if score > alpha {
            best_pos = legal_poss_vec[i];
            alpha = score;
        }
    }
    println!(
        "score: {} {}  searched: {}",
        alpha,
        pos_to_cmd(&best_pos),
        count_sum
    );
    best_pos
}

fn calc_move_ordering_value(board: &Board, former_transpose_table: &HashMap<Board, i32>) -> i32 {
    if let Some(v) = (*former_transpose_table).get(board) {
        let cache_hit_bonus = 20;
        cache_hit_bonus - v
    } else {
        -evaluate_board(board)
    }
}

fn nega_alpha_transpose(
    board: &mut Board,
    depth: i32,
    mut alpha: i32,
    beta: i32,
    transpose_table: &mut HashMap<Board, i32>,
) -> (i32, i32) {
    if let Some((count, score)) = check_end_score(board) {
        return (count, score);
    } else if depth <= 0 {
        return (1, evaluate_board(board));
    } else if let Some(v) = transpose_table.get(board) {
        return (1, *v);
    }
    let legal_poss_vec = legal_poss(board);
    if legal_poss_vec.len() == 0 {
        board.turn = !board.turn;
        board.no_legal_command += 1;
        let (count, score) = nega_alpha_transpose(board, depth, -beta, -alpha, transpose_table);
        return (count, -score);
    }
    let mut child_boards: Vec<Board> = Vec::new();
    for i in 0..legal_poss_vec.len() {
        let mut child_board = *board;
        child_board = execute_pos(&mut child_board, legal_poss_vec[i]);
        child_boards.push(child_board);
    }
    child_boards.sort_by(|a, b| b.value.cmp(&a.value));

    let mut searched_nodes = 0;
    for mut child in child_boards {
        let (count, mut score) =
            nega_alpha_transpose(&mut child, depth - 1, -beta, -alpha, transpose_table);
        score = -score;
        searched_nodes += count;
        if score > alpha {
            alpha = score;
        }
        if alpha >= beta {
            return (searched_nodes, alpha);
        }
    }
    transpose_table.insert(*board, alpha);
    (searched_nodes, alpha)
}

pub fn nega_alpha_transpose_pos(board: &Board, depth: i32) -> u64 {
    let start_time = Instant::now();
    let mut transpose_table: HashMap<Board, i32> = HashMap::new();
    let mut former_transpose_table: HashMap<Board, i32> = HashMap::new();
    let legal_poss_vec = legal_poss(board);
    let mut best_pos;
    if legal_poss_vec.len() == 0 {
        return 0;
    }
    let mut child_boards: Vec<Board> = Vec::new();
    for i in 0..legal_poss_vec.len() {
        let mut child_board = *board;
        child_board = execute_pos(&mut child_board, legal_poss_vec[i]);
        child_board.before_pos = legal_poss_vec[i];
        child_boards.push(child_board);
    }
    best_pos = legal_poss_vec[0];
    let start_depth = if 1 < depth - 5 { depth - 5 } else { 1 };
    let mut searched_nodes = 0;
    let mut best_score = 0;
    for search_depth in start_depth..=depth {
        if start_time.elapsed() >= Duration::from_millis(500) {
            println!("score: {}", best_score);
            return best_pos;
        }
        let mut alpha = std::i32::MIN + 1;
        let beta = -alpha;
        if legal_poss_vec.len() >= 2 {
            child_boards = child_boards
                .iter()
                .map(|b: &Board| {
                    let mut new_b: Board = *b;
                    new_b.value = calc_move_ordering_value(&new_b, &mut former_transpose_table);
                    new_b
                })
                .collect();
            child_boards.sort_by(|a, b| b.value.cmp(&a.value));
        }
        for mut child in child_boards.clone() {
            let (count, mut score) = nega_alpha_transpose(
                &mut child,
                search_depth - 1,
                -beta,
                -alpha,
                &mut transpose_table,
            );
            score = -score;
            searched_nodes += count;
            if score >= MAX_SCORE {
                println!("Complete");
                return child.before_pos;
            }
            if score > alpha {
                best_pos = child.before_pos;
                alpha = score;
                best_score = alpha;
            }
        }
        println!(
            "searched_depth: {}  command: {}  visited nodes: {}",
            search_depth,
            pos_to_cmd(&best_pos),
            searched_nodes
        );
        former_transpose_table = transpose_table.clone();
        transpose_table.clear();
    }
    println!("score: {}", best_score);
    best_pos
}

fn calc_move_ordering_value_nega_scout(
    board: &Board,
    former_transpose_table_upper: &HashMap<Board, i32>,
    former_transpose_table_lower: &HashMap<Board, i32>,
) -> i32 {
    let cache_hit_bonus = 20;
    if let Some(v) = (*former_transpose_table_upper).get(board) {
        cache_hit_bonus - v
    } else if let Some(v) = (*former_transpose_table_lower).get(board) {
        cache_hit_bonus - v
    } else {
        -evaluate_board(board)
    }
}

fn nega_scout_transpose(
    board: &mut Board,
    depth: i32,
    mut alpha: i32,
    mut beta: i32,
    transpose_table_upper: &mut HashMap<Board, i32>,
    transpose_table_lower: &mut HashMap<Board, i32>,
    former_transpose_table_upper: &mut HashMap<Board, i32>,
    former_transpose_table_lower: &mut HashMap<Board, i32>,
    remaining_time: Duration,
) -> (i32, i32) {
    let start_time = Instant::now();
    if let Some((count, score)) = check_end_score(board) {
        return (count, score);
    } else if depth <= 0 {
        return (1, evaluate_board(board));
    }

    let (mut u, mut l) = (-(std::i32::MIN + 1), std::i32::MIN + 1);
    if let Some(v) = transpose_table_upper.get(board) {
        u = *v;
    }
    if let Some(v) = transpose_table_lower.get(board) {
        l = *v;
    }

    if u == l {
        return (1, u);
    }

    if l > alpha {
        alpha = l;
    }
    if u < beta {
        beta = u;
    }
    let mut legal_poss = legal(*board);
    let legal_poss_num = legal_poss.count_ones();
    if legal_poss == 0 {
        board.turn = !board.turn;
        board.no_legal_command += 1;
        let (count, score) = nega_scout_transpose(
            board,
            depth,
            -beta,
            -alpha,
            transpose_table_upper,
            transpose_table_lower,
            former_transpose_table_upper,
            former_transpose_table_lower,
            remaining_time,
        );
        return (count, -score);
    }
    let mut child_boards: Vec<Board> = Vec::new();
    for _ in 0..legal_poss_num {
        let current_pos = msb(legal_poss);

        let mut child_board = *board;
        child_board = execute_pos(&mut child_board, current_pos);
        child_boards.push(child_board);

        legal_poss &= !current_pos;
    }
    if legal_poss_num >= 2 {
        child_boards = child_boards
            .iter()
            .map(|b: &Board| {
                let mut new_b: Board = *b;
                new_b.value = calc_move_ordering_value_nega_scout(
                    &new_b,
                    former_transpose_table_upper,
                    former_transpose_table_lower,
                );
                new_b
            })
            .collect();
        child_boards.sort_by(|a, b| b.value.cmp(&a.value));
    }
    let mut searched_nodes = 0;
    let mut best_score = std::i32::MIN + 1;
    for mut child in child_boards {
        if start_time.elapsed() >= remaining_time {
            return (searched_nodes, best_score);
        }
        let (count, mut score) = nega_scout_transpose(
            &mut child,
            depth - 1,
            -beta,
            -alpha,
            transpose_table_upper,
            transpose_table_lower,
            former_transpose_table_upper,
            former_transpose_table_lower,
            Duration::from_micros(100) + remaining_time - start_time.elapsed(),
        );
        score = -score;
        searched_nodes += count;
        if score >= beta {
            if score > l {
                transpose_table_lower.insert(*board, score);
            }
            return (searched_nodes, score);
        }
        if score > alpha {
            alpha = score;
        }
        if best_score < score {
            best_score = score;
        }
    }
    if best_score < alpha {
        transpose_table_upper.insert(*board, best_score);
    } else {
        transpose_table_upper.insert(*board, best_score);
        transpose_table_lower.insert(*board, best_score);
    }
    (searched_nodes, best_score)
}

fn nega_scout(
    board: &mut Board,
    depth: i32,
    mut alpha: i32,
    mut beta: i32,
    transpose_table_upper: &mut HashMap<Board, i32>,
    transpose_table_lower: &mut HashMap<Board, i32>,
    former_transpose_table_upper: &mut HashMap<Board, i32>,
    former_transpose_table_lower: &mut HashMap<Board, i32>,
    remaining_time: Duration,
) -> (i32, i32) {
    let start_time = Instant::now();
    if let Some((count, score)) = check_end_score(board) {
        return (count, score);
    } else if depth <= 0 {
        return (1, evaluate_board(board));
    }

    let (mut u, mut l) = (-(std::i32::MIN + 1), std::i32::MIN + 1);
    if let Some(v) = transpose_table_upper.get(board) {
        u = *v;
    }
    if let Some(v) = transpose_table_lower.get(board) {
        l = *v;
    }

    if u == l {
        return (1, u);
    }

    if l > alpha {
        alpha = l;
    }
    if u < beta {
        beta = u;
    }
    let mut legal_poss = legal(*board);
    let legal_poss_num = legal_poss.count_ones();
    if legal_poss == 0 {
        board.turn = !board.turn;
        board.no_legal_command += 1;
        let (count, score) = nega_scout(
            board,
            depth,
            -beta,
            -alpha,
            transpose_table_upper,
            transpose_table_lower,
            former_transpose_table_upper,
            former_transpose_table_lower,
            remaining_time,
        );
        return (count, -score);
    }
    let mut child_boards: Vec<Board> = Vec::new();
    for _ in 0..legal_poss_num {
        let current_pos = msb(legal_poss);

        let mut child_board = *board;
        child_board = execute_pos(&mut child_board, current_pos);
        child_boards.push(child_board);

        legal_poss &= !current_pos;
    }
    if legal_poss_num >= 2 {
        child_boards = child_boards
            .iter()
            .map(|b: &Board| {
                let mut new_b: Board = *b;
                new_b.value = calc_move_ordering_value_nega_scout(
                    &new_b,
                    former_transpose_table_upper,
                    former_transpose_table_lower,
                );
                new_b
            })
            .collect();
        child_boards.sort_by(|a, b| b.value.cmp(&a.value));
    }
    let mut searched_nodes = 0;
    let (count, mut score) = nega_scout(
        &mut child_boards[0],
        depth - 1,
        -beta,
        -alpha,
        transpose_table_upper,
        transpose_table_lower,
        former_transpose_table_upper,
        former_transpose_table_lower,
        remaining_time,
    );
    score = -score;
    searched_nodes += count;
    if score >= beta {
        if score > l {
            transpose_table_lower.insert(*board, score);
        }
        return (searched_nodes, score);
    }
    if alpha < score {
        alpha = score;
    }
    let mut best_score = score;

    for mut child in &mut child_boards[1..] {
        if start_time.elapsed() >= remaining_time {
            return (searched_nodes, best_score);
        }
        let (count, mut score) = nega_scout_transpose(
            &mut child,
            depth - 1,
            -alpha - 1,
            -alpha,
            transpose_table_upper,
            transpose_table_lower,
            former_transpose_table_upper,
            former_transpose_table_lower,
            Duration::from_micros(100) + remaining_time - start_time.elapsed(),
        );
        score = -score;
        searched_nodes += count;
        if score >= beta {
            if score > l {
                transpose_table_lower.insert(*board, score);
            }
            return (searched_nodes, score);
        }
        if score > alpha {
            alpha = score;
            if start_time.elapsed() >= remaining_time {
                return (searched_nodes, best_score);
            }
            let (count, mut score) = nega_scout(
                child,
                depth - 1,
                -beta,
                -alpha,
                transpose_table_upper,
                transpose_table_lower,
                former_transpose_table_upper,
                former_transpose_table_lower,
                Duration::from_micros(100) + remaining_time - start_time.elapsed(),
            );
            score = -score;
            searched_nodes += count;
            if score >= beta {
                if score > l {
                    transpose_table_lower.insert(*board, score);
                }
                return (searched_nodes, score);
            }
        }
        if alpha < score {
            alpha = score;
        }
        if best_score < score {
            best_score = score;
        }
    }
    if best_score < alpha {
        transpose_table_upper.insert(*board, best_score);
    } else {
        transpose_table_upper.insert(*board, best_score);
        transpose_table_lower.insert(*board, best_score);
    }
    (searched_nodes, best_score)
}

pub fn nega_scout_transpose_pos(board: &Board, depth: i32, thinking_time: Duration) -> u64 {
    let start_time = Instant::now();
    let mut transpose_table_upper: HashMap<Board, i32> = HashMap::new();
    let mut former_transpose_table_upper: HashMap<Board, i32> = HashMap::new();
    let mut transpose_table_lower: HashMap<Board, i32> = HashMap::new();
    let mut former_transpose_table_lower: HashMap<Board, i32> = HashMap::new();
    let mut legal_poss = legal(*board);
    let legal_poss_num = legal_poss.count_ones();
    if legal_poss == 0 {
        return 0;
    }
    let mut child_boards: Vec<Board> = Vec::new();
    let mut best_pos = 0;
    for _ in 0..legal_poss_num {
        let current_pos = msb(legal_poss);

        best_pos = current_pos;
        let mut child_board = *board;
        child_board = execute_pos(&mut child_board, current_pos);
        child_board.before_pos = current_pos;
        child_boards.push(child_board);

        legal_poss &= !current_pos;
    }
    let start_depth = if 1 < depth - 3 { depth - 3 } else { 1 };
    let mut searched_nodes = 0;
    let mut best_score = 0;
    for search_depth in start_depth..=depth {
        if start_time.elapsed() >= thinking_time {
            println!("score: {}", best_score);
            return best_pos;
        }
        let mut alpha = std::i32::MIN + 1;
        let beta = -alpha;
        if legal_poss_num >= 2 {
            child_boards = child_boards
                .iter()
                .map(|b: &Board| {
                    let mut new_b: Board = *b;
                    new_b.value = calc_move_ordering_value_nega_scout(
                        &new_b,
                        &mut former_transpose_table_upper,
                        &mut former_transpose_table_lower,
                    );
                    new_b
                })
                .collect();
            child_boards.sort_by(|a, b| b.value.cmp(&a.value));
        }
        if start_time.elapsed() >= thinking_time {
            println!("score: {}", best_score);
            return best_pos;
        }
        let (count, mut score) = nega_scout(
            &mut child_boards[0],
            search_depth - 1,
            -beta,
            -alpha,
            &mut transpose_table_upper,
            &mut transpose_table_lower,
            &mut former_transpose_table_upper,
            &mut former_transpose_table_lower,
            Duration::from_micros(100) + thinking_time - start_time.elapsed(),
        );
        score = -score;
        searched_nodes += count;
        alpha = score;
        best_pos = child_boards[0].before_pos;
        if score >= MAX_SCORE {
            println!("Complete");
            return best_pos;
        }

        for mut child in &mut child_boards.clone()[1..] {
            // println!(
            //     "used time:  {:.2?}  thinking_time: {:.2?}",
            //     start_time.elapsed(),
            //     thinking_time
            // );
            if start_time.elapsed() >= thinking_time {
                println!("score: {}", best_score);
                return best_pos;
            }
            let (count, mut score) = nega_scout_transpose(
                &mut child,
                search_depth - 1,
                -alpha - 1,
                -alpha,
                &mut transpose_table_upper,
                &mut transpose_table_lower,
                &mut former_transpose_table_upper,
                &mut former_transpose_table_lower,
                Duration::from_micros(100) + thinking_time - start_time.elapsed(),
            );
            score = -score;
            searched_nodes += count;
            if score >= MAX_SCORE {
                println!("Complete");
                return child.before_pos;
            }
            if score > alpha {
                best_pos = child.before_pos;
                alpha = score;
                best_score = alpha;
                if start_time.elapsed() >= thinking_time {
                    println!("score: {}", best_score);
                    return best_pos;
                }
                (_, score) = nega_scout(
                    &mut child,
                    search_depth - 1,
                    -beta,
                    -alpha,
                    &mut transpose_table_upper,
                    &mut transpose_table_lower,
                    &mut former_transpose_table_upper,
                    &mut former_transpose_table_lower,
                    Duration::from_micros(100) + thinking_time - start_time.elapsed(),
                );
                score = -score;
            }
            if score > alpha {
                alpha = score;
                best_score = alpha;
            }
        }
        println!(
            "searched_depth: {}  command: {}  visited nodes: {}",
            search_depth,
            pos_to_cmd(&best_pos),
            searched_nodes
        );
        former_transpose_table_upper = transpose_table_upper.clone();
        former_transpose_table_lower = transpose_table_lower.clone();
        transpose_table_upper.clear();
        transpose_table_lower.clear();
    }
    println!("score: {}", best_score);
    best_pos
}

pub fn create_book(path: &str, book: &mut HashMap<Board, u64>) {
    match File::open(path) {
        Err(e) => {
            println!("Failed in opening file ({}).", e);
        }
        Ok(file) => {
            println!("Success in opening file.");
            let reader = BufReader::new(file);

            for line in reader.lines() {
                let line_content = line.unwrap();
                let len = line_content.len();
                let mut board: Board = Board {
                    black_board: 0,
                    white_board: 0,
                    turn: BLACK,
                    no_legal_command: 0,
                    value: 0,
                    before_pos: 0,
                };

                init_board(&mut board);
                for i in 0..(len / 2 - 1) {
                    board =
                        execute_cmd(&mut board, line_content[(2 * i)..=(2 * i + 1)].to_string());
                }
                book.insert(
                    board,
                    cmd_to_pos(line_content[(len - 2)..=(len - 1)].to_string()),
                );

                init_board(&mut board);
                for i in 0..(len / 2 - 1) {
                    let mut pos = cmd_to_pos(line_content[(2 * i)..=(2 * i + 1)].to_string());
                    pos = rotate180_pos(pos);
                    board = execute_pos(&mut board, pos);
                }
                book.insert(
                    board,
                    rotate180_pos(cmd_to_pos(line_content[(len - 2)..=(len - 1)].to_string())),
                );

                init_board(&mut board);
                for i in 0..(len / 2 - 1) {
                    let mut pos = cmd_to_pos(line_content[(2 * i)..=(2 * i + 1)].to_string());
                    pos = flip_diagonal_pos(pos);
                    board = execute_pos(&mut board, pos);
                }
                book.insert(
                    board,
                    flip_diagonal_pos(cmd_to_pos(line_content[(len - 2)..=(len - 1)].to_string())),
                );

                init_board(&mut board);
                for i in 0..(len / 2 - 1) {
                    let mut pos = cmd_to_pos(line_content[(2 * i)..=(2 * i + 1)].to_string());
                    pos = rotate180_pos(flip_diagonal_pos(pos));
                    board = execute_pos(&mut board, pos);
                }
                book.insert(
                    board,
                    rotate180_pos(flip_diagonal_pos(cmd_to_pos(
                        line_content[(len - 2)..=(len - 1)].to_string(),
                    ))),
                );
            }
        }
    }
}

pub fn ai_pos(
    board: &mut Board,
    depth: i32,
    ai_name: String,
    book: &HashMap<Board, u64>,
    remaining_time: u64,
) -> (u64, Duration) {
    let pos;
    let start_time = Instant::now();
    let count: i32 = (board.black_board.count_ones() + board.white_board.count_ones()) as i32;
    let thinking_time;
    if count <= 20 {
        thinking_time = (remaining_time / (64 - count as u64)) / 8;
    } else {
        thinking_time = remaining_time / ((64 - count as u64) / 2 + 1);
    }
    if ai_name == "rn" {
        pos = random_pos(&board);
    } else if ai_name == "ab" {
        pos = alpha_beta_pos(&board, 8);
    } else if ai_name == "na" {
        pos = nega_alpha_transpose_pos(&board, depth);
    } else if let Some(pos_ref) = book.get(board) {
        pos = *pos_ref;
    } else if count >= 47 {
        println!(
            "Let's think {:.2?}",
            Duration::from_millis(thinking_time * 2)
        );
        pos =
            nega_scout_transpose_pos(&board, 65 - count, Duration::from_millis(thinking_time * 2));
    } else {
        println!("Let's think {:.2?}", Duration::from_millis(thinking_time));
        pos = nega_scout_transpose_pos(&board, depth, Duration::from_millis(thinking_time));
    }
    let duration = start_time.elapsed();
    println!("Thinking time: {:.2?}", duration);
    (pos, duration)
}
