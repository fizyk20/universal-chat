use core::{CoreAPI, SourceEvent};
use serde_json::Value;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResumeEventHandling {
    Stop,
    Resume,
}

pub trait Module {
    fn create(id: String, config: Option<Value>) -> Self
    where
        Self: Sized;
    fn handle_event(&mut self, core: &mut CoreAPI, event: SourceEvent) -> ResumeEventHandling;
}
