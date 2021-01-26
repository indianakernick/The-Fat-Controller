use super::{EventContext, Key};
use core_graphics::event::{CGEvent, CGEventTapLocation, CGKeyCode};

impl EventContext {
    pub fn key_sequence(&mut self, sequence: &str) {
        let event = CGEvent::new_keyboard_event(self.event_source.clone(), 0, true).unwrap();
        event.set_string(sequence);
        event.post(CGEventTapLocation::HID);
    }

    pub fn key_down(&mut self, key: Key) {
        CGEvent::new_keyboard_event(self.event_source.clone(), key as CGKeyCode, true)
            .unwrap().post(CGEventTapLocation::HID);
    }

    pub fn key_up(&mut self, key: Key) {
        CGEvent::new_keyboard_event(self.event_source.clone(), key as CGKeyCode, false)
            .unwrap().post(CGEventTapLocation::HID);
    }

    pub fn key_click(&mut self, key: Key) {
        self.key_down(key);
        self.key_up(key);
    }
}
