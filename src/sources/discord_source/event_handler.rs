use crate::core::*;
use crate::sources::*;
use serenity::http::client::Http;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::model::user::{CurrentUser, User};
use serenity::prelude::{Context, EventHandler};
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex, RwLock};

#[derive(Clone, Default)]
struct DiscordData {
    user: Option<CurrentUser>,
    channels: HashMap<String, ChannelId>,
    http: Option<Arc<Http>>,
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
pub struct DiscordEventHandler {
    inner: Arc<DiscordEventHandlerImpl>,
}

impl DiscordEventHandler {
    pub fn new(id: SourceId, sender: Sender<SourceEvent>) -> Self {
        Self {
            inner: Arc::new(DiscordEventHandlerImpl::new(id, sender)),
        }
    }

    pub fn nick(&self) -> String {
        self.inner
            .data
            .read()
            .unwrap()
            .user
            .as_ref()
            .map(|u| u.name.clone())
            .unwrap_or("".to_string())
    }

    fn send_to_channel(&mut self, dst: String, msg: MessageContent) -> SourceResult<()> {
        let data = self.inner.data.read().unwrap();
        let channel = match data.channels.get(&dst) {
            None => {
                return Err(SourceError::InvalidChannel(
                    self.inner.id.clone(),
                    Channel::Channel(dst),
                ));
            }
            Some(ch) => ch,
        };
        let msg = match msg {
            MessageContent::Text(t) => t,
            MessageContent::Me(t) => t,
            _ => return Err(SourceError::InvalidMessage(self.inner.id.clone(), msg)),
        };
        {
            let data = self.inner.data.read().unwrap();
            if let Some(ref http) = data.http {
                channel.say(&http, msg)?;
            }
        }
        Ok(())
    }

    fn send_to_user(&mut self, _dst: String, _msg: MessageContent) -> SourceResult<()> {
        Ok(())
    }

    pub fn send(&mut self, dst: Channel, msg: MessageContent) -> SourceResult<()> {
        match dst {
            Channel::Channel(ch) => self.send_to_channel(ch, msg),
            Channel::User(usr) => self.send_to_user(usr, msg),
            _ => return Err(SourceError::InvalidChannel(self.inner.id.clone(), dst)),
        }
    }

    fn replace_mentions(msg: String, mentions: &[User]) -> String {
        let mut result = msg;
        for mention in mentions {
            let str_mention = format!("<@{}>", mention.id.0);
            let str_mention_2 = format!("<@!{}>", mention.id.0);
            let nick = format!("@{}", mention.name);
            result = result.replace(&str_mention, &nick);
            result = result.replace(&str_mention_2, &nick);
        }
        result
    }
}

impl EventHandler for DiscordEventHandler {
    fn ready(&self, ctx: Context, ready: Ready) {
        let mut data = self.inner.data.write().unwrap();
        data.http = Some(ctx.http.clone());
        data.user = Some(ready.user);
        for guild in &ready.guilds {
            let gid = guild.id();
            for (cid, channel) in gid.channels(&ctx).unwrap() {
                let _ = data.channels.insert(channel.name.clone(), cid);
            }
        }
    }

    fn message(&self, ctx: Context, msg: Message) {
        let Message {
            author,
            channel_id,
            content,
            mentions,
            ..
        } = msg;
        if author.name == self.nick() {
            return;
        }
        let content_mentions_replaced = Self::replace_mentions(content, &mentions);
        let msg = crate::core::Message {
            author: author.name,
            channel: Channel::Channel(
                channel_id
                    .name(&ctx)
                    .clone()
                    .unwrap_or("[no channel]".to_string()),
            ),
            content: MessageContent::Text(content_mentions_replaced),
        };
        let _ = self.inner.sender.lock().unwrap().send(SourceEvent {
            source: self.inner.id.clone(),
            event: Event::ReceivedMessage(msg),
        });
    }
}
