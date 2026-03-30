use v_archive_rs::{
    register_record, ButtonMode, ChartType, LegacyExtCat, SongCatagory, UserChartRecord, UserToken,
};

fn main() {
    let user_num = 1;
    let user_key = "95d6c422-52b4-4016-8587-38c46a2e7917";

    let user_token = UserToken {
        user_num,
        user_token: user_key.to_owned(),
    };

    let record = UserChartRecord {
        song_id: 0,
        title: "Urban Night".to_string(),
        button: ButtonMode::Six,
        chart_type: ChartType::Sc,
        acc_rate: Some(90.9),
        is_max_combo: false,
        chart_level: None,
        floor_level: None,
        user_rating: None,
        maximum_rating: None,
        dj_power: None,
        song_cat: Some(SongCatagory::LegacyExtention(LegacyExtCat::EmotionalSense)),
        updated_at: None,
    };

    let req = register_record(user_token, record);

    match req {
        Ok(result) => {
            println!("Record registered successfully: {:?}", result);
            if result.update {
                println!("Your record updated!");
            }
        }
        Err(error) => {
            println!("Failed to register record: {:?}", error);
        }
    }
}
