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
