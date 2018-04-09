extern crate chrono;
extern crate discord;
extern crate irc;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate quick_error;
extern crate rand;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate slack;
extern crate slack_api;
extern crate timer;

mod sources;
mod config;
mod core;
mod logger;
mod modules;

pub use core::*;
pub use modules::*;
pub use sources::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
