use crate::iokit as io;

impl super::EventContext {
    pub fn mouse_move_rel(&mut self, dx: i32, dy: i32) -> bool {
        let mut event = io::NXEventData::default();
        event.mouseMove.dx = dx;
        event.mouseMove.dy = dy;
        let location = io::IOGPoint::default();
        unsafe {
            io::IOHIDPostEvent(
                self.connect,
                io::NX_MOUSEMOVED,
                location,
                &event,
                io::kNXEventDataVersion,
                0,
                0
            ) == io::KERN_SUCCESS
        }
    }

    pub fn mouse_move_abs(&mut self, x: i32, y: i32) -> bool {
        use std::os::raw::c_int;
        unsafe {
            io::IOHIDSetMouseLocation(
                self.connect,
                x as c_int,
                y as c_int
            ) == io::KERN_SUCCESS
        }
    }

    pub fn mouse_scroll(&mut self, dx: i32, dy: i32) -> bool {
        let mut event = io::NXEventData::default();
        event.scrollWheel.fixedDeltaAxis1 = dy << 13;
        event.scrollWheel.fixedDeltaAxis2 = dx << 13;
        let location = io::IOGPoint::default();
        unsafe {
            io::IOHIDPostEvent(
                self.connect,
                io::NX_SCROLLWHEELMOVED,
                location,
                &event,
                io::kNXEventDataVersion,
                0,
                0
            ) == io::KERN_SUCCESS
        }
    }
}
