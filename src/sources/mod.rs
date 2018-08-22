use core::{Channel, EventSourceBuilder, MessageContent, SourceId};
#[cfg(feature = "irc")]
use irc::error::IrcError;
use std::collections::HashMap;
#[cfg(feature = "irc")]
use std::sync::mpsc::RecvError;

#[cfg(feature = "discord")]
pub mod discord;
#[cfg(feature = "irc")]
pub mod irc;
#[cfg(feature = "slack")]
pub mod slack;
pub mod stdin;

#[cfg(feature = "discord")]
pub use self::discord::DiscordSource;
#[cfg(feature = "irc")]
pub use self::irc::IrcSource;
#[cfg(feature = "slack")]
pub use self::slack::SlackSource;
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

/// An error type for the application
#[cfg(not(feature = "irc"))]
quick_error! {
    #[derive(Debug)]
    pub enum SourceError {
        Eof(id: SourceId) {}
        Disconnected(id: SourceId) {}
        ConnectionError(id: SourceId, txt: String) {}
        InvalidChannel(id: SourceId, ch: Channel) {}
        InvalidMessage(id: SourceId, msg: MessageContent) {}
        Other(txt: String) {}
    }
}

/// An error type for the application
#[cfg(feature = "irc")]
quick_error! {
    #[derive(Debug)]
    pub enum SourceError {
        Eof(id: SourceId) {}
        Disconnected(id: SourceId) {}
        ConnectionError(id: SourceId, txt: String) {}
        InvalidChannel(id: SourceId, ch: Channel) {}
        InvalidMessage(id: SourceId, msg: MessageContent) {}
        IrcError(err: IrcError) {
            from()
        }
        RecvError(err: RecvError) {
            from()
        }
        Other(txt: String) {}
    }
}

/// A more concise result type
pub type SourceResult<T> = Result<T, SourceError>;

/// Trait representing a source of events
pub trait EventSource {
    /// Gets the bot's nickname on this source
    fn get_nick(&self) -> &str;
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
    use sources::*;

    #[test]
    fn test_object_safety() {
        // if this compiles, EventSource can be used as a trait object
        let _ = |a: &mut EventSource| {
            a.reconnect().unwrap();
        };
    }
}
