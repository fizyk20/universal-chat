#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct SourceId(pub String);

/// Different kinds of communication channels
#[derive(Clone, Debug, PartialEq)]
pub enum Channel {
    None,
    Channel(String),
    User(String),
    Group(Vec<String>),
}

impl Channel {
    pub fn as_str(&self) -> String {
        match *self {
            Channel::None => format!("[none]"),
            Channel::Channel(ref s) => format!("#{}", s),
            Channel::User(ref u) => format!("{}(priv)", u),
            Channel::Group(ref v) => {
                let mut v2 = v.clone();
                v2.sort();
                format!("{:?}", v2)
            }
        }
    }
}

/// Content of a message
#[derive(Clone, Debug)]
pub enum MessageContent {
    /// Simple text message
    Text(String),
    /// An image - TODO
    Image,
    /// A /me type message
    Me(String),
}

impl MessageContent {
    pub fn display_with_nick(&self, nick: &str) -> String {
        match *self {
            MessageContent::Text(ref txt) => format!("<{}> {}", nick, txt),
            MessageContent::Me(ref txt) => format!("* {} {}", nick, txt),
            MessageContent::Image => format!("<{}> [Image]", nick),
        }
    }
}

/// Message content bundled with the author and the source channel
#[derive(Clone, Debug)]
pub struct Message {
    pub author: String,
    pub channel: Channel,
    pub content: MessageContent,
}

/// Type representing events that can be sent by the sources
#[derive(Clone, Debug)]
pub enum Event {
    Connected,
    Disconnected(String),
    DirectInput(String),
    ReceivedMessage(Message),
    UserOnline(String),
    UserOffline(String, Option<String>),
    UserTyping(String),
    NickChange(String, String),
    Timer(String),
    Other(String),
}

/// Enum representing types of events
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventType {
    Connection,
    TextMessage,
    MeMessage,
    ImageMessage,
    UserStatus,
    Timer,
    Other,
}

impl Event {
    pub fn get_type(&self) -> EventType {
        match *self {
            Event::Connected | Event::Disconnected(_) => EventType::Connection,
            Event::DirectInput(_) => EventType::TextMessage,
            Event::ReceivedMessage(ref msg) => match msg.content {
                MessageContent::Text(_) => EventType::TextMessage,
                MessageContent::Me(_) => EventType::MeMessage,
                MessageContent::Image => EventType::ImageMessage,
            },
            Event::UserOnline(_)
            | Event::UserOffline(_, _)
            | Event::UserTyping(_)
            | Event::NickChange(_, _) => EventType::UserStatus,
            Event::Timer(_) => EventType::Timer,
            Event::Other(_) => EventType::Other,
        }
    }
}

/// The event bundled with the source ID
#[derive(Clone, Debug)]
pub struct SourceEvent {
    pub source: SourceId,
    pub event: Event,
}
