use serde::{Deserialize, Serialize};
use serde_this_or_that::{as_bool, as_f64};
use std::convert::From;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;
use ureq::{Error, Response};

/// An API raw struct for tier
#[derive(Deserialize)]
struct RawAPITier {
    pub rating: u32,
    pub name: String,
    pub code: String,
}

/// This is using for a lot of errors from V-Archive sever. Mostly, it comes as `Result<_, APIError>`
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
            Self::CannotFindUser => write!(f, "Cannot find user"),
            Self::HasNoButtonRecord => write!(f, "Has no button record"),
            Self::CannotFindSong => write!(f, "Cannot find song"),
            Self::FoundSeveralSongs => write!(f, "Found several songs"),
            Self::CannotFoundChart => write!(f, "Cannot find chart"),
            Self::WrongParameter(m) => write!(f, "Wrong parameter(s): {}", m),
            Self::InernalServerError => write!(f, "Inernal server error"),
            Self::APIUnknownError(c, m) => write!(f, "Unknown API error: {}, {}", c, m),
            Self::HTTPErr(c) => write!(f, "HTTP error: {}", c),
            Self::UnknownError => write!(f, "Unknown"),
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

/// Cartegories for new initial contents of DJMAX RESPECT or DMRV
#[derive(Debug)]
pub enum RespectCat {
    Respect,
    RespectV,
}

/// Cartegories for legacy initial contents of DJMAX RESPECT or DMRV
#[derive(Debug)]
pub enum LegacyCat {
    PortableOne,
    PortableTwo,
}

/// Cartegories for DLCs of legacy DJMAX series
#[derive(Debug)]
pub enum LegacyExtCat {
    Trilogy,
    Clazziquai,
    TechnikaOne,
    BlackSquare,
    TechnikaTwo,
    TechnikaThree,
    EmotionalSense,
    PortableThree,
    TechnikaTuneQ,
}

/// Cartegories for DLCs of new contents of DJMAX RESPECT V
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

/// Cartegories for a song
#[derive(Debug)]
pub enum SongCatagory {
    Respect(RespectCat),
    Legacy(LegacyCat),
    LegacyExtention(LegacyExtCat),
    NewExtention(NewExtCat),
    Pli(u8),
    Collab(String),
    Others(String),
}

impl From<&str> for SongCatagory {
    fn from(idfinder: &str) -> Self {
        match idfinder {
            // RESPECT V default songs
            "R" => Self::Respect(RespectCat::Respect),
            "P1" => Self::Legacy(LegacyCat::PortableOne),
            "P2" => Self::Legacy(LegacyCat::PortableTwo),
            // Legacy DLC series
            "ES" => Self::LegacyExtention(LegacyExtCat::EmotionalSense),
            "TR" => Self::LegacyExtention(LegacyExtCat::Trilogy),
            "BS" => Self::LegacyExtention(LegacyExtCat::BlackSquare),
            "CE" => Self::LegacyExtention(LegacyExtCat::Clazziquai),
            "T3" => Self::LegacyExtention(LegacyExtCat::TechnikaThree),
            "T2" => Self::LegacyExtention(LegacyExtCat::TechnikaTwo),
            "T1" => Self::LegacyExtention(LegacyExtCat::TechnikaOne),
            "P3" => Self::LegacyExtention(LegacyExtCat::PortableThree),
            "TQ" => Self::LegacyExtention(LegacyExtCat::TechnikaTuneQ),
            // New original DLC series
            "VE" => Self::NewExtention(NewExtCat::VExtentionOne),
            "VE2" => Self::NewExtention(NewExtCat::VExtentionTwo),
            "VE3" => Self::NewExtention(NewExtCat::VExtentionThree),
            "VE4" => Self::NewExtention(NewExtCat::VExtentionFour),
            "VE5" => Self::NewExtention(NewExtCat::VExtentionFive),
            "VL" => Self::NewExtention(NewExtCat::VLivertyOne),
            "VL2" => Self::NewExtention(NewExtCat::VLivertyTwo),
            // PLI extention
            "PLI1" => Self::Pli(1),
            // Collab DLC
            "GG" | "GC" | "CY" | "CHU" | "ESTI" | "NXN" | "MD" | "EZ2" | "MAP" | "FAL" | "TEK" => {
                Self::Collab(idfinder.to_owned())
            }
            _ => Self::Others(idfinder.to_owned()),
        }
    }
}

