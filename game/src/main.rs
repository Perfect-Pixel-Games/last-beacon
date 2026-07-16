//! Thin binary wrapper for the LastBeacon library.
//!
//! Development should normally launch through Foundation:
//! `cargo run -p foundation -- --game last-beacon`.

fn main() -> bevy::prelude::AppExit {
    last_beacon::run()
}
