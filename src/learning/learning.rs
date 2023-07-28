use crate::parameter::parameter::*;
use crate::reversi::reversi::*;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

/*

pattern 0
1 1 1 1 1 1 1 1
  1         1

pattern 1
1
  1
    1
      1
        1
          1
            1
              1

pattern 2
1 1 1 1
1 1 1
1 1
1

*/

type P0Index = (usize, usize, usize, usize);
type P1Index = (usize, usize);
type P2Index = (usize, usize, usize, usize);
type PatternIndex = (P0Index, P1Index, P2Index);
type PatternValue = (Vec<f32>, Vec<f32>, Vec<f32>);

#[inline]
fn calculate_index(board: &Board, index: &usize, i: i32) -> usize {
    let mut new_index = *index;
    new_index *= 3;
    new_index += ((((board.black_board >> i) & 1) << 1) + ((board.white_board >> i) & 1)) as usize;
    new_index
}

pub fn pattern0_indexes(board: &Board) -> P0Index {
    let mut index0: usize = 0;
    for i in 0..8 {
        index0 = calculate_index(board, &index0, i);
    }
    index0 = calculate_index(board, &index0, 9);
    index0 = calculate_index(board, &index0, 14);
    let mut index1 = 0;
    for i in 0..8 {
        index1 = calculate_index(board, &index1, 56 - 8 * i);
    }
    index1 = calculate_index(board, &index1, 49);
    index1 = calculate_index(board, &index1, 9);
    let mut index2 = 0;
    for i in 0..8 {
        index2 = calculate_index(board, &index2, 63 - i);
    }
    index2 = calculate_index(board, &index2, 54);
    index2 = calculate_index(board, &index2, 49);
    let mut index3 = 0;
    for i in 0..8 {
        index3 = calculate_index(board, &index3, 7 + 8 * i);
    }
    index3 = calculate_index(board, &index3, 14);
    index3 = calculate_index(board, &index3, 54);
    (index0, index1, index2, index3)
}

pub fn pattern1_indexes(board: &Board) -> P1Index {
    let mut index0: usize = 0;
    for i in 0..8 {
        index0 = calculate_index(board, &index0, i + 8 * i);
    }
    let mut index1 = 0;
    for i in 0..8 {
        index1 = calculate_index(board, &index1, (7 - i) + 8 * i);
    }
    (index0, index1)
}

pub fn pattern2_indexes(board: &Board) -> P2Index {
    let mut index0: usize = 0;
    let positions0 = [0, 1, 2, 3, 8, 9, 10, 11, 16, 24];
    for i in positions0 {
        index0 = calculate_index(board, &index0, i);
    }
    let mut index1 = 0;
    let positions1 = [56, 48, 40, 32, 57, 49, 41, 58, 50, 59];
    for i in positions1 {
        index1 = calculate_index(board, &index1, i);
    }
    let mut index2 = 0;
    let positions2 = [63, 62, 61, 60, 55, 54, 53, 47, 46, 39];
    for i in positions2 {
        index2 = calculate_index(board, &index2, i);
    }
    let mut index3 = 0;
    let positions3 = [7, 15, 23, 31, 6, 14, 22, 5, 13, 4];
    for i in positions3 {
        index3 = calculate_index(board, &index3, i);
    }
    (index0, index1, index2, index3)
}

pub fn evaluate_board_pattern(board: &Board, pattern_value: &PatternValue) -> (PatternIndex, f32) {
    let mut score: f32 = 0.0;
    let (p0v, p1v, p2v) = pattern_value;
    let p0_indexes = pattern0_indexes(board);
    let p1_indexes = pattern1_indexes(board);
    let p2_indexes = pattern2_indexes(board);

    let (p0_0, p0_1, p0_2, p0_3) = p0_indexes;
    score += p0v[p0_0];
    score += p0v[p0_1];
    score += p0v[p0_2];
    score += p0v[p0_3];

    let (p1_0, p1_1) = p1_indexes;
    score += p1v[p1_0];
    score += p1v[p1_1];

    let (p2_0, p2_1, p2_2, p2_3) = p2_indexes;
    score += p2v[p2_0];
    score += p2v[p2_1];
    score += p2v[p2_2];
    score += p2v[p2_3];

    ((p0_indexes, p1_indexes, p2_indexes), score)
}