/// Button modes for a chart
#[repr(u8)]
#[derive(Debug)]
pub enum ButtonMode {
    Four,
    Five,
    Six,
    Eight,
    Other(u8),
}

impl ButtonMode {
    pub fn new() -> Self {
        Self::Four
    }
}

impl From<u8> for ButtonMode {
    fn from(button: u8) -> Self {
        match button {
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            8 => Self::Eight,
            b => Self::Other(b),
        }
    }
}

impl FromStr for ButtonMode {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, ParseIntError> {
        match s {
            "4" => Ok(Self::Four),
            "5" => Ok(Self::Five),
            "6" => Ok(Self::Six),
            "8" => Ok(Self::Eight),
            b => {
                let parse_button = b.parse::<u8>();
                match parse_button {
                    Ok(n) => Ok(Self::Other(n)),
                    Err(error) => Err(error),
                }
            }
        }
    }
}

impl fmt::Display for ButtonMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Four => write!(f, "4"),
            Self::Five => write!(f, "5"),
            Self::Six => write!(f, "6"),
            Self::Eight => write!(f, "8"),
            Self::Other(b) => write!(f, "{}", b),
        }
    }
}

/// Difficulty types for a chart
#[derive(Debug)]
pub enum ChartType {
    Normal,
    Hard,
    Maximum,
    Sc,
    Other(String),
}

impl From<&str> for ChartType {
    fn from(name: &str) -> Self {
        match name {
            "NM" => ChartType::Normal,
            "HD" => ChartType::Hard,
            "MX" => ChartType::Maximum,
            "SC" => ChartType::Sc,
            other => ChartType::Other(other.to_owned()),
        }
    }
}

impl fmt::Display for ChartType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Normal => write!(f, "NM"),
            Self::Hard => write!(f, "HD"),
            Self::Maximum => write!(f, "MX"),
            Self::Sc => write!(f, "SC"),
            Self::Other(t) => write!(f, "{}", t),
        }
    }
}

/// A struct for user's record for a chart
#[derive(Debug)]
pub struct UserChartRecord {
    /// ID number for a song of the chart
    pub song_id: usize,
    /// A song title for the chart
    pub title: String,
    /// A button type of the chart
    pub button: ButtonMode,
    /// A difficulty type of the chart (e. g.: Normal, Hard, Maximum, SC)
    pub chart_type: ChartType,
    /// A user's accuracy rate for the chart
    pub acc_rate: Option<f64>,
    /// user's max combo or not for the chart
    pub is_max_combo: bool,
    /// A level for the chart
    pub chart_level: Option<u8>,
    /// A level on V-Archive's floor
    pub floor_level: f64,
    /// A user's rating on V-Archive for a chart
    pub user_rating: f64,
    /// A maximum rating on V-Archive for a chart
    pub maximum_rating: Option<f64>,
    /// A DJPOWER point for DJMAX. (This may differ from in-game.)
    pub dj_power: Option<f64>,
    /// A category for a song of the chart
    pub song_cat: Option<SongCatagory>,
}

impl UserChartRecord {
    pub fn new() -> Self {
        Self {
            song_id: 0,
            title: String::new(),
            button: ButtonMode::Four,
            chart_type: ChartType::Normal,
            acc_rate: None,
            is_max_combo: false,
            chart_level: None,
            floor_level: 0.0,
            user_rating: 0.0,
            maximum_rating: None,
            dj_power: None,
            song_cat: None,
        }
    }
}

