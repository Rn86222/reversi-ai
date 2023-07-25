pub fn cmd_to_pos(cmd: String) -> u64 {
    let mut pos: u64 = 1;
    if let Some(first) = cmd.chars().nth(0) {
        if (first as u8) < ('A' as u8) || ('H' as u8) < (first as u8) {
            println!("invalid command");
            return 0;
        }
        let x = first as u8 - 'A' as u8;
        pos <<= x;
    } else {
        println!("too short command");
        return 0;
    }
    if let Some(second) = cmd.chars().nth(1) {
        if (second as u8) < ('1' as u8) || ('8' as u8) < (second as u8) {
            println!("invalid command");
            return 0;
        }
        let y = second as u8 - '1' as u8;
        pos <<= y * 8;
    } else {
        println!("too short command");
        return 0;
    }
    pos
}

pub fn lower_cmd_to_pos(cmd: String) -> u64 {
    let mut pos: u64 = 1;
    if let Some(first) = cmd.chars().nth(0) {
        if (first as u8) < ('a' as u8) || ('h' as u8) < (first as u8) {
            println!("invalid command");
            return 0;
        }
        let x = first as u8 - 'a' as u8;
        pos <<= x;
    } else {
        println!("too short command");
        return 0;
    }
    if let Some(second) = cmd.chars().nth(1) {
        if (second as u8) < ('1' as u8) || ('8' as u8) < (second as u8) {
            println!("invalid command");
            return 0;
        }
        let y = second as u8 - '1' as u8;
        pos <<= y * 8;
    } else {
        println!("too short command");
        return 0;
    }
    pos
}

pub fn pos_to_cmd(pos: &u64) -> String {
    let first;
    let second;
    let pos_index = pos.trailing_zeros();
    first = (pos_index % 8) as u8 + 'A' as u8;
    second = (pos_index / 8) as u8 + '1' as u8;
    let cmd = format!("{}{}", first as char, second as char);
    cmd
}
