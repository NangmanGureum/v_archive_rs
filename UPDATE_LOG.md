* 0.11.0
  * Add to support a collab DLC (`BA`)
  * Change function name: `UserFloorRecord` -> `UserFloorRecordSet`
  * Update `README.md` at Todo-list
* 0.10.2
  * Add more document stuff.
  * Add some examples.
* 0.10.1
  * Fix some error at *Guidelines on the Derivative Works of DJMAX IP*.
* 0.10.0
  * Support for collab DLCs and PLI (e. g. `SongCatagory::Pli(1)`)
  * Add some examples.
  * Add a file: `DJMAX-Derivative-Works-Guidelines.md`
  * Fix Licenses: `MIT` ->  `MIT AND SEE LICENSE IN DJMAX-Derivative-Works-Guidelines.md`
  * Support `to_string()` for `ButtonMode`
  * Update for Docs
* 0.9.0
  * Add new `struct`s
    * `Tier`
    * `UserFloorRecordBoard`
    * `UserFloorRecord`
  * Add new `enum`: `FloorBoardType`
  * Remove `struct`s
    * `VArchivePatternResult` (replaced to `UserChartRecord`)
    * `VArchiveTier` (replaced to `Tier`)
    * `VArchiveUserTierInfo`(replaced to `UserTierRecordTable`)
    * `VArchiveFloorSongResult` (replaced to `UserChartRecord`)
    * `VArchiveUserFloor` (replaced to `UserFloorRecord`)
    * `VArciveUserBoard` (replaced to `UserFloorRecordBoard`)
  * Add new functions
    * `load_user_tier()`
    * `load_user_floor_board()`
* 0.8.0
  * Add new `struct`: `UserTierRecordTable`
  * Change `struct` name: `UserRecordForChart` -> `UserChartRecord`
  * Add new function: `load_user_tier()`
* 0.7.0
  * Remove `struct`: `VArchiveErr`
  * Add new `struct`: `UserRecordForChart` (for replace)
  * Add new `enum`s (note: `cat` stands for 'catagory')
    * `SongCatagory`
    * `RespectCat`
    * `LegacyCat`
    * `LegacyExtCat`
    * `NewExtCat`
    * `ButtonMode`
* 0.6.1
  * Fix `README.md`
* 0.6.0
  * Add new `enum`: `APIError` (for replace)
  * Change function: `VArchiveUserTierInfo::load_user_tier`
    * Return type is replaced to `Result<VArchiveUserTierInfo, APIError>`.
* 0.5.0
  * Support to load list of all songs, tiers, and board types.
  * Fix sturct with serde's `Default`:  `VArchivePatternList`, and related stuffs
* 0.4.0
  * Support to register user record. [#](https://github.com/djmax-in/openapi/wiki/%EA%B8%B0%EB%A1%9D-%EB%93%B1%EB%A1%9D-API)
  * Fix [TODO.md](./TODO.md)
* 0.3.0
  * Add [document URL](https://docs.rs/v_archive_rs) on this crate
  * Add more document stuff (`VArchiveUserTierInfo` and related stuff)
  * Support to load user floor result table. [#](https://github.com/djmax-in/openapi/wiki/%EC%9C%A0%EC%A0%80-%EC%84%B1%EA%B3%BC%ED%91%9C-%EC%A1%B0%ED%9A%8C-API)
  * Add more `README.md` stuff
* 0.2.1
  * Add [repository URL](https://github.com/NangmanGureum/v_archive_rs) on this crate
* 0.2.0
  * Fix `VArchiveUserTierInfo` to public
  * Support to load user data for a song. [#](https://github.com/djmax-in/openapi/wiki/%EC%9C%A0%EC%A0%80-%EA%B3%A1%EB%B3%84-%EA%B8%B0%EB%A1%9D-%EC%A1%B0%ED%9A%8C-API)
* 0.1.0
  * Support to load user data for tier. [#](https://github.com/djmax-in/openapi/wiki/%EC%9C%A0%EC%A0%80-%ED%8B%B0%EC%96%B4-%EC%A1%B0%ED%9A%8C-API)