/// Tier enum for a table
#[derive(Debug)]
pub enum Tier {
    Beginner(u16),
    AmateurIV(u16),
    AmateurIII(u16),
    AmateurII(u16),
    AmateurI(u16),
    IronIV(u16),
    IronIII(u16),
    IronII(u16),
    IronI(u16),
    BronzeIV(u16),
    BronzeIII(u16),
    BronzeII(u16),
    BronzeI(u16),
    SilverIV(u16),
    SilverIII(u16),
    SilverII(u16),
    SilverI(u16),
    GoldIV(u16),
    GoldIII(u16),
    GoldII(u16),
    GoldI(u16),
    PlatinumIV(u16),
    PlatinumIII(u16),
    PlatinumII(u16),
    PlatinumI(u16),
    DiamondIV(u16),
    DiamondIII(u16),
    DiamondII(u16),
    DiamondI(u16),
    MasterIII(u16),
    MasterII(u16),
    MasterI(u16),
    GrandMaster(u16),
}

impl Tier {
    pub fn new() -> Self {
        Self::Beginner(0)
    }

    /// Points to tier enum
    pub fn from(points: u16) -> Self {
        match points {
            0..=499 => Self::Beginner(0),
            500..=999 => Self::AmateurIV(500),
            1000..=1999 => Self::AmateurIII(1000),
            2000..=2999 => Self::AmateurII(2000),
            3000..=3999 => Self::AmateurI(3000),
            4000..=4299 => Self::IronIV(4000),
            4300..=4599 => Self::IronIII(4300),
            4600..=4899 => Self::IronII(4600),
            4900..=5299 => Self::IronI(4900),
            5300..=5649 => Self::BronzeIV(5300),
            5650..=5999 => Self::BronzeIII(5650),
            6000..=6299 => Self::BronzeII(6000),
            6300..=6599 => Self::BronzeI(6300),
            6600..=6799 => Self::SilverIV(6600),
            6800..=6999 => Self::SilverIII(6800),
            7000..=7199 => Self::SilverII(7000),
            7200..=7399 => Self::SilverI(7200),
            7400..=7599 => Self::GoldIV(7400),
            7600..=7799 => Self::GoldIII(7600),
            7800..=7999 => Self::GoldII(7800),
            8000..=8199 => Self::GoldI(8000),
            8200..=8399 => Self::PlatinumIV(8200),
            8400..=8599 => Self::PlatinumIII(8400),
            8600..=8799 => Self::PlatinumII(8600),
            8800..=8999 => Self::PlatinumI(8800),
            9000..=9199 => Self::DiamondIV(9000),
            9200..=9399 => Self::DiamondIII(9200),
            9400..=9599 => Self::DiamondII(9400),
            9600..=9699 => Self::DiamondI(9600),
            9700..=9799 => Self::MasterIII(9700),
            9800..=9899 => Self::MasterII(9800),
            9900..=9945 => Self::MasterI(9900),
            _ => Self::GrandMaster(9950),
        }
    }
}

impl fmt::Display for Tier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Beginner(_) => write!(f, "Beginner"),
            Self::AmateurIV(_) => write!(f, "Amateur IV"),
            Self::AmateurIII(_) => write!(f, "Amateur III"),
            Self::AmateurII(_) => write!(f, "Amateur II"),
            Self::AmateurI(_) => write!(f, "Amateur I"),
            Self::IronIV(_) => write!(f, "Iron IV"),
            Self::IronIII(_) => write!(f, "Iron III"),
            Self::IronII(_) => write!(f, "Iron II"),
            Self::IronI(_) => write!(f, "Iron I"),
            Self::BronzeIV(_) => write!(f, "Bronze IV"),
            Self::BronzeIII(_) => write!(f, "Bronze III"),
            Self::BronzeII(_) => write!(f, "Bronze II"),
            Self::BronzeI(_) => write!(f, "Bronze I"),
            Self::SilverIV(_) => write!(f, "Silver IV"),
            Self::SilverIII(_) => write!(f, "Silver III"),
            Self::SilverII(_) => write!(f, "Silver II"),
            Self::SilverI(_) => write!(f, "Silver I"),
            Self::GoldIV(_) => write!(f, "Gold IV"),
            Self::GoldIII(_) => write!(f, "Gold III"),
            Self::GoldII(_) => write!(f, "Gold II"),
            Self::GoldI(_) => write!(f, "Gold I"),
            Self::PlatinumIV(_) => write!(f, "Platinum IV"),
            Self::PlatinumIII(_) => write!(f, "Platinum III"),
            Self::PlatinumII(_) => write!(f, "Platinum II"),
            Self::PlatinumI(_) => write!(f, "Platinum I"),
            Self::DiamondIV(_) => write!(f, "Diamond IV"),
            Self::DiamondIII(_) => write!(f, "Diamond III"),
            Self::DiamondII(_) => write!(f, "Diamond II"),
            Self::DiamondI(_) => write!(f, "Diamond I"),
            Self::MasterIII(_) => write!(f, "Master III"),
            Self::MasterII(_) => write!(f, "Master II"),
            Self::MasterI(_) => write!(f, "Master I"),
            Self::GrandMaster(_) => write!(f, "Grand Master"),
        }
    }
}

