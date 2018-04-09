use core::{Event, SourceEvent, SourceId};
use serde_json::Value;
use sources::*;
use std::io;
use std::sync::mpsc::Sender;
use std::thread::{self, JoinHandle};

pub struct StdinSource(JoinHandle<()>);

impl StdinSource {
    /// Creates the Stdin source - a simple loop sending lines read from the standard input
    pub fn new(
        source_id: SourceId,
        sender: Sender<SourceEvent>,
        _: Option<Value>,
    ) -> Box<EventSource> {
        let handle = thread::spawn(move || {
            let stdin = io::stdin();
            loop {
                let mut buffer = String::new();
                stdin.read_line(&mut buffer).unwrap();
                sender
                    .send(SourceEvent {
                        source: source_id.clone(),
                        event: Event::DirectInput(buffer),
                    })
                    .unwrap();
            }
        });
        Box::new(StdinSource(handle))
    }
}

impl EventSource for StdinSource {
    fn get_nick(&self) -> &str {
        ""
    }

    fn connect(&mut self) -> SourceResult<()> {
        Ok(())
    }

    fn join(&mut self, _: &str) -> SourceResult<()> {
        Ok(())
    }

    fn send(&mut self, _: Channel, _: MessageContent) -> SourceResult<()> {
        Ok(())
    }

    fn reconnect(&mut self) -> SourceResult<()> {
        Ok(())
    }
}
