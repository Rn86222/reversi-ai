pub enum Command {
    Open(String),
    Start(String, String, String),
    Move(String),
    Ack(u64),
    End(String, u32, u32, String),
    Bye(String),
}

pub enum ClientState {
    CardWaiting,
    MyTurn,
    OpponentTurn,
    AckWaiting,
    Ended,
}

use Command::*;

pub fn mes_to_command(mes: &String) -> Command {
    assert_eq!(mes.ends_with('\n'), true);
    let mes = mes[..mes.len() - 1].to_string();
    let parsed_mes: Vec<String> = mes.split_whitespace().map(|s| s.to_string()).collect();
    let length = parsed_mes.len();
    assert!(length > 1);

    if parsed_mes[0] == String::from("OPEN") {
        assert_eq!(length, 2);
        Move(parsed_mes[1].to_string())
    } else if parsed_mes[0] == String::from("ACK") {
        assert_eq!(length, 2);
        let time: u64 = parsed_mes[1].parse().unwrap();
        Ack(time)
    } else if parsed_mes[0] == String::from("END") {
        assert_eq!(length, 5);
        let n: u32 = parsed_mes[2].parse().unwrap();
        let m: u32 = parsed_mes[3].parse().unwrap();
        End(parsed_mes[1].to_string(), n, m, parsed_mes[4].to_string())
    } else if parsed_mes[0] == String::from("START") {
        assert_eq!(length, 4);
        Start(
            parsed_mes[1].to_string(),
            parsed_mes[2].to_string(),
            parsed_mes[3].to_string(),
        )
    } else if parsed_mes[0] == String::from("MOVE") {
        assert_eq!(length, 2);
        Move(parsed_mes[1].to_string())
    } else if parsed_mes[0] == String::from("OPEN") {
        assert_eq!(length, 2);
        Open(parsed_mes[1].to_string())
    } else if parsed_mes[0] == String::from("BYE") {
        assert!(mes.len() > 4);
        Bye(mes[4..].to_string())
    } else {
        panic!();
    }
}