/// A user's record table with V-Archive tier.
#[derive(Debug)]
pub struct UserTierRecordTable {
    pub fifteen_sum: f64,
    pub tier_point: f64,
    pub current_tier: Tier,
    pub next_tier: Tier,
    pub top_records: Vec<UserChartRecord>,
}

impl UserTierRecordTable {
    pub fn new() -> Self {
        Self {
            fifteen_sum: 0.0,
            tier_point: 0.0,
            current_tier: Tier::from(0),
            next_tier: Tier::from(0),
            top_records: Vec::new(),
        }
    }
}

fn load_user_tier_parse(parse_text: String) -> UserTierRecordTable {
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct APIPlayRecord {
        // ID number for a song of the chart
        title: usize,
        // A song title of the chart
        name: String,
        // A button type of the chart
        button: u8,
        // A difficulty type of the chart
        pattern: String,
        // A level number of the chart
        level: u8,
        // A floor level from V-Archive of the chart
        floor: String,
        // Maximum rating of the pattern
        max_rating: String,
        // The user's accuracy rate of the chart
        score: Option<String>,
        // The user's MAX COMBO
        #[serde(deserialize_with = "as_bool")]
        max_combo: bool,
        // The user's rating of the chart
        rating: String,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct APIBody {
        success: bool,
        top50sum: f64,
        tier_point: f64,
        tier: RawAPITier,
        next: RawAPITier,
        top_list: Vec<APIPlayRecord>,
    }

    let api_body: APIBody = serde_json::from_str(&parse_text).unwrap();

    let mut top_list: Vec<UserChartRecord> = Vec::new();
    for record in api_body.top_list {
        let mut user_record = UserChartRecord::new();

        user_record.song_id = record.title;
        user_record.title = record.name;
        user_record.button = ButtonMode::from(record.button);
        user_record.chart_type = ChartType::from(record.pattern.as_str());
        user_record.is_max_combo = record.max_combo;
        user_record.chart_level = Some(record.level);
        user_record.floor_level = record.floor.parse().unwrap();
        user_record.user_rating = record.rating.parse().unwrap();
        user_record.maximum_rating = Some(record.max_rating.parse().unwrap());

        user_record.acc_rate = match record.score {
            None => None,
            Some(s) => Some(s.parse().unwrap()),
        };

        top_list.push(user_record);
    }

    let mut user_record_table = UserTierRecordTable::new();

    user_record_table.fifteen_sum = api_body.top50sum;
    user_record_table.tier_point = api_body.tier_point;
    user_record_table.current_tier = Tier::from(api_body.tier.rating as u16);
    user_record_table.next_tier = Tier::from(api_body.next.rating as u16);
    user_record_table.top_records = top_list;

    user_record_table
}

/// Load a user's tier info from server
/// ## Example
/// ```rust
/// # use v_archive_rs::load_user_tier;
/// #
/// # fn main() {
/// # // Starts for showing code
/// let username = "내꺼";
/// let tier_record = load_user_tier(username, 6);
///
/// match tier_record {
///     Ok(r) => {
///         println!(
///             "Success: {}'s Tier is {}",
///             username,
///             r.current_tier.to_string()
///         );
///     }
///     Err(e) => {
///         println!("Load failed: {:?}", e);
///     }
/// }
/// # // Ends for showing code
/// # }
/// ```
pub fn load_user_tier(username: &str, buttons: u8) -> Result<UserTierRecordTable, APIError> {
    let get_url = format!("https://v-archive.net/api/archive/{username}/tier/{buttons}");
    let resp = ureq::get(&get_url)
        .set("Content-Type", "application/json")
        .call();

    match resp {
        Ok(resp) => {
            let resp_str = resp.into_string().unwrap();
            Ok(load_user_tier_parse(resp_str))
        }
        Err(Error::Status(code, resp)) => Err(catch_server_err(code, resp)),
        Err(_) => Err(APIError::UnknownError),
    }
}

#[derive(Debug)]
pub struct UserFloorRecord {
    /// A number of floor
    pub floor_number: f64,
    /// A bunch of records
    pub records: Vec<UserChartRecord>,
}

impl UserFloorRecord {
    pub fn new() -> Self {
        Self {
            floor_number: 0.0,
            records: Vec::new(),
        }
    }
}

/// Types of user's record floor board.
#[derive(Debug)]
pub enum FloorBoardType {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Maximum,
    Sc,
    ScFive,
    ScTen,
    ScFifteen,
    Djpower,
    Others(String),
}

impl FloorBoardType {
    pub fn new() -> Self {
        FloorBoardType::One
    }
}

impl From<&str> for FloorBoardType {
    fn from(idfinder: &str) -> Self {
        match idfinder {
            "1" => Self::One,
            "2" => Self::Two,
            "3" => Self::Three,
            "4" => Self::Four,
            "5" => Self::Five,
            "6" => Self::Six,
            "7" => Self::Seven,
            "8" => Self::Eight,
            "9" => Self::Nine,
            "10" => Self::Ten,
            "11" => Self::Eleven,
            "MX" => Self::Maximum,
            "SC" => Self::Sc,
            "SC5" => Self::ScFive,
            "SC10" => Self::ScTen,
            "SC15" => Self::ScFifteen,
            "DJPOWER" => Self::Djpower,
            _ => Self::Others(idfinder.to_owned()),
        }
    }
}

impl From<usize> for FloorBoardType {
    fn from(index: usize) -> Self {
        match index {
            0 => Self::One,
            1 => Self::Two,
            2 => Self::Three,
            3 => Self::Four,
            4 => Self::Five,
            5 => Self::Six,
            6 => Self::Seven,
            7 => Self::Eight,
            8 => Self::Nine,
            9 => Self::Ten,
            10 => Self::Eleven,
            11 => Self::Maximum,
            12 => Self::Sc,
            13 => Self::ScFive,
            14 => Self::ScTen,
            15 => Self::ScFifteen,
            16 => Self::Djpower,
            _ => Self::Others(index.to_string()),
        }
    }
}

impl fmt::Display for FloorBoardType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::One => write!(f, "1"),
            Self::Two => write!(f, "2"),
            Self::Three => write!(f, "3"),
            Self::Four => write!(f, "4"),
            Self::Five => write!(f, "5"),
            Self::Six => write!(f, "6"),
            Self::Seven => write!(f, "7"),
            Self::Eight => write!(f, "8"),
            Self::Nine => write!(f, "9"),
            Self::Ten => write!(f, "10"),
            Self::Eleven => write!(f, "11"),
            Self::Maximum => write!(f, "MX"),
            Self::Sc => write!(f, "SC"),
            Self::ScFive => write!(f, "SC5"),
            Self::ScTen => write!(f, "SC10"),
            Self::ScFifteen => write!(f, "SC15"),
            Self::Djpower => write!(f, "DJPOWER"),
            Self::Others(t) => write!(f, "{}", t),
        }
    }
}

