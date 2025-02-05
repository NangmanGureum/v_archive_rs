use serde::{Deserialize, Serialize};
use serde_this_or_that::{as_bool, as_f64, as_u64};
use std::fmt;
use ureq::{Error, Response};

/// This is using for a lot of errors from V-Archive sever.
/// Mostly, it comes `Result<_, APIError>`
#[derive(Debug)]
pub enum APIError {
    CannotFindUser,
    HasNoButtonRecord,
    CannotFindSong,
    FoundSeveralSongs,
    CannotFoundChart,
    WrongParameter(String),
    //
    // Other Server Error
    InernalServerError,
    APIUnknownError(u16, String),
    HTTPErr(u16),
    UnknownError,
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            APIError::CannotFindUser => write!(f, "Cannot find user"),
            APIError::HasNoButtonRecord => write!(f, "Has no button record"),
            APIError::CannotFindSong => write!(f, "Cannot find song"),
            APIError::FoundSeveralSongs => write!(f, "Found several songs"),
            APIError::CannotFoundChart => write!(f, "Cannot find chart"),
            APIError::WrongParameter(m) => write!(f, "Wrong parameter(s): {}", m),
            APIError::InernalServerError => write!(f, "Inernal server error"),
            APIError::APIUnknownError(c, m) => write!(f, "Unknown API error: {}, {}", c, m),
            APIError::HTTPErr(c) => write!(f, "HTTP error: {}", c),
            APIError::UnknownError => write!(f, "Unknown"),
        }
    }
}

/// Return to error object. for in this crate
fn catch_server_err(code: u16, resp: Response) -> APIError {
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct APIBody {
        error_code: u16,
        message: String,
    }

    let resp_str = resp.into_string().unwrap();
    let error_body: APIBody = serde_json::from_str(&resp_str).unwrap();

    match code {
        400 | 404 => {
            let error_code = error_body.error_code;
            match error_code {
                101 => {
                    return APIError::CannotFindUser;
                }
                111 => {
                    return APIError::HasNoButtonRecord;
                }
                201 => {
                    return APIError::CannotFindSong;
                }
                202 => {
                    return APIError::FoundSeveralSongs;
                }
                211 => {
                    return APIError::CannotFoundChart;
                }
                900 => {
                    let err_msg = error_body.message;
                    return APIError::WrongParameter(err_msg);
                }
                c => {
                    let err_msg = error_body.message;
                    return APIError::APIUnknownError(c, err_msg);
                }
            }
        }
        500 => return APIError::InernalServerError,
        c => return APIError::HTTPErr(c),
    };
}

#[derive(Debug)]
pub enum RespectCat {
    Respect,
    RespectV,
}

#[derive(Debug)]
pub enum LegacyCat {
    PortableOne,
    PortableTwo,
}

#[derive(Debug)]
pub enum LegacyExtCat {
    Trilogy,
    Clazziquai,
    Technika,
    BlackSquare,
    TechnikaTwo,
    TechnikaThree,
    EmotionalSense,
    PortableThree,
    TechnikaTuneQ,
}

#[derive(Debug)]
pub enum NewExtCat {
    VExtentionOne,
    VExtentionTwo,
    VExtentionThree,
    VExtentionFour,
    VExtentionFive,
    VLivertyOne,
    VLivertyTwo,
}

#[derive(Debug)]
pub enum SongCatagory {
    Respect(RespectCat),
    Legacy(LegacyCat),
    LegacyExtention(LegacyExtCat),
    NewExtention(NewExtCat),
    MusicCollab(String),
    Others(String),
}

#[derive(Debug)]
pub enum ButtonMode {
    Four,
    Five,
    Six,
    Eight,
}

