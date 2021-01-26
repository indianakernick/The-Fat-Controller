mod key;
mod mouse;

use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

pub struct EventContext {
    event_source: CGEventSource,
}

impl Default for EventContext {
    fn default() -> Self {
        Self {
            event_source: CGEventSource::new(CGEventSourceStateID::CombinedSessionState).unwrap(),
        }
    }
}
