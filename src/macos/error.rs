use super::{Context, ffi};
use crate::FallibleContext;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
#[repr(u32)] // #[repr(ffi::kern_return_t)]
pub enum PlatformError {
    Error = ffi::kIOReturnError,
    NoMemory = ffi::kIOReturnNoMemory,
    NoResources = ffi::kIOReturnNoResources,
    IPCError = ffi::kIOReturnIPCError,
    NoDevice = ffi::kIOReturnNoDevice,
    NotPrivileged = ffi::kIOReturnNotPrivileged,
    BadArgument = ffi::kIOReturnBadArgument,
    LockedRead = ffi::kIOReturnLockedRead,
    LockedWrite = ffi::kIOReturnLockedWrite,
    ExclusiveAccess = ffi::kIOReturnExclusiveAccess,
    BadMessageID = ffi::kIOReturnBadMessageID,
    Unsupported = ffi::kIOReturnUnsupported,
    VMError = ffi::kIOReturnVMError,
    InternalError = ffi::kIOReturnInternalError,
    IOError = ffi::kIOReturnIOError,
    CannotLock = ffi::kIOReturnCannotLock,
    NotOpen = ffi::kIOReturnNotOpen,
    NotReadable = ffi::kIOReturnNotReadable,
    NotWritable = ffi::kIOReturnNotWritable,
    NotAligned = ffi::kIOReturnNotAligned,
    BadMedia = ffi::kIOReturnBadMedia,
    StillOpen = ffi::kIOReturnStillOpen,
    RLDError = ffi::kIOReturnRLDError,
    DMAError = ffi::kIOReturnDMAError,
    Busy = ffi::kIOReturnBusy,
    Timeout = ffi::kIOReturnTimeout,
    Offline = ffi::kIOReturnOffline,
    NotReady = ffi::kIOReturnNotReady,
    NotAttached = ffi::kIOReturnNotAttached,
    NoChannels = ffi::kIOReturnNoChannels,
    NoSpace = ffi::kIOReturnNoSpace,
    PortExists = ffi::kIOReturnPortExists,
    CannotWire = ffi::kIOReturnCannotWire,
    NoInterrupt = ffi::kIOReturnNoInterrupt,
    NoFrames = ffi::kIOReturnNoFrames,
    MessageTooLarge = ffi::kIOReturnMessageTooLarge,
    NotPermitted = ffi::kIOReturnNotPermitted,
    NoPower = ffi::kIOReturnNoPower,
    NoMedia = ffi::kIOReturnNoMedia,
    UnformattedMedia = ffi::kIOReturnUnformattedMedia,
    UnsupportedMode = ffi::kIOReturnUnsupportedMode,
    Underrun = ffi::kIOReturnUnderrun,
    Overrun = ffi::kIOReturnOverrun,
    DeviceError = ffi::kIOReturnDeviceError,
    NoCompletion = ffi::kIOReturnNoCompletion,
    Aborted = ffi::kIOReturnAborted,
    NoBandwidth = ffi::kIOReturnNoBandwidth,
    NotResponding = ffi::kIOReturnNotResponding,
    IsoTooOld = ffi::kIOReturnIsoTooOld,
    IsoTooNew = ffi::kIOReturnIsoTooNew,
    NotFound = ffi::kIOReturnNotFound,
    Invalid = ffi::kIOReturnInvalid,
}

impl PlatformError {
    pub fn new(error_code: ffi::kern_return_t) -> Self {
        unsafe {
            std::mem::transmute(error_code)
        }
    }
}

impl Display for PlatformError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use PlatformError::*;
        match self {
            Error => write!(f, "General error"),
            NoMemory => write!(f, "Cannot allocate memory"),
            NoResources => write!(f, "Resource shortage"),
            IPCError => write!(f, "IPC error"),
            NoDevice => write!(f, "No such device"),
            NotPrivileged => write!(f, "Privilege violation"),
            BadArgument => write!(f, "Invalid argument"),
            LockedRead => write!(f, "Device read locked"),
            LockedWrite => write!(f, "Device write locked"),
            ExclusiveAccess => write!(f, "Exclusive access device is already open"),
            BadMessageID => write!(f, "Invalid message ID"),
            Unsupported => write!(f, "Unsupported function"),
            VMError => write!(f, "VM failure"),
            InternalError => write!(f, "Internal error"),
            IOError => write!(f, "General I/O error"),
            CannotLock => write!(f, "Cannot acquire lock"),
            NotOpen => write!(f, "Device not open"),
            NotReadable => write!(f, "Read not supported"),
            NotWritable => write!(f, "Write not supported"),
            NotAligned => write!(f, "Alignment error"),
            BadMedia => write!(f, "Media error"),
            StillOpen => write!(f, "Device still open"),
            RLDError => write!(f, "RLD failure"),
            DMAError => write!(f, "DMA failure"),
            Busy => write!(f, "Device busy"),
            Timeout => write!(f, "I/O timeout"),
            Offline => write!(f, "Device offline"),
            NotReady => write!(f, "Not ready"),
            NotAttached => write!(f, "Device not attached"),
            NoChannels => write!(f, "No DMA channels left"),
            NoSpace => write!(f, "No space for data"),
            PortExists => write!(f, "Port already exists"),
            CannotWire => write!(f, "Cannot wire down physical memory"),
            NoInterrupt => write!(f, "No interrupt attached"),
            NoFrames => write!(f, "No DMA frames enqueued"),
            MessageTooLarge => write!(f, "Oversized message on interrupt port"),
            NotPermitted => write!(f, "Not permitted"),
            NoPower => write!(f, "No power to device"),
            NoMedia => write!(f, "Media not present"),
            UnformattedMedia => write!(f, "Media not formatted"),
            UnsupportedMode => write!(f, "Unsupported mode"),
            Underrun => write!(f, "Data underrun"),
            Overrun => write!(f, "Data overrun"),
            DeviceError => write!(f, "Device is not working properly"),
            NoCompletion => write!(f, "A completion routine is required"),
            Aborted => write!(f, "Operation aborted"),
            NoBandwidth => write!(f, "Bus bandwidth would be exceeded"),
            NotResponding => write!(f, "Device not responding"),
            IsoTooOld => write!(f, "Isochronous I/O request for distant past"),
            IsoTooNew => write!(f, "Isochronous I/O request for distant future"),
            NotFound => write!(f, "Data was not found"),
            Invalid => write!(f, "(Invalid error code)"),
        }
    }
}

impl std::error::Error for PlatformError {}

impl FallibleContext for Context {
    type PlatformError = PlatformError;
}
