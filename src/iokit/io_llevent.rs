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
    subx: u8,
    suby: u8,
    eventNum: i16,
    click: i32,
    pressure: u8,
    buttonNumber: u8,
    subType: u8,
    reserved2: u8,
    reserved3: i32,
    tablet: [u8; 32], // I don't need this so I didn't bother declaring it
}

// Synthesised
#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_snake_case)]
pub struct NXMouseMoveEventData {
    dx: i32,
    dy: i32,
    subx: u8,
    suby: u8,
    subType: u8,
    reserved1: u8,
    reserved2: i32,
    tablet: [u8; 32], // I don't need this so I didn't bother declaring it
}

// Synthesised
#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_snake_case)]
pub struct NXKeyEventData {
    origChatSet: u16,
    repeat: i16,
    charSet: u16,
    charCode: u16,
    keyCode: u16,
    origCharCode: u16,
    reserved1: i32,
    keyboardType: u32,
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
    deltaAxis1: i16,
    deltaAxis2: i16,
    deltaAxis3: i16,
    reserved1: i16,
    fixedDeltaAxis1: i32,
    fixedDeltaAxis2: i32,
    fixedDeltaAxis3: i32,
    pointDeltaAxis1: i32,
    pointDeltaAxis2: i32,
    pointDeltaAxis3: i32,
    reserved8: [i32; 4]
}

// Synthesised
#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_snake_case)]
pub union NXCompoundEventDataMisc {
    F: [f32; 11],
    L: [i32; 11],
    S: [i16; 22],
    C: [i8; 44],
}

// Synthesised
#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_snake_case)]
pub struct NXCompoundEventData {
    reserved: i16,
    subType: i16,
    misc: NXCompoundEventDataMisc,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_snake_case)]
pub union NXEventData {
    mouse: NXMouseEventData,
    mouseMove: NXMouseMoveEventData,
    key: NXKeyEventData,
    scrollWheel: NXScrollWheelEventData,
    compound: NXCompoundEventData,
}

#[allow(non_upper_case_globals)]
pub const kNXEventDataVersion: u32 = 2;
