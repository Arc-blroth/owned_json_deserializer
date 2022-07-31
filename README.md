# Owned JSON Deserializer

[![Crates.io](https://img.shields.io/crates/v/owned_json_deserializer.svg)](https://crates.io/crates/owned_json_deserializer)
[![Documentation](https://docs.rs/owned_json_deserializer/badge.svg)](https://docs.rs/owned_json_deserializer)
[![License](https://img.shields.io/crates/l/owned_json_deserializer)](https://github.com/Arc-blroth/owned_json_deserializer)
![Dragon Powered](https://img.shields.io/badge/%F0%9F%90%89-dragon%20powered-brightgreen)

Because apparently `serde_json` only deserializes through a reference™.


## Usage

```rust
use serde::{Deserialize, Deserializer};
use owned_json_deserializer::OwnedJsonDeserializer;

#[derive(Deserialize)]
struct Wave {
    hi: String,
}

fn gimme_a_deserializer(say_hi_to: String) -> impl Deserializer<'static> { 
    OwnedJsonDeserializer::from_string(
        format!(r#"{{ "hi": "{}" }}"#, say_hi_to)
    )
}

fn main() {
    let deserializer = gimme_a_deserializer("mom".to_string());
    let wave = Wave::deserialize(deserializer).unwrap();
    assert_eq!(wave.hi, "mom");
}
```


## License

This crate is dual-licensed under either:

- the [Apache License, Version 2.0](LICENSE-APACHE)
- the [MIT license](LICENSE-MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