pub fn evaluate_board_pattern_score(board: &Board) -> i32 {
    let mut score: i32 = 0;
    let p0_indexes = pattern0_indexes(board);
    let p1_indexes = pattern1_indexes(board);
    let p2_indexes = pattern2_indexes(board);

    let (p0_0, p0_1, p0_2, p0_3) = p0_indexes;
    score += P0_VALUE[p0_0];
    score += P0_VALUE[p0_1];
    score += P0_VALUE[p0_2];
    score += P0_VALUE[p0_3];

    let (p1_0, p1_1) = p1_indexes;
    score += P1_VALUE[p1_0];
    score += P1_VALUE[p1_1];

    let (p2_0, p2_1, p2_2, p2_3) = p2_indexes;
    score += P2_VALUE[p2_0];
    score += P2_VALUE[p2_1];
    score += P2_VALUE[p2_2];
    score += P2_VALUE[p2_3];

    score
}

pub fn evaluate_board_legal(board: &Board, legal_value: f32) -> (i32, f32) {
    let mut tmp_board = *board;
    tmp_board.turn = BLACK;
    let black_legal = legal(tmp_board).count_ones();
    tmp_board.turn = WHITE;
    let white_legal = legal(tmp_board).count_ones();

    let legal_diff = black_legal as i32 - white_legal as i32;
    (legal_diff, legal_diff as f32 * legal_value)
}

pub fn evaluate_board_legal_score(board: &Board) -> i32 {
    let mut tmp_board = *board;
    tmp_board.turn = BLACK;
    let black_legal = legal(tmp_board).count_ones();
    tmp_board.turn = WHITE;
    let white_legal = legal(tmp_board).count_ones();

    let legal_diff = black_legal as i32 - white_legal as i32;
    legal_diff * LEGAL_VALUE
}

pub fn one_play(
    record: String,
    pattern_value: &PatternValue,
    legal_value: f32,
) -> (Vec<(PatternIndex, i32, f32)>, i32) {
    let mut board: Board = Board {
        black_board: 0,
        white_board: 0,
        turn: BLACK,
        no_legal_command: 0,
        value: 0,
        before_pos: 0,
    };
    init_board(&mut board);
    let mut data: Vec<(PatternIndex, i32, f32)> = vec![];
    let len = record.len();

    for i in 0..(len / 2) {
        board = execute_lower_cmd(&mut board, record[(2 * i)..=(2 * i + 1)].to_string());
        if legal(board) == 0 {
            board.turn = !board.turn;
        }
        if i >= 15 {
            let (pattern_index, pattern_score) = evaluate_board_pattern(&board, pattern_value);
            let (legal_diff, legal_score) = evaluate_board_legal(&board, legal_value);
            data.push((pattern_index, legal_diff, pattern_score + legal_score));
        }
    }
    let final_diff = board.black_board.count_ones() as i32 - board.white_board.count_ones() as i32;
    (data, final_diff)
}

pub fn one_train(
    record: String,
    pattern_value: &PatternValue,
    legal_value: f32,
    learning_rate: f32,
) -> (PatternValue, f32) {
    let (data, final_diff) = one_play(record, pattern_value, legal_value);
    let (mut new_p0_value, mut new_p1_value, mut new_p2_value) = pattern_value.clone();
    let mut new_legal_value = legal_value;
    for ((p0_indexes, p1_indexes, p2_indexes), legal_diff, score) in data {
        let error = final_diff as f32 - score;
        // println!(
        //     "final_diff: {}, score: {}, error: {}",
        //     final_diff, score, error
        // );
        let (p0_0, p0_1, p0_2, p0_3) = p0_indexes;
        let (p1_0, p1_1) = p1_indexes;
        let (p2_0, p2_1, p2_2, p2_3) = p2_indexes;
        if p0_0 > 0 {
            new_p0_value[p0_0] += error * learning_rate;
        }
        if p0_1 > 0 {
            new_p0_value[p0_1] += error * learning_rate;
        }
        if p0_2 > 0 {
            new_p0_value[p0_2] += error * learning_rate;
        }
        if p0_3 > 0 {
            new_p0_value[p0_3] += error * learning_rate;
        }
        if p1_0 > 0 {
            new_p1_value[p1_0] += error * learning_rate;
        }
        if p1_1 > 0 {
            new_p1_value[p1_1] += error * learning_rate;
        }
        if p2_0 > 0 {
            new_p2_value[p2_0] += error * learning_rate;
        }
        if p2_1 > 0 {
            new_p2_value[p2_1] += error * learning_rate;
        }
        if p2_2 > 0 {
            new_p2_value[p2_2] += error * learning_rate;
        }
        if p2_3 > 0 {
            new_p2_value[p2_3] += error * learning_rate;
        }
        new_legal_value += legal_diff as f32 * error * learning_rate;
    }
    ((new_p0_value, new_p1_value, new_p2_value), new_legal_value)
}