/// A struct for user's record for a chart
#[derive(Debug)]
pub struct UserRecordForChart {
    /// ID number for a song of the chart
    pub song_id: usize,
    /// A song title for the chart
    pub title: String,
    /// A button type of the chart
    pub button: ButtonMode,
    /// A user's accuracy rate for the chart
    pub acc_rate: f64,
    /// user's max combo or not for the chart
    pub is_max_combo: bool,
    /// A level for the chart
    pub level: u8,
    /// A level on V-Archive's floor
    pub floor_level: f64,
    /// A user's rating on V-Archive for a chart
    pub user_rating: f64,
    /// A maximum rating on V-Archive for a chart
    pub maximum_rating: f64,
    /// A DJPOWER point for DJMAX. (This may differ from in-game.)
    pub dj_power: f64,
    /// A category for a song of the chart
    pub song_cat: SongCatagory,
}

/// This is a user's play result for a song. Legacy struct (which will be removed.)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VArchivePatternResult {
    /// ID number for a song of the pattern
    pub title: usize,
    /// A song title of the pattern
    pub name: String,
    /// A button type of the pattern
    pub button: u8,
    /// A difficulty type of the pattern
    pub pattern: String,
    /// A level number of the pattern
    pub level: u8,
    /// A floor level from V-Archive of the pattern
    #[serde(deserialize_with = "as_f64")]
    pub floor: f64,
    /// Maximum rating of the pattern
    #[serde(deserialize_with = "as_f64")]
    pub max_rating: f64,
    /// The user's accuracy rate of the pattern
    #[serde(deserialize_with = "as_f64")]
    pub score: f64,
    /// The user's MAX COMBO
    #[serde(deserialize_with = "as_bool")]
    pub max_combo: bool,
    /// The user's rating of the pattern
    #[serde(deserialize_with = "as_f64")]
    pub rating: f64,
}

impl VArchivePatternResult {
    pub fn new() -> Self {
        VArchivePatternResult {
            title: 0,
            name: String::new(),
            button: 4,
            pattern: String::from("NM"),
            level: 1,
            floor: 0.0,
            max_rating: 0.0,
            score: 0.0,
            max_combo: false,
            rating: 0.0,
        }
    }
}

/// This is a tier. Legacy struct (which will be removed.)
#[derive(Serialize, Deserialize, Debug)]
pub struct VArchiveTier {
    pub rating: u32,
    pub name: String,
    pub code: String,
}

impl VArchiveTier {
    pub fn new() -> Self {
        Self {
            rating: 0,
            name: String::from("Beginner"),
            code: String::from("BG"),
        }
    }

    /// Make a tier via server.
    pub fn from_point(tier_point: u32) -> Self {
        let req_result = ureq::get("https://v-archive.net/db/tiers.json").call();
        if let Ok(resp) = req_result {
            // If get tier list
            let resp_str = resp.into_string().unwrap();
            let tier_list: Vec<Self> = serde_json::from_str(&resp_str).unwrap();

            for tier in tier_list {
                if tier_point >= tier.rating {
                    return tier;
                }
            }

            return Self::new();
        } else {
            // If cannot tier list
            Self {
                rating: 0,
                name: String::from("Load Error"),
                code: String::from("ER"),
            }
        }
    }
}

/// This is a user's tier table. Legacy struct (which will be removed.)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VArchiveUserTierInfo {
    success: bool,
    #[serde(deserialize_with = "as_f64")]
    pub top50sum: f64,
    #[serde(deserialize_with = "as_f64")]
    pub tier_point: f64,
    pub tier: VArchiveTier,
    pub next: VArchiveTier,
    pub top_list: Vec<VArchivePatternResult>,
}

impl VArchiveUserTierInfo {
    pub fn new() -> Self {
        Self {
            success: true,
            top50sum: 0.0,
            tier_point: 0.0,
            tier: VArchiveTier::new(),
            next: VArchiveTier::new(),
            top_list: Vec::new(),
        }
    }

    pub fn current_tier_diff(&self) -> f64 {
        self.tier_point - (self.tier.rating as f64)
    }

    pub fn next_tier_diff(&self) -> f64 {
        self.next.rating as f64 - self.tier_point
    }

