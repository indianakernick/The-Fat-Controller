mod key;
mod mouse;
mod command;
mod command_code_enum;
mod key_enum;
mod mouse_button_enum;

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

pub use key_enum::Key;
pub use mouse_button_enum::MouseButton;
pub use command_code_enum::CommandCode;
pub use command::Command;
