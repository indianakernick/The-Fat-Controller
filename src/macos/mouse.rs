use super::EventContext;
use foreign_types::ForeignType;
use core_graphics::display::CGPoint;
use core_graphics::base::{boolean_t, CGFloat};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
use core_graphics::event::{CGEventType, CGEvent, CGMouseButton, CGEventTapLocation, ScrollEventUnit, EventField};

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum MouseButton {
    Left,
    Right,
    Center,
}

#[link(name = "CoreGraphics", kind = "framework")]
extern {
    /// Returns a Boolean value indicating the current button state of a Quartz event source
    fn CGEventSourceButtonState(state_id: CGEventSourceStateID, button: CGMouseButton) -> boolean_t;
}

impl EventContext {
    fn button_state(button: CGMouseButton) -> bool {
        unsafe {
            CGEventSourceButtonState(CGEventSourceStateID::CombinedSessionState, button) != 0
        }
    }

    fn mouse_location() -> CGPoint {
        let source;
        unsafe {
            source = CGEventSource::from_ptr(std::ptr::null_mut());
        }
        CGEvent::new(source).unwrap().location()
    }

    fn mouse_move_to_point(&mut self, point: CGPoint) {
        let (button, event_type) = if Self::button_state(CGMouseButton::Left) {
            (CGMouseButton::Left, CGEventType::LeftMouseDragged)
        } else if Self::button_state(CGMouseButton::Right) {
            (CGMouseButton::Right, CGEventType::RightMouseDragged)
        } else if Self::button_state(CGMouseButton::Center) {
            (CGMouseButton::Center, CGEventType::OtherMouseDragged)
        } else {
            (CGMouseButton::Left, CGEventType::MouseMoved)
        };

        CGEvent::new_mouse_event(
            self.event_source.clone(),
            event_type,
            point,
            button
        ).unwrap().post(CGEventTapLocation::HID);
    }

    pub fn mouse_move_to(&mut self, x: i32, y: i32) {
        self.mouse_move_to_point(CGPoint::new(x as CGFloat, y as CGFloat));
    }

    pub fn mouse_move_relative(&mut self, x: i32, y: i32) {
        let mut pos = Self::mouse_location();
        pos.x += x as CGFloat;
        pos.y += y as CGFloat;
        self.mouse_move_to_point(pos);
    }

    fn mouse_event(&mut self, button: CGMouseButton, event_type: CGEventType, click_count: u32) {
        let event = CGEvent::new_mouse_event(
            self.event_source.clone(),
            event_type,
            Self::mouse_location(),
            button
        ).unwrap();
        event.set_integer_value_field(EventField::MOUSE_EVENT_CLICK_STATE, click_count as i64);
        event.post(CGEventTapLocation::HID);
    }

    fn convert_button(button: MouseButton) -> CGMouseButton {
        match button {
            MouseButton::Left => CGMouseButton::Left,
            MouseButton::Right => CGMouseButton::Right,
            MouseButton::Center => CGMouseButton::Center,
        }
    }

    fn mouse_nth_down(&mut self, button: MouseButton, click_count: u32) {
        let button = Self::convert_button(button);
        let event_type = match button {
            CGMouseButton::Left => CGEventType::LeftMouseDown,
            CGMouseButton::Right => CGEventType::RightMouseDown,
            CGMouseButton::Center => CGEventType::OtherMouseDown,
        };
        self.mouse_event(button, event_type, click_count);
    }

    fn mouse_nth_up(&mut self, button: MouseButton, click_count: u32) {
        let button = Self::convert_button(button);
        let event_type = match button {
            CGMouseButton::Left => CGEventType::LeftMouseUp,
            CGMouseButton::Right => CGEventType::RightMouseUp,
            CGMouseButton::Center => CGEventType::OtherMouseUp,
        };
        self.mouse_event(button, event_type, click_count);
    }

    pub fn mouse_down(&mut self, button: MouseButton) {
        self.mouse_nth_down(button, 1);
    }

    pub fn mouse_up(&mut self, button: MouseButton) {
        self.mouse_nth_up(button, 1);
    }

    pub fn mouse_click(&mut self, button: MouseButton) {
        self.mouse_nth_click(button, 1);
    }

    pub fn mouse_nth_click(&mut self, button: MouseButton, click_count: u32) {
        self.mouse_nth_down(button, click_count);
        self.mouse_nth_up(button, click_count);
    }

    pub fn mouse_scroll_x(&mut self, length: i32) {
        CGEvent::new_scroll_event(
            self.event_source.clone(),
            ScrollEventUnit::PIXEL,
            2,
            0,
            length,
            0,
        ).unwrap().post(CGEventTapLocation::HID);
    }

    pub fn mouse_scroll_y(&mut self, length: i32) {
        CGEvent::new_scroll_event(
            self.event_source.clone(),
            ScrollEventUnit::PIXEL,
            1,
            length,
            0,
            0,
        ).unwrap().post(CGEventTapLocation::HID);
    }
}
