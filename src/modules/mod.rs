use core::{CoreAPI, SourceEvent};
use serde_json::Value;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResumeEventHandling {
    Stop,
    Resume,
}

pub type ModuleBuilder = fn(String, Option<Value>) -> Box<Module>;

pub trait Module {
    fn handle_event(&mut self, core: &mut CoreAPI, event: SourceEvent) -> ResumeEventHandling;
}
