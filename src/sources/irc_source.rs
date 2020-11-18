use crate::core::*;
use crate::sources::*;
use irc::client::prelude::*;
use std::sync::mpsc::{channel, Sender};
use std::thread::{self, JoinHandle};
use toml::Value;

/// A helper enum for IrcSource
enum SourceState {
    Disconnected,
    Connected(IrcClient, JoinHandle<SourceResult<()>>),
}

/// An IRC event source
pub struct IrcSource {
    /// bot's nick on the server
    nick: String,
    /// the source ID
    id: SourceId,
    /// IRC client configuration data
    config: Config,
    /// Event sender object
    sender: Sender<SourceEvent>,
    /// Current state of the source
    state: SourceState,
}

impl IrcSource {
    /// Creates an IrcSource with the given configuration
    pub fn new(
        source_id: SourceId,
        sender: Sender<SourceEvent>,
        config: Option<Value>,
    ) -> Box<dyn EventSource> {
        let config = config.expect(&format!("No config given for IRC source {:?}!", source_id));
        let config: Config = config.try_into().ok().expect(&format!(
            "Invalid configuration supplied to IRC source {:?}",
            source_id
        ));

        Box::new(IrcSource {
            id: source_id.clone(),
            nick: config
                .nickname()
                .expect("Couldn't get the IRC nickname!")
                .to_owned(),
            config,
            sender,
            state: SourceState::Disconnected,
        })
    }
}

fn message_to_events(msg: ::irc::client::prelude::Message) -> Vec<Event> {
    use irc::client::prelude::Command::*;
    use irc::client::prelude::Response::*;
    let sender = msg
        .prefix
        .clone()
        .unwrap_or_else(|| "".to_string())
        .chars()
        .take_while(|c| *c != '!')
        .collect();
    match msg.command {
        PING(_, _) => vec![],
        PONG(_, _) => vec![],
        PRIVMSG(from, txt) => vec![Event::ReceivedMessage(crate::core::Message {
            author: sender,
            channel: if from.starts_with("#") {
                Channel::Channel(from)
            } else {
                Channel::User(from)
            },
            content: MessageContent::Text(txt),
        })],
        NICK(new_nick) => vec![Event::NickChange(sender, new_nick)],
        JOIN(_, _, _) => vec![Event::UserOnline(sender)],
        PART(_, comment) | QUIT(comment) => vec![Event::UserOffline(sender, comment)],
        Response(code, _, ref msg) if code == RPL_NAMREPLY => {
            if let &Some(ref msg) = msg {
                msg.split_whitespace()
                    .map(|x| Event::UserOnline(x.to_owned()))
                    .collect()
            } else {
                vec![]
            }
        }
        _ => vec![Event::Other(format!("{:?}", msg))],
    }
}

impl EventSource for IrcSource {
    fn get_nick(&self) -> String {
        self.nick.clone()
    }

    fn connect(&mut self) -> SourceResult<()> {
        // create clones of some values for the event thread
        let thread_sender = self.sender.clone();
        let source_id = self.id.clone();

        let (tx, rx) = channel();
        let config = self.config.clone();

        // create the event handling thread
        let handle = thread::spawn(move || -> SourceResult<()> {
            let mut reactor = IrcReactor::new()?;
            let client = reactor.prepare_client_and_connect(&config)?;

            client.identify()?;
            reactor.register_client_with_handler(client.clone(), move |_, message| {
                let events = message_to_events(message);
                for event in events {
                    let _ = thread_sender.send(SourceEvent {
                        source: source_id.clone(),
                        event,
                    });
                }
                Ok(())
            });

            // send a copy of the client to the external thread
            let _ = tx.send(client);

            reactor.run()?;
            Ok(())
        });

        // receive the client from the reactor thread
        let client = rx.recv()?;
        // save the server object and thread handle
        self.state = SourceState::Connected(client, handle);
        Ok(())
    }

    fn join(&mut self, _channel: &str) -> SourceResult<()> {
        Ok(())
    }

    /// Sends a message to a user or an IRC channel
    fn send(&mut self, dst: Channel, msg: MessageContent) -> SourceResult<()> {
        let state = match self.state {
            SourceState::Connected(ref client, _) => client,
            _ => return Err(SourceError::Disconnected(self.id.clone())),
        };
        let target = match dst {
            Channel::Channel(c) => c,
            Channel::User(u) => u,
            _ => return Err(SourceError::InvalidChannel(self.id.clone(), dst)),
        };
        let msg = match msg {
            MessageContent::Text(t) => t,
            MessageContent::Me(t) => t,
            _ => return Err(SourceError::InvalidMessage(self.id.clone(), msg)),
        };
        let message = ::irc::client::prelude::Command::PRIVMSG(target, msg);
        state.send(message)?;
        Ok(())
    }

    fn reconnect(&mut self) -> SourceResult<()> {
        Ok(())
    }
}
