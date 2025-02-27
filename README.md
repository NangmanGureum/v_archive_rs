# v_archive_rs
[<img alt="github" src="https://img.shields.io/badge/github-source-8da0cb?style=for-the-badge&logo=github" height="22">](https://github.com/NangmanGureum/v_archive_rs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/v_archive_rs?style=for-the-badge" height="22">](https://crates.io/crates/v_archive_rs)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/v_archive_rs?style=for-the-badge" height="22">](https://docs.rs/v_archive_rs/)

This is Rust library for using [V-Archive](https://v-archive.net/). It uses [their API](https://v-archive.net/info/api)

## Example
```rust
use v_archive_rs::load_user_tier;

fn main() {
    let username = "내꺼";
    let user_tier = load_user_tier(username, 6).unwrap();

    println!("{}'s tier is: {}", username, user_tier.current_tier.to_string());
}
```

See more examples to [`examples`](./examples)

## Update log
See [UPDATE_LOG.md](./UPDATE_LOG.md)

## License
Most of codes follow [MIT License](https://opensource.org/license/mit). But, some parts related to *DJMAX RESPECT V* (such as `SongCatagory` enum) follow [Guidelines on the Derivative Works of DJMAX IP](./DJMAX-Derivative-Works-Guidelines.md).

`SPDX-License-Identifier: MIT AND SEE LICENSE IN DJMAX-Derivative-Works-Guidelines.md`
