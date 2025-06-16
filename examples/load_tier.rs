use v_archive_rs::{load_user_tier, UserTierRecordTable};

fn a_line(line_type: i32) {
    match line_type {
        0 => println!("{}", "-".repeat(45)),
        1 => println!("{}", "=".repeat(45)),
        _ => {}
    }
}

fn display_tier(table: UserTierRecordTable, username: &str, buttons: u8) {
    a_line(1);
    println!("{}\'s {} buttons tier records", username, buttons);
    a_line(0);
    println!("Your tier is {}", table.current_tier.to_string());
    println!();
    println!("Top 50 sum: {}", table.fifteen_sum);
    println!("Tier point: {}", table.tier_point);
    a_line(0);
    println!("Song list");
    a_line(0);
    let mut count = 0;
    for r in table.top_records {
        print!("{} ", r.title);
        print!("<{}> ", r.chart_type.to_string());
        print!("- {}({})", r.acc_rate.unwrap(), r.user_rating.unwrap());
        print!("\n");

        if count > 10 {
            break;
        } else {
            count += 1;
        }
    }
    a_line(1);
}

fn main() {
    let username = "내꺼";
    let buttons: u8 = 6;

    let user_data_resp = load_user_tier(username, buttons);

    match user_data_resp {
        Ok(d) => display_tier(d, username, buttons),
        Err(e) => println!("You've got an error: {}", e.to_string()),
    };
}