    /// Load a user's tier info from server
    /// ## Example
    /// ```rust
    /// # use v_archive_rs::VArchiveUserTierInfo;
    /// #
    /// # fn main() {
    /// # // Starts for showing code
    /// let username = "내꺼";
    /// let user_tier = VArchiveUserTierInfo::load_user_tier(username, 6);
    ///
    /// match user_tier {
    ///     Ok(tier) => {
    ///         println!(
    ///             "Success: {}'s Tier is {}(+{})",
    ///             username,
    ///             tier.tier.name,
    ///             tier.current_tier_diff()
    ///         );
    ///     }
    ///     Err(e) => {
    ///         println!("Load failed: {:?}", e);
    ///     }
    /// }
    /// # // Ends for showing code
    /// # }
    /// ```
    pub fn load_user_tier(username: &str, buttons: u8) -> Result<Self, APIError> {
        let get_url = format!("https://v-archive.net/api/archive/{username}/tier/{buttons}");
        let resp = ureq::get(&get_url)
            .set("Content-Type", "application/json")
            .call();

        match resp {
            Ok(resp) => {
                let resp_str = resp.into_string().unwrap();
                Ok(serde_json::from_str(&resp_str).unwrap())
            }
            Err(Error::Status(code, resp)) => Err(catch_server_err(code, resp)),
            Err(_) => Err(APIError::UnknownError),
        }
    }
}

/// This is a pattern for a song. Legacy struct (which will be removed.)
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct VArchivePattern {
    pub level: u8,
    #[serde(default)]
    pub floor: f64,
    #[serde(default)]
    #[serde(deserialize_with = "as_f64")]
    pub score: f64,
    #[serde(default)]
    #[serde(deserialize_with = "as_bool")]
    pub max_combo: bool,
    #[serde(default)]
    pub rating: f64,
}

impl VArchivePattern {
    pub fn new() -> Self {
        Self {
            level: 0,
            floor: 0.0,
            score: 0.0,
            max_combo: false,
            rating: 0.0,
        }
    }
}

/// This is pattern list for a kind of buttons for a song. Legacy struct (which will be removed.)
#[derive(Serialize, Deserialize, Debug)]
pub struct VArchivePatternList {
    #[serde(default)]
    #[serde(alias = "NM")]
    pub normal: VArchivePattern,
    #[serde(default)]
    #[serde(alias = "HD")]
    pub hard: VArchivePattern,
    #[serde(default)]
    #[serde(alias = "MX")]
    pub maximum: VArchivePattern,
    #[serde(default)]
    #[serde(alias = "SC")]
    pub sc: VArchivePattern,
}

impl VArchivePatternList {
    pub fn new() -> Self {
        Self {
            normal: VArchivePattern::new(),
            hard: VArchivePattern::new(),
            maximum: VArchivePattern::new(),
            sc: VArchivePattern::new(),
        }
    }
}

/// This is a pattern table for a song.
#[derive(Serialize, Deserialize, Debug)]
pub struct VArchivePatternTable {
    #[serde(alias = "4B")]
    pub four_buttons: VArchivePatternList,
    #[serde(alias = "5B")]
    pub five_buttons: VArchivePatternList,
    #[serde(alias = "6B")]
    pub six_buttons: VArchivePatternList,
    #[serde(alias = "8B")]
    pub eight_buttons: VArchivePatternList,
}

impl VArchivePatternTable {
    pub fn new() -> Self {
        Self {
            four_buttons: VArchivePatternList::new(),
            five_buttons: VArchivePatternList::new(),
            six_buttons: VArchivePatternList::new(),
            eight_buttons: VArchivePatternList::new(),
        }
    }
}

/// This is a user's song result. Legacy struct (which will be removed.)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VArchiveSongUserResult {
    success: bool,
    pub title: usize,
    pub name: String,
    pub composer: String,
    pub dlc_code: String,
    pub dlc: String,
    pub patterns: VArchivePatternTable,
}

impl VArchiveSongUserResult {
    pub fn new() -> Self {
        Self {
            success: true,
            title: 0,
            name: String::new(),
            composer: String::new(),
            dlc_code: String::new(),
            dlc: String::new(),
            patterns: VArchivePatternTable::new(),
        }
    }

