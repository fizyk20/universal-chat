use crate::core::*;
use crate::sources::*;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::model::user::CurrentUser;
use serenity::prelude::{Context, EventHandler};
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex, RwLock};

#[derive(Clone, Debug, Default)]
struct DiscordData {
    user: Option<CurrentUser>,
    channels: HashMap<String, ChannelId>,
}

struct DiscordEventHandlerImpl {
    id: SourceId,
    sender: Mutex<Sender<SourceEvent>>,
    data: RwLock<DiscordData>,
}

impl DiscordEventHandlerImpl {
    pub fn new(id: SourceId, sender: Sender<SourceEvent>) -> Self {
        Self {
            id,
            sender: Mutex::new(sender),
            data: Default::default(),
        }
    }
}

#[derive(Clone)]
pub struct DiscordEventHandler(Arc<DiscordEventHandlerImpl>);

impl DiscordEventHandler {
    pub fn new(id: SourceId, sender: Sender<SourceEvent>) -> Self {
        Self(Arc::new(DiscordEventHandlerImpl::new(id, sender)))
    }

    pub fn nick(&self) -> String {
        self.0
            .data
            .read()
            .unwrap()
            .user
            .as_ref()
            .map(|u| u.name.clone())
            .unwrap_or("".to_string())
    }

    fn send_to_channel(&mut self, dst: String, msg: MessageContent) -> SourceResult<()> {
        let data = self.0.data.read().unwrap();
        let channel = match data.channels.get(&dst) {
            None => {
                return Err(SourceError::InvalidChannel(
                    self.0.id.clone(),
                    Channel::Channel(dst),
                ));
            }
            Some(ch) => ch,
        };
        let msg = match msg {
            MessageContent::Text(t) => t,
            MessageContent::Me(t) => t,
            _ => return Err(SourceError::InvalidMessage(self.0.id.clone(), msg)),
        };
        channel.say(msg)?;
        Ok(())
    }

    fn send_to_user(&mut self, _dst: String, _msg: MessageContent) -> SourceResult<()> {
        Ok(())
    }

    pub fn send(&mut self, dst: Channel, msg: MessageContent) -> SourceResult<()> {
        match dst {
            Channel::Channel(ch) => self.send_to_channel(ch, msg),
            Channel::User(usr) => self.send_to_user(usr, msg),
            _ => return Err(SourceError::InvalidChannel(self.0.id.clone(), dst)),
        }
    }
}

impl EventHandler for DiscordEventHandler {
    fn ready(&self, _ctx: Context, ready: Ready) {
        let mut data = self.0.data.write().unwrap();
        data.user = Some(ready.user);
        for guild in &ready.guilds {
            let gid = guild.id();
            for (cid, channel) in gid.channels().unwrap() {
                let _ = data.channels.insert(channel.name.clone(), cid);
            }
        }
    }

    fn message(&self, _ctx: Context, msg: Message) {
        let Message {
            author,
            channel_id,
            content,
            ..
        } = msg;
        let msg = crate::core::Message {
            author: author.name,
            channel: Channel::Channel(
                channel_id
                    .name()
                    .clone()
                    .unwrap_or("[no channel]".to_string()),
            ),
            content: MessageContent::Text(content),
        };
        let _ = self.0.sender.lock().unwrap().send(SourceEvent {
            source: self.0.id.clone(),
            event: Event::ReceivedMessage(msg),
        });
    }
}