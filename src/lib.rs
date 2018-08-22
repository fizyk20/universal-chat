extern crate chrono;
#[cfg(feature = "discord")]
extern crate discord;
#[cfg(feature = "irc")]
extern crate irc;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate quick_error;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[cfg(feature = "slack")]
extern crate slack;
extern crate timer;

mod config;
mod core;
mod logger;
mod modules;
mod sources;

pub use config::Config;
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
