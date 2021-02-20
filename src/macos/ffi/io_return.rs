// IOKit/IOReturn.h
// /Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/IOKit.framework/Versions/A/Headers/IOReturn.h

#[allow(non_camel_case_types)]
pub type kern_return_t = u32;

// sys_iokit | sub_iokit_common
#[allow(non_upper_case_globals)]
const iokit_common_err: kern_return_t = 0xE0000000;

#[allow(non_upper_case_globals)]
pub const kIOReturnSuccess: kern_return_t = 0;
#[allow(non_upper_case_globals)]
pub const kIOReturnError: kern_return_t = iokit_common_err | 0x2bc;
#[allow(non_upper_case_globals)]
pub const kIOReturnNoMemory: kern_return_t = iokit_common_err | 0x2bd;
#[allow(non_upper_case_globals)]
pub const kIOReturnNoResources: kern_return_t = iokit_common_err | 0x2be;
#[allow(non_upper_case_globals)]
pub const kIOReturnIPCError: kern_return_t = iokit_common_err | 0x2bf;
#[allow(non_upper_case_globals)]
pub const kIOReturnNoDevice: kern_return_t = iokit_common_err | 0x2c0;
#[allow(non_upper_case_globals)]
pub const kIOReturnNotPrivileged: kern_return_t = iokit_common_err | 0x2c1;
#[allow(non_upper_case_globals)]
pub const kIOReturnBadArgument: kern_return_t = iokit_common_err | 0x2c2;
#[allow(non_upper_case_globals)]
pub const kIOReturnLockedRead: kern_return_t = iokit_common_err | 0x2c3;
#[allow(non_upper_case_globals)]
pub const kIOReturnLockedWrite: kern_return_t = iokit_common_err | 0x2c4;
#[allow(non_upper_case_globals)]
pub const kIOReturnExclusiveAccess: kern_return_t = iokit_common_err | 0x2c5;
#[allow(non_upper_case_globals)]
pub const kIOReturnBadMessageID: kern_return_t = iokit_common_err | 0x2c6;
#[allow(non_upper_case_globals)]
pub const kIOReturnUnsupported: kern_return_t = iokit_common_err | 0x2c7;
#[allow(non_upper_case_globals)]
pub const kIOReturnVMError: kern_return_t = iokit_common_err | 0x2c8;
#[allow(non_upper_case_globals)]
pub const kIOReturnInternalError: kern_return_t = iokit_common_err | 0x2c9;
#[allow(non_upper_case_globals)]
pub const kIOReturnIOError: kern_return_t = iokit_common_err | 0x2ca;
#[allow(non_upper_case_globals)]
pub const kIOReturnCannotLock: kern_return_t = iokit_common_err | 0x2cc;
#[allow(non_upper_case_globals)]
pub const kIOReturnNotOpen: kern_return_t = iokit_common_err | 0x2cd;
#[allow(non_upper_case_globals)]
pub const kIOReturnNotReadable: kern_return_t = iokit_common_err | 0x2ce;
#[allow(non_upper_case_globals)]
pub const kIOReturnNotWritable: kern_return_t = iokit_common_err | 0x2cf;
#[allow(non_upper_case_globals)]
pub const kIOReturnNotAligned: kern_return_t = iokit_common_err | 0x2d0;
#[allow(non_upper_case_globals)]
pub const kIOReturnBadMedia: kern_return_t = iokit_common_err | 0x2d1;
#[allow(non_upper_case_globals)]
pub const kIOReturnStillOpen: kern_return_t = iokit_common_err | 0x2d2;
#[allow(non_upper_case_globals)]
pub const kIOReturnRLDError: kern_return_t = iokit_common_err | 0x2d3;
#[allow(non_upper_case_globals)]
pub const kIOReturnDMAError: kern_return_t = iokit_common_err | 0x2d4;
#[allow(non_upper_case_globals)]
pub const kIOReturnBusy: kern_return_t = iokit_common_err | 0x2d5;
#[allow(non_upper_case_globals)]
pub const kIOReturnTimeout: kern_return_t = iokit_common_err | 0x2d6;
#[allow(non_upper_case_globals)]
pub const kIOReturnOffline: kern_return_t = iokit_common_err | 0x2d7;
#[allow(non_upper_case_globals)]
pub const kIOReturnNotReady: kern_return_t = iokit_common_err | 0x2d8;
#[allow(non_upper_case_globals)]
pub const kIOReturnNotAttached: kern_return_t = iokit_common_err | 0x2d9;
#[allow(non_upper_case_globals)]
pub const kIOReturnNoChannels: kern_return_t = iokit_common_err | 0x2da;
#[allow(non_upper_case_globals)]
pub const kIOReturnNoSpace: kern_return_t = iokit_common_err | 0x2db;
#[allow(non_upper_case_globals)]
pub const kIOReturnPortExists: kern_return_t = iokit_common_err | 0x2dd;
#[allow(non_upper_case_globals)]
pub const kIOReturnCannotWire: kern_return_t = iokit_common_err | 0x2de;
#[allow(non_upper_case_globals)]
pub const kIOReturnNoInterrupt: kern_return_t = iokit_common_err | 0x2df;
#[allow(non_upper_case_globals)]
pub const kIOReturnNoFrames: kern_return_t = iokit_common_err | 0x2e0;
#[allow(non_upper_case_globals)]
pub const kIOReturnMessageTooLarge: kern_return_t = iokit_common_err | 0x2e1;
#[allow(non_upper_case_globals)]
pub const kIOReturnNotPermitted: kern_return_t = iokit_common_err | 0x2e2;
#[allow(non_upper_case_globals)]
pub const kIOReturnNoPower: kern_return_t = iokit_common_err | 0x2e3;
#[allow(non_upper_case_globals)]
pub const kIOReturnNoMedia: kern_return_t = iokit_common_err | 0x2e4;
#[allow(non_upper_case_globals)]
pub const kIOReturnUnformattedMedia: kern_return_t = iokit_common_err | 0x2e5;
#[allow(non_upper_case_globals)]
pub const kIOReturnUnsupportedMode: kern_return_t = iokit_common_err | 0x2e6;
#[allow(non_upper_case_globals)]
pub const kIOReturnUnderrun: kern_return_t = iokit_common_err | 0x2e7;
#[allow(non_upper_case_globals)]
pub const kIOReturnOverrun: kern_return_t = iokit_common_err | 0x2e8;
#[allow(non_upper_case_globals)]
pub const kIOReturnDeviceError: kern_return_t = iokit_common_err | 0x2e9;
#[allow(non_upper_case_globals)]
pub const kIOReturnNoCompletion: kern_return_t = iokit_common_err | 0x2ea;
#[allow(non_upper_case_globals)]
pub const kIOReturnAborted: kern_return_t = iokit_common_err | 0x2eb;
#[allow(non_upper_case_globals)]
pub const kIOReturnNoBandwidth: kern_return_t = iokit_common_err | 0x2ec;
#[allow(non_upper_case_globals)]
pub const kIOReturnNotResponding: kern_return_t = iokit_common_err | 0x2ed;
#[allow(non_upper_case_globals)]
pub const kIOReturnIsoTooOld: kern_return_t = iokit_common_err | 0x2ee;
#[allow(non_upper_case_globals)]
pub const kIOReturnIsoTooNew: kern_return_t = iokit_common_err | 0x2ef;
#[allow(non_upper_case_globals)]
pub const kIOReturnNotFound: kern_return_t = iokit_common_err | 0x2f0;
#[allow(non_upper_case_globals)]
pub const kIOReturnInvalid: kern_return_t = iokit_common_err | 0x1;
