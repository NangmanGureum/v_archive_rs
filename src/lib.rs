use serde::{Deserialize, Serialize};
use serde_this_or_that::{as_bool, as_f64};
use ureq::{Error, Response};

/// This is using for a lot of errors from V-Archive sever.
/// Mostly, it comes `Result<_,VArchiveErr>`
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VArchiveErr {
    pub error_code: u16,
    pub message: String,
}

impl VArchiveErr {
    pub fn new() -> Self {
        Self {
            error_code: 0,
            message: String::new(),
        }
    }

    fn catch_server_err(code: u16, resp: Response) -> Self {
        match code {
            400 => {
                let resp_str = resp.into_string().unwrap();
                return serde_json::from_str(&resp_str).unwrap();
            }
            404 => {
                let resp_str = resp.into_string().unwrap();
                return serde_json::from_str(&resp_str).unwrap();
            }
            500 => {
                return VArchiveErr {
                    error_code: 999,
                    message: String::from("Internal Server Error (500)"),
                }
            }
            _ => {
                // println!("Error: Unknown, as code {code}")
                let message = format!("Unknown error from server ({})", code);
                return VArchiveErr {
                    error_code: 999,
                    message,
                };
            }
        };
    }
}

/// This is a user's play result for a song.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VArchiveSongResult {
    pub title: usize,
    pub name: String,
    pub button: u8,
    pub pattern: String,
    pub level: u8,
    #[serde(deserialize_with = "as_f64")]
    pub floor: f64,
    #[serde(deserialize_with = "as_f64")]
    pub max_rating: f64,
    #[serde(deserialize_with = "as_f64")]
    pub score: f64,
    #[serde(deserialize_with = "as_bool")]
    pub max_combo: bool,
    #[serde(deserialize_with = "as_f64")]
    pub rating: f64,
}

impl VArchiveSongResult {
    pub fn new() -> Self {
        VArchiveSongResult {
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

/// This is a tier.
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

    /// Make an tier via server. If it errored, it goes `Load Error`
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

/// This is a user's tier table
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
    pub top_list: Vec<VArchiveSongResult>,
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

    pub fn load_user_tier(username: &str, buttons: &u8) -> Result<Self, VArchiveErr> {
        let get_url = format!("https://v-archive.net/api/archive/{username}/tier/{buttons}");
        let resp = ureq::get(&get_url)
            .set("Content-Type", "application/json")
            .call();

        match resp {
            Ok(resp) => {
                let resp_str = resp.into_string().unwrap();
                Ok(serde_json::from_str(&resp_str).unwrap())
            }
            Err(Error::Status(code, resp)) => Err(VArchiveErr::catch_server_err(code, resp)),
            Err(_) => Err(VArchiveErr {
                error_code: 999,
                message: String::from("Unknown error"),
            }),
        }
    }
}

/// This is a pattern for a song.
#[derive(Serialize, Deserialize, Debug)]
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

/// This is pattern list for a kind of buttons for a song.
#[derive(Serialize, Deserialize, Debug)]
pub struct VArchivePatternList {
    #[serde(alias = "NM")]
    pub normal: VArchivePattern,
    #[serde(alias = "HD")]
    pub hard: VArchivePattern,
    #[serde(alias = "MX")]
    pub maximum: VArchivePattern,
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

/// This is a user's song result
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

    pub fn load_song_result(username: &str, song_id: &usize) -> Result<Self, VArchiveErr> {
        let get_url = format!("https://v-archive.net/api/archive/{username}/title/{song_id}");
        let resp = ureq::get(&get_url)
            .set("Content-Type", "application/json")
            .call();

        match resp {
            Ok(resp) => {
                let resp_str = resp.into_string().unwrap();
                Ok(serde_json::from_str(&resp_str).unwrap())
            }
            Err(Error::Status(code, resp)) => Err(VArchiveErr::catch_server_err(code, resp)),
            Err(_) => Err(VArchiveErr {
                error_code: 999,
                message: String::from("Unknown error"),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tier_info_load() {
        // Loading tier info; as 4 buttons tier on "DEV"
        let example_username = "DEV";
        let load_user_tier = VArchiveUserTierInfo::load_user_tier(&example_username, &4);

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
    fn not_available_buttons() {
        // Loading tier info; as "10" buttons(which is **not available**) tier on DEV
        let example_username = "DEV";
        let load_user_tier = VArchiveUserTierInfo::load_user_tier(&example_username, &10);

        match load_user_tier {
            Ok(_) => {
                panic!("this should be become to error.")
            }
            Err(e) => {
                // Error code 900 means worng parameter; include "not avaliable buttons".
                assert_eq!(e.error_code, 900);
            }
        };
    }

    #[test]
    fn check_no_data() {
        // Loading tier info; as 4 buttons tier on "no_data"
        let example_username = "no_data";
        let load_user_tier = VArchiveUserTierInfo::load_user_tier(&example_username, &4);

        match load_user_tier {
            Ok(_) => {
                panic!("this should be become to error.")
            }
            Err(e) => {
                // Error code 111 means they have no 4 buttons data.
                assert_eq!(e.error_code, 111);
            }
        };
    }

    #[test]
    fn check_no_user() {
        // Loading tier info; as 4 buttons tier on "no_account"
        let example_username = "no_account";
        let load_user_tier = VArchiveUserTierInfo::load_user_tier(&example_username, &4);

        match load_user_tier {
            Ok(_) => {
                panic!("this should be become to error.")
            }
            Err(e) => {
                // Error code 101 means cannot find user.
                assert_eq!(e.error_code, 101);
            }
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
                panic!("it has error: {},{}", e.error_code, e.message)
            }
        };
    }
}