#[derive(Debug)]
pub struct UserFloorRecordBoard {
    /// A type of the board
    pub board_type: FloorBoardType,
    /// A button type of the board
    pub button: ButtonMode,
    /// Numbers of records on the board
    pub total_count: usize,
    /// List of floors with records
    pub floors: Vec<UserFloorRecord>,
}

impl UserFloorRecordBoard {
    pub fn new() -> Self {
        Self {
            board_type: FloorBoardType::new(),
            button: ButtonMode::new(),
            total_count: 0,
            floors: Vec::new(),
        }
    }
}

fn user_floor_board_parse(parse_text: String) -> UserFloorRecordBoard {
    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct APIPlayRecord {
        title: usize,
        name: String,
        composer: String,
        pattern: String,
        score: Option<String>,
        #[serde(deserialize_with = "as_bool")]
        max_combo: bool,
        djpower: f64,
        rating: f64,
        dlc: String,
        dlc_code: String,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct APIFloorLevel {
        floor_number: f64,
        patterns: Vec<APIPlayRecord>,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct APIBody {
        success: bool,
        board: String,
        button: String,
        total_count: usize,
        floors: Vec<APIFloorLevel>,
    }

    let api_body: APIBody = serde_json::from_str(&parse_text).unwrap();

    let mut floors = Vec::new();

    for api_floor in api_body.floors {
        let mut floor = UserFloorRecord::new();
        floor.floor_number = api_floor.floor_number;

        let mut user_records = Vec::new();

        for record in api_floor.patterns {
            let mut user_record = UserChartRecord::new();

            user_record.song_id = record.title;
            user_record.title = record.name;
            user_record.button = ButtonMode::from_str(&api_body.button).unwrap();
            user_record.chart_type = ChartType::from(record.pattern.as_str());
            user_record.is_max_combo = record.max_combo;
            user_record.floor_level = api_floor.floor_number;
            user_record.user_rating = record.rating;
            user_record.dj_power = Some(record.djpower);
            user_record.song_cat = Some(SongCatagory::from(record.dlc_code.as_str()));

            user_record.acc_rate = match record.score {
                None => None,
                Some(s) => Some(s.parse().unwrap()),
            };

            user_records.push(user_record);
        }
        floor.records = user_records;

        floors.push(floor);
    }

    let mut floor_board = UserFloorRecordBoard::new();
    floor_board.board_type = FloorBoardType::from(api_body.board.as_str());
    floor_board.button = ButtonMode::from_str(&api_body.button).unwrap();
    floor_board.total_count = api_body.total_count;
    floor_board.floors = floors;

    floor_board
}

/// Load a user's floor board from server
/// ## Example
/// ```rust
/// # use v_archive_rs::load_user_floor_board;
/// #
/// # fn main() {
/// # // Starts for showing code
/// let username = "내꺼";
/// let floor_board = load_user_floor_board(username, 6, "MX");
///
/// match floor_board {
///     Ok(b) => {
///         println!("Board type: {}", b.board_type.to_string());
///         println!("Board buttons mode: {} buttons", b.button.to_string());
///         println!("Board total count: {}", b.total_count);
///     }
///     Err(e) => {
///         println!("Load failed: {:?}", e);
///     }
/// }
/// # // Ends for showing code
/// # }
/// ```
pub fn load_user_floor_board(
    username: &str,
    buttons: u8,
    board_type: &str,
) -> Result<UserFloorRecordBoard, APIError> {
    let get_url =
        format!("https://v-archive.net/api/archive/{username}/board/{buttons}/{board_type}");
    let resp = ureq::get(&get_url)
        .set("Content-Type", "application/json")
        .call();

    match resp {
        Ok(resp) => {
            let resp_str = resp.into_string().unwrap();
            Ok(user_floor_board_parse(resp_str))
        }
        Err(Error::Status(code, resp)) => Err(catch_server_err(code, resp)),
        Err(_) => Err(APIError::UnknownError),
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

// /// Legacy struct (which will be removed.)
// #[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct VArchiveFloorSongResult {
//     pub title: usize,
//     pub name: String,
//     pub composer: String,
//     pub pattern: String,
//     #[serde(deserialize_with = "as_f64")]
//     pub score: f64,
//     #[serde(deserialize_with = "as_bool")]
//     pub max_combo: bool,
//     pub djpower: f64,
//     pub rating: f64,
//     pub dlc: String,
//     pub dlc_code: String,
// }

// impl VArchiveFloorSongResult {
//     pub fn new() -> Self {
//         Self {
//             title: 0,
//             name: String::new(),
//             composer: String::new(),
//             pattern: String::new(),
//             score: 0.0,
//             max_combo: false,
//             djpower: 0.0,
//             rating: 0.0,
//             dlc: String::new(),
//             dlc_code: String::new(),
//         }
//     }
// }

// /// Legacy struct (which will be removed.)
// #[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct VArchiveUserFloor {
//     pub floor_number: f64,
//     pub patterns: Vec<VArchiveFloorSongResult>,
// }

// impl VArchiveUserFloor {
//     pub fn new() -> Self {
//         Self {
//             floor_number: 0.0,
//             patterns: Vec::new(),
//         }
//     }
// }

// /// Legacy struct (which will be removed.)
// #[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct VArciveUserBoard {
//     success: bool,
//     #[serde(alias = "board")]
//     pub board_type: String,
//     #[serde(deserialize_with = "as_u64")]
//     pub button: u64,
//     pub total_count: usize,
//     pub floors: Vec<VArchiveUserFloor>,
// }

// impl VArciveUserBoard {
//     pub fn new() -> Self {
//         Self {
//             success: true,
//             board_type: String::new(),
//             button: 4,
//             total_count: 0,
//             floors: Vec::new(),
//         }
//     }

//     pub fn load_user_board(
//         username: &str,
//         buttons: &u8,
//         board_type: &str,
//     ) -> Result<Self, APIError> {
//         let get_url =
//             format!("https://v-archive.net/api/archive/{username}/board/{buttons}/{board_type}");
//         let resp = ureq::get(&get_url)
//             .set("Content-Type", "application/json")
//             .call();

//         match resp {
//             Ok(resp) => {
//                 let resp_str = resp.into_string().unwrap();
//                 Ok(serde_json::from_str(&resp_str).unwrap())
//             }
//             Err(Error::Status(code, resp)) => Err(catch_server_err(code, resp)),
//             Err(_) => Err(APIError::UnknownError),
//         }
//     }
// }

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

fn tier_list_parse(parse_text: String) -> Vec<Tier> {
    let api_tier_list: Vec<RawAPITier> = serde_json::from_str(&parse_text).unwrap();
    let mut tier_list: Vec<Tier> = Vec::new();

    for t in api_tier_list {
        let tier_converted = Tier::from(t.rating as u16);
        tier_list.push(tier_converted);
    }

    tier_list
}

pub fn tier_list() -> Result<Vec<Tier>, APIError> {
    let resp = ureq::get("https://v-archive.net/db/tiers.json").call();

    match resp {
        Ok(resp) => {
            let resp_str = resp.into_string().unwrap();
            Ok(tier_list_parse(resp_str))
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
        let load_user_tier = load_user_tier(example_username, 10);

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
        let load_user_tier = load_user_tier(&example_username, 4);

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
        let load_user_tier = load_user_tier(&example_username, 4);

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
        let tier = Tier::from(tier_point);

        match tier {
            Tier::SilverII(pt) => {
                assert_eq!(pt, 7000);
                assert_eq!(tier.to_string(), "Silver II".to_string());
            }
            t => panic!(
                "It does not converted to right tier (it should be \"Silver II\"): {}",
                t.to_string()
            ),
        }
    }

    #[test]
    fn tier_info_load() {
        // Loading tier info; as 4 buttons tier on "DEV"
        let example_username = "DEV";
        let load_user_tier = load_user_tier(&example_username, 4);

        match load_user_tier {
            Ok(_) => {}
            Err(e) => {
                panic!("not successed to load user tier info: {}", e.to_string())
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
            Err(e) => panic!("it has error: {}", e.to_string()),
        };
    }

    #[test]
    fn get_user_board() {
        let example_username = "내꺼";
        let user_board_resp = load_user_floor_board(example_username, 6, "MX");

        match user_board_resp {
            Ok(board) => {
                assert_eq!(board.board_type.to_string(), "MX".to_string());
                match board.button {
                    ButtonMode::Six => {}
                    b => panic!(
                        "It should be 6 button but it's not: {} buttons",
                        b.to_string()
                    ),
                }
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
