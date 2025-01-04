# v_arch_rs

This is Rust library for using [V-Archive](https://v-archive.net/). It uses [their API](https://v-archive.net/info/api)

## Example
```rust
use v_archive_rs::VArchiveUserTierInfo as UserTier;

fn main() {
    let username = "내꺼";
    let user_tier = UserTier::load_user_tier(username, &6).unwrap();

    println!("{}'s tier is: {}", username, user_tier.tier.name);
}
```

## Update log
* 0.2.1
  * Add [Repository URL](https://github.com/NangmanGureum/v_archive_rs) on this crate
* 0.2.0
  * Fix `VArchiveUserTierInfo` to public
  * Support to load user data for a song. [#](https://github.com/djmax-in/openapi/wiki/%EC%9C%A0%EC%A0%80-%EA%B3%A1%EB%B3%84-%EA%B8%B0%EB%A1%9D-%EC%A1%B0%ED%9A%8C-API)
* 0.1.0
  * Support to load user data for tier. [#](https://github.com/djmax-in/openapi/wiki/%EC%9C%A0%EC%A0%80-%ED%8B%B0%EC%96%B4-%EC%A1%B0%ED%9A%8C-API)

## List of does not support (also goal)
* Register a play result to V-Archive
* Load whole of song list
* Load kind of song difficulty table
* Load user results table
