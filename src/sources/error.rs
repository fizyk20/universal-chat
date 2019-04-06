use crate::core::*;
#[cfg(feature = "irc")]
use irc::error::IrcError;
#[cfg(feature = "discord")]
use serenity::Error as SerenityError;
#[cfg(any(feature = "irc", feature = "discord"))]
use std::convert::From;
#[cfg(feature = "irc")]
use std::sync::mpsc::RecvError;

/// An error type for the application
#[derive(Debug)]
pub enum SourceError {
    Eof(SourceId),
    Disconnected(SourceId),
    ConnectionError(SourceId, String),
    InvalidChannel(SourceId, Channel),
    InvalidMessage(SourceId, MessageContent),
    #[cfg(feature = "irc")]
    IrcError(IrcError),
    #[cfg(feature = "irc")]
    RecvError(RecvError),
    #[cfg(feature = "discord")]
    DiscordError(SerenityError),
    Other(String),
}

#[cfg(feature = "irc")]
impl From<IrcError> for SourceError {
    fn from(e: IrcError) -> Self {
        SourceError::IrcError(e)
    }
}

#[cfg(feature = "irc")]
impl From<RecvError> for SourceError {
    fn from(e: RecvError) -> Self {
        SourceError::RecvError(e)
    }
}

#[cfg(feature = "discord")]
impl From<SerenityError> for SourceError {
    fn from(e: SerenityError) -> Self {
        SourceError::DiscordError(e)
    }
}
