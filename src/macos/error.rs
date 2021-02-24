use super::{Context, ffi};
use std::fmt::{self, Display, Formatter};
use crate::{FallibleContext, utils::NonZero};

type NonZeroKernReturn = <ffi::kern_return_t as NonZero>::Type;

#[derive(Debug)]
pub struct PlatformError(NonZeroKernReturn);

impl PlatformError {
    pub(super) fn new(error_code: ffi::kern_return_t) -> Self {
        unsafe {
            Self(NonZeroKernReturn::new_unchecked(error_code))
        }
    }
}

impl Display for PlatformError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use ffi::*;
        #[allow(non_upper_case_globals)]
        match self.0.get() {
            kIOReturnError => write!(f, "General error"),
            kIOReturnNoMemory => write!(f, "Cannot allocate memory"),
            kIOReturnNoResources => write!(f, "Resource shortage"),
            kIOReturnIPCError => write!(f, "IPC error"),
            kIOReturnNoDevice => write!(f, "No such device"),
            kIOReturnNotPrivileged => write!(f, "Privilege violation"),
            kIOReturnBadArgument => write!(f, "Invalid argument"),
            kIOReturnLockedRead => write!(f, "Device read locked"),
            kIOReturnLockedWrite => write!(f, "Device write locked"),
            kIOReturnExclusiveAccess => write!(f, "Exclusive access device is already open"),
            kIOReturnBadMessageID => write!(f, "Invalid message ID"),
            kIOReturnUnsupported => write!(f, "Unsupported function"),
            kIOReturnVMError => write!(f, "VM failure"),
            kIOReturnInternalError => write!(f, "Internal error"),
            kIOReturnIOError => write!(f, "General I/O error"),
            kIOReturnCannotLock => write!(f, "Cannot acquire lock"),
            kIOReturnNotOpen => write!(f, "Device not open"),
            kIOReturnNotReadable => write!(f, "Read not supported"),
            kIOReturnNotWritable => write!(f, "Write not supported"),
            kIOReturnNotAligned => write!(f, "Alignment error"),
            kIOReturnBadMedia => write!(f, "Media error"),
            kIOReturnStillOpen => write!(f, "Device still open"),
            kIOReturnRLDError => write!(f, "RLD failure"),
            kIOReturnDMAError => write!(f, "DMA failure"),
            kIOReturnBusy => write!(f, "Device busy"),
            kIOReturnTimeout => write!(f, "I/O timeout"),
            kIOReturnOffline => write!(f, "Device offline"),
            kIOReturnNotReady => write!(f, "Not ready"),
            kIOReturnNotAttached => write!(f, "Device not attached"),
            kIOReturnNoChannels => write!(f, "No DMA channels left"),
            kIOReturnNoSpace => write!(f, "No space for data"),
            kIOReturnPortExists => write!(f, "Port already exists"),
            kIOReturnCannotWire => write!(f, "Cannot wire down physical memory"),
            kIOReturnNoInterrupt => write!(f, "No interrupt attached"),
            kIOReturnNoFrames => write!(f, "No DMA frames enqueued"),
            kIOReturnMessageTooLarge => write!(f, "Oversized message on interrupt port"),
            kIOReturnNotPermitted => write!(f, "Not permitted"),
            kIOReturnNoPower => write!(f, "No power to device"),
            kIOReturnNoMedia => write!(f, "Media not present"),
            kIOReturnUnformattedMedia => write!(f, "Media not formatted"),
            kIOReturnUnsupportedMode => write!(f, "Unsupported mode"),
            kIOReturnUnderrun => write!(f, "Data underrun"),
            kIOReturnOverrun => write!(f, "Data overrun"),
            kIOReturnDeviceError => write!(f, "Device is not working properly"),
            kIOReturnNoCompletion => write!(f, "A completion routine is required"),
            kIOReturnAborted => write!(f, "Operation aborted"),
            kIOReturnNoBandwidth => write!(f, "Bus bandwidth would be exceeded"),
            kIOReturnNotResponding => write!(f, "Device not responding"),
            kIOReturnIsoTooOld => write!(f, "Isochronous I/O request for distant past"),
            kIOReturnIsoTooNew => write!(f, "Isochronous I/O request for distant future"),
            kIOReturnNotFound => write!(f, "Data was not found"),
            kIOReturnInvalid | _ => write!(f, "(Invalid error code)"),
        }
    }
}

impl std::error::Error for PlatformError {}

impl FallibleContext for Context {
    type PlatformError = PlatformError;
}
