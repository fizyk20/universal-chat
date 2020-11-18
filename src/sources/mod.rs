use crate::core::{Channel, EventSourceBuilder, MessageContent};
use std::collections::HashMap;

#[cfg(feature = "discord")]
pub mod discord_source;
mod error;
#[cfg(feature = "irc")]
pub mod irc_source;
#[cfg(feature = "slack")]
pub mod slack_source;
pub mod stdin;

#[cfg(feature = "discord")]
pub use self::discord_source::DiscordSource;
pub use self::error::SourceError;
#[cfg(feature = "irc")]
pub use self::irc_source::IrcSource;
#[cfg(feature = "slack")]
pub use self::slack_source::SlackSource;
pub use self::stdin::StdinSource;

lazy_static! {
    pub static ref BUILDERS: HashMap<String, EventSourceBuilder> = {
        let mut m = HashMap::<String, EventSourceBuilder>::new();
        #[cfg(feature = "discord")]
        m.insert("Discord".to_owned(), DiscordSource::new);
        #[cfg(feature = "irc")]
        m.insert("Irc".to_owned(), IrcSource::new);
        #[cfg(feature = "slack")]
        m.insert("Slack".to_owned(), SlackSource::new);
        m.insert("stdin".to_owned(), StdinSource::new);
        m
    };
}

/// A more concise result type
pub type SourceResult<T> = Result<T, SourceError>;

/// Trait representing a source of events
pub trait EventSource {
    /// Gets the bot's nickname on this source
    fn get_nick(&self) -> String;
    /// Connects to the source
    fn connect(&mut self) -> SourceResult<()>;
    /// Joins a channel in the source
    fn join(&mut self, channel: &str) -> SourceResult<()>;
    /// Sends a message to the source
    fn send(&mut self, dst: Channel, msg: MessageContent) -> SourceResult<()>;
    /// Reconnects to the source
    fn reconnect(&mut self) -> SourceResult<()>;
}

#[cfg(test)]
mod test {
    use crate::sources::*;

    #[test]
    fn test_object_safety() {
        // if this compiles, EventSource can be used as a trait object
        let _ = |a: &mut dyn EventSource| {
            a.reconnect().unwrap();
        };
    }
}
