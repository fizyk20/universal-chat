use crate::core::{CoreAPI, SourceEvent};
use toml::Value;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResumeEventHandling {
    Stop,
    Resume,
}

pub type ModuleBuilder = fn(String, Option<Value>) -> Box<dyn Module>;

pub trait Module {
    fn handle_event(&mut self, core: &mut CoreAPI, event: SourceEvent) -> ResumeEventHandling;
}