    pub fn load_song_result(username: &str, song_id: &usize) -> Result<Self, APIError> {
        let get_url = format!("https://v-archive.net/api/archive/{username}/title/{song_id}");
        let resp = ureq::get(&get_url)
            .set("Content-Type", "application/json")
            .call();

        match resp {
            Ok(resp) => {
                let resp_str = resp.into_string().unwrap();
                Ok(serde_json::from_str(&resp_str).unwrap())
            }
            Err(Error::Status(code, resp)) => Err(catch_server_err(code, resp)),
            Err(_) => Err(APIError::UnknownError),
        }
    }
}

/// Legacy struct (which will be removed.)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VArchiveFloorSongResult {
    pub title: usize,
    pub name: String,
    pub composer: String,
    pub pattern: String,
    #[serde(deserialize_with = "as_f64")]
    pub score: f64,
    #[serde(deserialize_with = "as_bool")]
    pub max_combo: bool,
    pub djpower: f64,
    pub rating: f64,
    pub dlc: String,
    pub dlc_code: String,
}

impl VArchiveFloorSongResult {
    pub fn new() -> Self {
        Self {
            title: 0,
            name: String::new(),
            composer: String::new(),
            pattern: String::new(),
            score: 0.0,
            max_combo: false,
            djpower: 0.0,
            rating: 0.0,
            dlc: String::new(),
            dlc_code: String::new(),
        }
    }
}

/// Legacy struct (which will be removed.)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VArchiveUserFloor {
    pub floor_number: f64,
    pub patterns: Vec<VArchiveFloorSongResult>,
}

impl VArchiveUserFloor {
    pub fn new() -> Self {
        Self {
            floor_number: 0.0,
            patterns: Vec::new(),
        }
    }
}

/// Legacy struct (which will be removed.)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VArciveUserBoard {
    success: bool,
    #[serde(alias = "board")]
    pub board_type: String,
    #[serde(deserialize_with = "as_u64")]
    pub button: u64,
    pub total_count: usize,
    pub floors: Vec<VArchiveUserFloor>,
}

impl VArciveUserBoard {
    pub fn new() -> Self {
        Self {
            success: true,
            board_type: String::new(),
            button: 4,
            total_count: 0,
            floors: Vec::new(),
        }
    }

