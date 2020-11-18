mod event_handler;

use crate::core::*;
use crate::sources::*;
use event_handler::DiscordEventHandler;
use serenity::client::Client;
use std::sync::mpsc::Sender;
use std::thread::{self, JoinHandle};
use toml::Value;

enum DiscordState {
    Disconnected,
    Running(JoinHandle<()>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct DiscordConfig {
    token: String,
}

pub struct DiscordSource {
    id: SourceId,
    sender: Sender<SourceEvent>,
    state: DiscordState,
    config: DiscordConfig,
    handler: DiscordEventHandler,
}

impl DiscordSource {
    pub fn new(
        source_id: SourceId,
        sender: Sender<SourceEvent>,
        config: Option<Value>,
    ) -> Box<dyn EventSource> {
        let config = config.expect(&format!(
            "No config given for Discord source {:?}!",
            source_id
        ));
        let config: DiscordConfig = config.try_into().ok().expect(&format!(
            "Invalid configuration supplied to Discord source {:?}",
            source_id
        ));

        let handler = DiscordEventHandler::new(source_id.clone(), sender.clone());
        Box::new(DiscordSource {
            id: source_id,
            sender,
            config,
            handler,
            state: DiscordState::Disconnected,
        })
    }
}

impl EventSource for DiscordSource {
    fn get_nick(&self) -> String {
        self.handler.nick()
    }

    fn connect(&mut self) -> SourceResult<()> {
        let mut client = Client::new(&self.config.token, self.handler.clone())?;
        let sender = self.sender.clone();
        let id = self.id.clone();

        let handle = thread::spawn(move || {
            if let Err(e) = client.start() {
                let _ = sender.send(SourceEvent {
                    source: id.clone(),
                    event: Event::Disconnected(format!("{:?}", e)),
                });
            }
        });

        self.state = DiscordState::Running(handle);

        Ok(())
    }

    fn join(&mut self, _channel: &str) -> SourceResult<()> {
        Ok(())
    }

    fn send(&mut self, dst: Channel, msg: MessageContent) -> SourceResult<()> {
        self.handler.send(dst, msg)
    }

    fn reconnect(&mut self) -> SourceResult<()> {
        self.connect()
    }
}