pub fn evaluate_model(path: String, pattern_value: &PatternValue, legal_value: f32) -> f32 {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut loss = 0.0;
    let mut record_count = 0;
    for line in reader.lines() {
        record_count += 1;
        let mut one_loss = 0.0;
        let record = line.unwrap();
        let (data, final_diff) = one_play(record, pattern_value, legal_value);
        let len = data.len();
        for (_, _, score) in data {
            one_loss += (score - final_diff as f32).abs();
        }
        loss += one_loss / len as f32;
    }
    loss /= record_count as f32;
    println!("loss = {}", loss);
    loss
}

pub fn train(epoch: usize, learning_rate: f32) {
    let mut paths: Vec<String> = vec![];
    for i in 0..19 {
        paths.push(format!("self_play/{i:0>7}.txt"));
    }
    paths.push("wthor_2005_2009.txt".to_string());
    paths.push("wthor_2010_2023.txt".to_string());

    let p0_value: Vec<f32> = vec![0.0; 59049];
    let p1_value: Vec<f32> = vec![0.0; 6561];
    let p2_value: Vec<f32> = vec![0.0; 59049];
    let mut pattern_value = (p0_value, p1_value, p2_value);
    let mut legal_value: f32 = 0.0;

    for i in 0..epoch {
        println!("epoch: {}", i);
        for path in paths.clone() {
            let file = File::open(path).unwrap();
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let record = line.unwrap();
                (pattern_value, legal_value) =
                    one_train(record, &pattern_value, legal_value, learning_rate);
            }
        }
        evaluate_model(
            "self_play/0000019.txt".to_string(),
            &pattern_value,
            legal_value,
        );
    }

    let (p0_value, p1_value, p2_value) = pattern_value;
    let mut file = File::create("train_result4.txt").unwrap();

    let mut nonzero_count = 0;
    for item in p0_value {
        if item < -0.0001 || 0.0001 < item {
            nonzero_count += 1;
        }
        file.write_all(item.to_string().as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }
    for item in p1_value {
        if item < -0.0001 || 0.0001 < item {
            nonzero_count += 1;
        }
        file.write_all(item.to_string().as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }
    for item in p2_value {
        if item < -0.00001 || 0.00001 < item {
            nonzero_count += 1;
        }
        file.write_all(item.to_string().as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }
    file.write_all(legal_value.to_string().as_bytes()).unwrap();

    file.flush().unwrap();

    println!("nonzero_count: {}", nonzero_count);
}

/*

pattern 3
1 1 1 1 1 1 1 1


pattern 4
1
  1
    1
      1
        1
          1
            1
              1

pattern 5
1 1 1
1 1 1
1 1


*/

type P3Index = (usize, usize, usize, usize);
type P4Index = (usize, usize);
type P5Index = (usize, usize, usize, usize);

pub fn pattern3_indexes(board: &Board) -> P3Index {
    let mut index0: usize = 0;
    for i in 0..8 {
        index0 = calculate_index(board, &index0, i);
    }
    let mut index1 = 0;
    for i in 0..8 {
        index1 = calculate_index(board, &index1, 56 - 8 * i);
    }
    let mut index2 = 0;
    for i in 0..8 {
        index2 = calculate_index(board, &index2, 63 - i);
    }
    let mut index3 = 0;
    for i in 0..8 {
        index3 = calculate_index(board, &index3, 7 + 8 * i);
    }
    (index0, index1, index2, index3)
}

pub fn pattern4_indexes(board: &Board) -> P4Index {
    let mut index0: usize = 0;
    for i in 0..8 {
        index0 = calculate_index(board, &index0, i + 8 * i);
    }
    let mut index1 = 0;
    for i in 0..8 {
        index1 = calculate_index(board, &index1, (7 - i) + 8 * i);
    }
    (index0, index1)
}

pub fn pattern5_indexes(board: &Board) -> P5Index {
    let mut index0: usize = 0;
    let positions0 = [0, 1, 2, 8, 9, 10, 11, 16];
    for i in positions0 {
        index0 = calculate_index(board, &index0, i);
    }
    let mut index1 = 0;
    let positions1 = [56, 48, 40, 57, 49, 41, 58, 50];
    for i in positions1 {
        index1 = calculate_index(board, &index1, i);
    }
    let mut index2 = 0;
    let positions2 = [63, 62, 61, 55, 54, 53, 47, 46];
    for i in positions2 {
        index2 = calculate_index(board, &index2, i);
    }
    let mut index3 = 0;
    let positions3 = [7, 15, 23, 6, 14, 22, 5, 13];
    for i in positions3 {
        index3 = calculate_index(board, &index3, i);
    }
    (index0, index1, index2, index3)
}

pub fn evaluate_board_pattern_mini(
    board: &Board,
    pattern_value: &PatternValue,
) -> (PatternIndex, f32) {
    let mut score: f32 = 0.0;
    let (p3v, p4v, p5v) = pattern_value;
    let p3_indexes = pattern3_indexes(board);
    let p4_indexes = pattern4_indexes(board);
    let p5_indexes = pattern5_indexes(board);

    let (p3_0, p3_1, p3_2, p3_3) = p3_indexes;
    score += p3v[p3_0];
    score += p3v[p3_1];
    score += p3v[p3_2];
    score += p3v[p3_3];

    let (p4_0, p4_1) = p4_indexes;
    score += p4v[p4_0];
    score += p4v[p4_1];

    let (p5_0, p5_1, p5_2, p5_3) = p5_indexes;
    score += p5v[p5_0];
    score += p5v[p5_1];
    score += p5v[p5_2];
    score += p5v[p5_3];

    ((p3_indexes, p4_indexes, p5_indexes), score)
}

// pub fn evaluate_board_pattern_score_mini(board: &Board) -> f32 {
//     let mut score: f32 = 0.0;
//     let p3_indexes = pattern3_indexes(board);
//     let p4_indexes = pattern4_indexes(board);
//     let p5_indexes = pattern5_indexes(board);

//     let (p3_0, p3_1, p3_2, p3_3) = p3_indexes;
//     score += P3_VALUE[p3_0];
//     score += P3_VALUE[p3_1];
//     score += P3_VALUE[p3_2];
//     score += P3_VALUE[p3_3];

//     let (p4_0, p4_1) = p4_indexes;
//     score += P4_VALUE[p4_0];
//     score += P4_VALUE[p4_1];

//     let (p5_0, p5_1, p5_2, p5_3) = p5_indexes;
//     score += P5_VALUE[p5_0];
//     score += P5_VALUE[p5_1];
//     score += P5_VALUE[p5_2];
//     score += P5_VALUE[p5_3];

//     score
// }

pub fn evaluate_board_legal_mini(board: &Board, legal_value: f32) -> (i32, f32) {
    let mut tmp_board = *board;
    tmp_board.turn = BLACK;
    let black_legal = legal(tmp_board).count_ones();
    tmp_board.turn = WHITE;
    let white_legal = legal(tmp_board).count_ones();

    let legal_diff = black_legal as i32 - white_legal as i32;
    (legal_diff, legal_diff as f32 * legal_value)
}

// pub fn evaluate_board_legal_score_mini(board: &Board) -> f32 {
//     let mut tmp_board = *board;
//     tmp_board.turn = BLACK;
//     let black_legal = legal(tmp_board).count_ones();
//     tmp_board.turn = WHITE;
//     let white_legal = legal(tmp_board).count_ones();

//     let legal_diff = black_legal as i32 - white_legal as i32;
//     legal_diff as f32 * LEGAL_VALUE_MINI
// }

pub fn one_play_mini(
    record: String,
    pattern_value: &PatternValue,
    legal_value: f32,
) -> (Vec<(PatternIndex, i32, f32)>, i32) {
    let mut board: Board = Board {
        black_board: 0,
        white_board: 0,
        turn: BLACK,
        no_legal_command: 0,
        value: 0,
        before_pos: 0,
    };
    init_board(&mut board);
    let mut data: Vec<(PatternIndex, i32, f32)> = vec![];
    let len = record.len();

    for i in 0..(len / 2) {
        board = execute_lower_cmd(&mut board, record[(2 * i)..=(2 * i + 1)].to_string());
        if legal(board) == 0 {
            board.turn = !board.turn;
        }
        if i >= 15 {
            let (pattern_index, pattern_score) = evaluate_board_pattern_mini(&board, pattern_value);
            let (legal_diff, legal_score) = evaluate_board_legal_mini(&board, legal_value);
            data.push((pattern_index, legal_diff, pattern_score + legal_score));
        }
    }
    let final_diff = board.black_board.count_ones() as i32 - board.white_board.count_ones() as i32;
    (data, final_diff)
}

pub fn one_train_mini(
    record: String,
    pattern_value: &PatternValue,
    legal_value: f32,
    learning_rate: f32,
) -> (PatternValue, f32) {
    let (data, final_diff) = one_play_mini(record, pattern_value, legal_value);
    let (mut new_p3_value, mut new_p4_value, mut new_p5_value) = pattern_value.clone();
    let mut new_legal_value = legal_value;
    for ((p3_indexes, p4_indexes, p5_indexes), legal_diff, score) in data {
        let error = final_diff as f32 - score;
        // println!(
        //     "final_diff: {}, score: {}, error: {}",
        //     final_diff, score, error
        // );
        let (p3_0, p3_1, p3_2, p3_3) = p3_indexes;
        let (p4_0, p4_1) = p4_indexes;
        let (p5_0, p5_1, p5_2, p5_3) = p5_indexes;
        if p3_0 > 0 {
            new_p3_value[p3_0] += error * learning_rate;
        }
        if p3_1 > 0 {
            new_p3_value[p3_1] += error * learning_rate;
        }
        if p3_2 > 0 {
            new_p3_value[p3_2] += error * learning_rate;
        }
        if p3_3 > 0 {
            new_p3_value[p3_3] += error * learning_rate;
        }
        if p4_0 > 0 {
            new_p4_value[p4_0] += error * learning_rate;
        }
        if p4_1 > 0 {
            new_p4_value[p4_1] += error * learning_rate;
        }
        if p5_0 > 0 {
            new_p5_value[p5_0] += error * learning_rate;
        }
        if p5_1 > 0 {
            new_p5_value[p5_1] += error * learning_rate;
        }
        if p5_2 > 0 {
            new_p5_value[p5_2] += error * learning_rate;
        }
        if p5_3 > 0 {
            new_p5_value[p5_3] += error * learning_rate;
        }
        new_legal_value += legal_diff as f32 * error * learning_rate;
    }
    ((new_p3_value, new_p4_value, new_p5_value), new_legal_value)
}

pub fn evaluate_model_mini(path: String, pattern_value: &PatternValue, legal_value: f32) -> f32 {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut loss = 0.0;
    let mut record_count = 0;
    for line in reader.lines() {
        record_count += 1;
        let mut one_loss = 0.0;
        let record = line.unwrap();
        let (data, final_diff) = one_play_mini(record, pattern_value, legal_value);
        let len = data.len();
        for (_, _, score) in data {
            one_loss += (score - final_diff as f32).abs();
        }
        loss += one_loss / len as f32;
    }
    loss /= record_count as f32;
    println!("loss = {}", loss);
    loss
}

pub fn train_mini(epoch: usize, learning_rate: f32) {
    let mut paths: Vec<String> = vec![];
    for i in 0..19 {
        paths.push(format!("self_play/{i:0>7}.txt"));
    }
    paths.push("wthor_2005_2009.txt".to_string());
    paths.push("wthor_2010_2023.txt".to_string());

    let p0_value: Vec<f32> = vec![0.0; 6561];
    let p1_value: Vec<f32> = vec![0.0; 6561];
    let p2_value: Vec<f32> = vec![0.0; 6561];
    let mut pattern_value = (p0_value, p1_value, p2_value);
    let mut legal_value: f32 = 0.0;

    for i in 0..epoch {
        println!("epoch: {}", i);
        for path in paths.clone() {
            let file = File::open(path).unwrap();
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let record = line.unwrap();
                (pattern_value, legal_value) =
                    one_train_mini(record, &pattern_value, legal_value, learning_rate);
            }
        }
        evaluate_model_mini(
            "self_play/0000019.txt".to_string(),
            &pattern_value,
            legal_value,
        );
    }

    let (p0_value, p1_value, p2_value) = pattern_value;
    let mut file = File::create("train_result_mini.txt").unwrap();

    let mut nonzero_count = 0;
    for item in p0_value {
        if item < -0.0001 || 0.0001 < item {
            nonzero_count += 1;
        }
        file.write_all(item.to_string().as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }
    for item in p1_value {
        if item < -0.0001 || 0.0001 < item {
            nonzero_count += 1;
        }
        file.write_all(item.to_string().as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }
    for item in p2_value {
        if item < -0.00001 || 0.00001 < item {
            nonzero_count += 1;
        }
        file.write_all(item.to_string().as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }
    file.write_all(legal_value.to_string().as_bytes()).unwrap();

    file.flush().unwrap();

    println!("nonzero_count: {}", nonzero_count);
}
