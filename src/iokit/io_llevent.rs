// IOKit/hidsystem/IOLLEvent.h

pub const NX_NULLEVENT: u32 = 0;
pub const NX_LMOUSEDOWN: u32 = 1;
pub const NX_LMOUSEUP: u32 = 2;
pub const NX_RMOUSEDOWN: u32 = 3;
pub const NX_RMOUSEUP: u32 = 4;
pub const NX_MOUSEMOVED: u32 = 5;
pub const NX_LMOUSEDRAGGED: u32 = 6;
pub const NX_RMOUSEDRAGGED: u32 = 7;
pub const NX_MOUSEENTERED: u32 = 8;
pub const NX_MOUSEEXITED: u32 = 9;
pub const NX_OMOUSEDOWN: u32 = 25;
pub const NX_OMOUSEUP: u32 = 26;
pub const NX_OMOUSEDRAGGED: u32 = 27;
pub const NX_KEYDOWN: u32 = 10;
pub const NX_KEYUP: u32 = 11;
pub const NX_FLAGSCHANGED: u32 = 12;
pub const NX_KITDEFINED: u32 = 13;
pub const NX_SYSDEFINED: u32 = 14;
pub const NX_APPDEFINED: u32 = 15;
pub const NX_SCROLLWHEELMOVED: u32 = 22;
pub const NX_ZOOM: u32 = 28;
pub const NX_TABLETPOINTER: u32 = 23;
pub const NX_TABLETPROXIMITY: u32 = 24;

pub const NX_SUBTYPE_AUX_MOUSE_BUTTONS: i16 = 7;
pub const NX_SUBTYPE_AUX_CONTROL_BUTTONS: i16 = 8;

pub const NX_ALPHASHIFTMASK: u32 = 0x00010000;
pub const NX_SHIFTMASK: u32 = 0x00020000;
pub const NX_CONTROLMASK: u32 = 0x00040000;
pub const NX_ALTERNATEMASK: u32 = 0x00080000;
pub const NX_COMMANDMASK: u32 = 0x00100000;
pub const NX_NUMERICPADMASK: u32 = 0x00200000;
pub const NX_HELPMASK: u32 = 0x00400000;
pub const NX_SECONDARYFNMASK: u32 = 0x00800000;
pub const NX_ALPHASHIFT_STATELESS_MASK: u32 = 0x01000000;

pub const NX_DEVICELCTLKEYMASK: u32 = 0x00000001;
pub const NX_DEVICELSHIFTKEYMASK: u32 = 0x00000002;
pub const NX_DEVICERSHIFTKEYMASK: u32 = 0x00000004;
pub const NX_DEVICELCMDKEYMASK: u32 = 0x00000008;
pub const NX_DEVICERCMDKEYMASK: u32 = 0x00000010;
pub const NX_DEVICELALTKEYMASK: u32 = 0x00000020;
pub const NX_DEVICERALTKEYMASK: u32 = 0x00000040;
pub const NX_DEVICE_ALPHASHIFT_STATELESS_MASK: u32 = 0x00000080;
pub const NX_DEVICERCTLKEYMASK: u32 = 0x00002000;

pub const NX_ASCIISET: u16 = 0;
pub const NX_SYMBOLSET: u16 = 1;
pub const NX_DINGBATSSET: u16 = 2;

// Rust doesn't support anonymous structs or anonymous unions so the
// declarations marked as "synthesised" don't exist in I/O Kit.

// Synthesised
#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_snake_case)]
pub struct NXMouseEventData {
    pub subx: u8,
    pub suby: u8,
    pub eventNum: i16,
    pub click: i32,
    pub pressure: u8,
    pub buttonNumber: u8,
    pub subType: u8,
    reserved2: u8,
    reserved3: i32,
    tablet: [u8; 32], // I don't need this so I didn't bother declaring it
}

// Synthesised
#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_snake_case)]
pub struct NXMouseMoveEventData {
    pub dx: i32,
    pub dy: i32,
    pub subx: u8,
    pub suby: u8,
    pub subType: u8,
    reserved1: u8,
    reserved2: i32,
    tablet: [u8; 32], // I don't need this so I didn't bother declaring it
}

// Synthesised
#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_snake_case)]
pub struct NXKeyEventData {
    pub origCharSet: u16,
    pub repeat: i16,
    pub charSet: u16,
    pub charCode: u16,
    pub keyCode: u16,
    pub origCharCode: u16,
    reserved1: i32,
    pub keyboardType: u32,
    reserved2: i32,
    reserved3: i32,
    reserved4: i32,
    reserved5: [i32; 4],
}

// Synthesised
#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_snake_case)]
pub struct NXScrollWheelEventData {
    pub deltaAxis1: i16,
    pub deltaAxis2: i16,
    pub deltaAxis3: i16,
    reserved1: i16,
    pub fixedDeltaAxis1: i32,
    pub fixedDeltaAxis2: i32,
    pub fixedDeltaAxis3: i32,
    pub pointDeltaAxis1: i32,
    pub pointDeltaAxis2: i32,
    pub pointDeltaAxis3: i32,
    reserved8: [i32; 4]
}

// Synthesised
#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_snake_case)]
pub union NXCompoundEventDataMisc {
    pub F: [f32; 11],
    pub L: [i32; 11],
    pub S: [i16; 22],
    pub C: [i8; 44],
}

// Synthesised
#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_snake_case)]
pub struct NXCompoundEventData {
    reserved: i16,
    pub subType: i16,
    pub misc: NXCompoundEventDataMisc,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_snake_case)]
pub union NXEventData {
    pub mouse: NXMouseEventData,
    pub mouseMove: NXMouseMoveEventData,
    pub key: NXKeyEventData,
    pub scrollWheel: NXScrollWheelEventData,
    pub compound: NXCompoundEventData,
}

impl Default for NXEventData {
    fn default() -> Self {
        unsafe {
            std::mem::zeroed()
        }
    }
}

#[allow(non_upper_case_globals)]
pub const kNXEventDataVersion: u32 = 2;
