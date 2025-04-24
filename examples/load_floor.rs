use v_archive_rs::{
    load_user_floor_board, UserChartRecord, UserFloorRecordBoard, UserFloorRecordSet,
};

fn a_line(line_type: i32) {
    match line_type {
        0 => println!("{}", "-".repeat(45)),
        1 => println!("{}", "=".repeat(45)),
        _ => {}
    }
}

fn get_records_from_board(floors: Vec<UserFloorRecordSet>) -> Vec<UserChartRecord> {
    let mut song_list = Vec::new();

    for f in floors {
        for r in f.records {
            song_list.push(r);
        }
    }

    song_list
}

fn display_song_result(board: UserFloorRecordBoard, display_range: usize) {
    let mut records = get_records_from_board(board.floors);

    records.sort_by(|a, b| {
        b.acc_rate
            .unwrap_or(0.0)
            .partial_cmp(&a.acc_rate.unwrap_or(0.0))
            .unwrap()
    });

    let display_range = if display_range < records.len() {
        display_range
    } else {
        records.len()
    };
    for r in &records[..display_range] {
        print!("[{}] ", r.acc_rate.unwrap_or(0.0));
        print!("{} ({})", r.title, r.chart_type.to_string());
        print!("\n");
    }
}

fn display_board(board: UserFloorRecordBoard, username: &str, buttons: u8) {
    a_line(1);
    println!(
        "{}\'s {} buttons \"{}\" record board.",
        username,
        buttons,
        board.board_type.to_string()
    );
    a_line(0);
    println!("Here's top 5 on this board:");
    display_song_result(board, 5);
    a_line(1);
}

fn main() {
    let username = "내꺼";
    let buttons: u8 = 6;
    let board_type = "MX";

    let user_data_resp = load_user_floor_board(username, buttons, board_type);

    match user_data_resp {
        Ok(d) => display_board(d, username, buttons),
        Err(e) => println!("You've got an error: {}", e.to_string()),
    };
}
