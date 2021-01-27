use super::{EventContext, Key, Flags};
use core_graphics::event::{CGEvent, CGEventTapLocation, CGKeyCode, CGEventFlags};

impl EventContext {
    pub fn key_sequence(&mut self, sequence: &str) {
        let event = CGEvent::new_keyboard_event(self.event_source.clone(), 0, true).unwrap();
        event.set_string(sequence);
        event.post(CGEventTapLocation::HID);
    }

    fn key_event(&mut self, key: Key, down: bool) {
        CGEvent::new_keyboard_event(self.event_source.clone(), key as CGKeyCode, down).unwrap()
            .post(CGEventTapLocation::HID);
    }

    fn convert_flags(flags: Flags) -> CGEventFlags {
        CGEventFlags::from_bits_truncate((flags.bits() as u64) << 16)
    }

    fn key_event_flags(&mut self, key: Key, flags: Flags, down: bool) {
        let event = CGEvent::new_keyboard_event(self.event_source.clone(), key as CGKeyCode, down).unwrap();
        event.set_flags(Self::convert_flags(flags));
        event.post(CGEventTapLocation::HID);
    }

    pub fn key_down(&mut self, key: Key) {
        self.key_event(key, true);
    }

    pub fn key_up(&mut self, key: Key) {
        self.key_event(key, false);
    }

    pub fn key_click(&mut self, key: Key) {
        self.key_down(key);
        self.key_up(key);
    }

    pub fn key_click_flags(&mut self, key: Key, flags: Flags) {
        self.key_event_flags(key, flags, true);
        self.key_event_flags(key, Flags::NULL, false);
    }
}