    pub fn load_user_board(
        username: &str,
        buttons: &u8,
        board_type: &str,
    ) -> Result<Self, APIError> {
        let get_url =
            format!("https://v-archive.net/api/archive/{username}/board/{buttons}/{board_type}");
        let resp = ureq::get(&get_url)
            .set("Content-Type", "application/json")
            .call();

        match resp {
            Ok(resp) => {
                let resp_str = resp.into_string().unwrap();
                Ok(serde_json::from_str(&resp_str).unwrap())
            }
            Err(Error::Status(code, resp)) => Err(catch_server_err(code, resp)),
            Err(_) => Err(APIError::UnknownError),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VArchiveRegisterResult {
    success: bool,
    update: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VArchiveRegisterRecord {
    pub name: String,
    pub dlc: String,
    pub composer: String,
    pub button: u8,
    pub pattern: String,
    pub score: f64,
    pub max_combo: u8,
}

impl VArchiveRegisterRecord {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            dlc: String::new(),
            composer: String::new(),
            button: 0,
            pattern: "NORMAL".to_string(),
            score: 0.0,
            max_combo: 0,
        }
    }
}

pub struct VArchiveUserToken {
    pub user_num: usize,
    pub user_token: String,
}

impl VArchiveUserToken {
    pub fn new() -> Self {
        Self {
            user_num: 0,
            user_token: String::new(),
        }
    }

    pub fn register_record(
        self,
        record: VArchiveRegisterRecord,
    ) -> Result<VArchiveRegisterResult, APIError> {
        let user_num = self.user_num;
        let record_serial = serde_json::to_string(&record).unwrap();

        let post_url = format!("https://v-archive.net/client/open/{user_num}/score");
        let resp = ureq::post(&post_url)
            .set("Authorization", &self.user_token)
            .set("Content-Type", "application/json")
            .send_string(&record_serial);

        match resp {
            Ok(resp) => {
                let resp_str = resp.into_string().unwrap();
                Ok(serde_json::from_str(&resp_str).unwrap())
            }
            Err(Error::Status(code, resp)) => Err(catch_server_err(code, resp)),
            Err(_) => Err(APIError::UnknownError),
        }
    }
}

/// Legacy struct (which will be removed.)
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct VArchiveSongPattern {
    pub level: u8,
    #[serde(default)]
    pub floor: f64,
    #[serde(default)]
    pub rating: f64,
}

/// Legacy struct (which will be removed.)
#[derive(Serialize, Deserialize, Debug)]
pub struct VArchiveSongPatternList {
    #[serde(default)]
    #[serde(alias = "NM")]
    pub normal: VArchiveSongPattern,
    #[serde(default)]
    #[serde(alias = "HD")]
    pub hard: VArchiveSongPattern,
    #[serde(default)]
    #[serde(alias = "MX")]
    pub maximum: VArchiveSongPattern,
    #[serde(default)]
    #[serde(alias = "SC")]
    pub sc: VArchiveSongPattern,
}

/// Legacy struct (which will be removed.)
#[derive(Serialize, Deserialize, Debug)]
pub struct VArchiveSongPatternTable {
    #[serde(alias = "4B")]
    pub four_buttons: VArchiveSongPatternList,
    #[serde(alias = "5B")]
    pub five_buttons: VArchiveSongPatternList,
    #[serde(alias = "6B")]
    pub six_buttons: VArchiveSongPatternList,
    #[serde(alias = "8B")]
    pub eight_buttons: VArchiveSongPatternList,
}

/// Legacy struct (which will be removed.)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VArchiveSong {
    pub title: usize,
    pub name: String,
    pub composer: String,
    pub dlc_code: String,
    pub dlc: String,
    pub patterns: VArchiveSongPatternTable,
}

pub fn all_songs() -> Result<Vec<VArchiveSong>, APIError> {
    let resp = ureq::get("https://v-archive.net/db/songs.json").call();

    match resp {
        Ok(resp) => {
            let resp_str = resp.into_string().unwrap();
            Ok(serde_json::from_str(&resp_str).unwrap())
        }
        Err(Error::Status(code, resp)) => Err(catch_server_err(code, resp)),
        Err(_) => Err(APIError::UnknownError),
    }
}

pub fn tier_list() -> Result<Vec<VArchiveTier>, APIError> {
    let resp = ureq::get("https://v-archive.net/db/tiers.json").call();

    match resp {
        Ok(resp) => {
            let resp_str = resp.into_string().unwrap();
            Ok(serde_json::from_str(&resp_str).unwrap())
        }
        Err(Error::Status(code, resp)) => Err(catch_server_err(code, resp)),
        Err(_) => Err(APIError::UnknownError),
    }
}

pub fn board_types() -> Result<Vec<String>, APIError> {
    let resp = ureq::get("https://v-archive.net/db/boards.json").call();

    match resp {
        Ok(resp) => {
            let resp_str = resp.into_string().unwrap();
            Ok(serde_json::from_str(&resp_str).unwrap())
        }
        Err(Error::Status(code, resp)) => Err(catch_server_err(code, resp)),
        Err(_) => Err(APIError::UnknownError),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_available_buttons() {
        // Loading tier info; as "10" buttons(which is **not available**) tier on DEV
        let example_username = "DEV";
        let load_user_tier = VArchiveUserTierInfo::load_user_tier(example_username, 10);

        match load_user_tier {
            Ok(_) => {
                panic!("this should be become to error.")
            }
            Err(e) => {
                match e {
                    APIError::WrongParameter(m) => {
                        assert_eq!(m, "버튼 찾을 수 없음".to_owned());
                    }
                    another_e => {
                        panic!("It raised another error: {:?}", another_e);
                    }
                };
            }
        };
    }

    #[test]
    fn check_no_data() {
        // Loading tier info; as 4 buttons tier on "no_data"
        let example_username = "no_data";
        let load_user_tier = VArchiveUserTierInfo::load_user_tier(&example_username, 4);

        match load_user_tier {
            Ok(_) => {
                panic!("this should be become to error.")
            }
            Err(e) => {
                match e {
                    APIError::HasNoButtonRecord => {}
                    another_e => {
                        panic!("It raised another error: {:?}", another_e);
                    }
                };
            }
        };
    }

    #[test]
    fn check_no_user() {
        // Loading tier info; as 4 buttons tier on "no_account"
        let example_username = "no_account";
        let load_user_tier = VArchiveUserTierInfo::load_user_tier(&example_username, 4);

        match load_user_tier {
            Ok(_) => {
                panic!("this should be become to error.")
            }
            Err(e) => match e {
                APIError::CannotFindUser => {}
                another_e => {
                    panic!("It raised another error: {:?}", another_e);
                }
            },
        };
    }

    #[test]
    fn tier_convert() {
        let tier_point = 7028;
        let tier = VArchiveTier::from_point(tier_point);

        assert_eq!(tier.rating, 7000);
        assert_eq!(tier.name, "Silver II".to_string());
        assert_eq!(tier.code, "SV".to_string());
    }

    #[test]
    fn tier_info_load() {
        // Loading tier info; as 4 buttons tier on "DEV"
        let example_username = "DEV";
        let load_user_tier = VArchiveUserTierInfo::load_user_tier(&example_username, 4);

        match load_user_tier {
            Ok(info) => {
                assert_eq!(info.success, true);
            }
            Err(_) => {
                panic!("not successed to load user tier info")
            }
        };
    }

    #[test]
    fn get_user_song_info() {
        let example_username = "내꺼";
        let song_result = VArchiveSongUserResult::load_song_result(example_username, &555);

        match song_result {
            Ok(r) => {
                assert_eq!(r.success, true);
                assert_eq!(r.title, 555);
                // assert_eq!(r.song_id, 555); -- Maybe does later
                assert_eq!(r.name, "Gloxinia".to_string());
                assert_eq!(r.composer, "Ruxxi, Milkoi".to_string());
                assert_eq!(r.dlc_code, "VE4".to_string());
                assert_eq!(r.patterns.four_buttons.normal.level, 5);
            }
            Err(e) => {
                panic!("it has error: {}", e.to_string())
            }
        };
    }

    #[test]
    fn get_user_board() {
        let example_username = "내꺼";
        let user_board_resp = VArciveUserBoard::load_user_board(example_username, &6, "MX");

        match user_board_resp {
            Ok(board) => {
                assert_eq!(board.success, true);
                assert_eq!(board.board_type, "MX".to_string());
                assert_eq!(board.button, 6);
            }
            Err(e) => {
                panic!("it has error: {}", e)
            }
        };
    }

    #[test]
    fn register_record() {
        let user = VArchiveUserToken {
            user_num: 1,
            user_token: "95d6c422-52b4-4016-8587-38c46a2e7917".to_string(),
        };

        let play_record = VArchiveRegisterRecord {
            name: "Urban Night".to_string(),
            dlc: "EMOTIONAL S.".to_string(),
            composer: "Electronic Boutique".to_string(),
            button: 6,
            pattern: "SC".to_string(),
            score: 90.9,
            max_combo: 0,
        };

        let req = user.register_record(play_record);

        match req {
            Ok(r) => assert_eq!(r.success, true),
            Err(e) => {
                panic!("it has error: {}", e)
            }
        };
    }

    #[test]
    fn load_all_songs() {
        let song_list_resp = all_songs();

        match song_list_resp {
            Ok(list) => {
                let first_song = &list[0];
                assert_eq!(first_song.name, "비상 ~Stay With Me~".to_string());
                assert_eq!(first_song.composer, "Mycin.T".to_string());
                assert_eq!(first_song.dlc_code, "R".to_string());
                assert_eq!(first_song.dlc, "RESPECT".to_string());
                assert_eq!(first_song.patterns.four_buttons.normal.level, 4);
            }
            Err(e) => {
                panic!("it has error: {}", e)
            }
        };
    }
}
