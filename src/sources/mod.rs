use core::{Channel, MessageContent, SourceId};
use irc::error::Error as IrcError;
use std::sync::mpsc::Sender;

pub mod discord;
pub mod irc;
pub mod stdin;
pub mod slack;

pub use self::discord::DiscordSource;
pub use self::irc::IrcSource;
pub use self::slack::SlackSource;
pub use self::stdin::StdinSource;

/// An error type for the application
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
