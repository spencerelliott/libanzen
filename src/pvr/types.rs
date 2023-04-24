use crate::pvr::transfer_protocol::DataTransferProtocol;

// =====================
// ==== Basic Types ====
// =====================

pub type PowerVRList = u8;
pub type PowerVRContextResult = PowerVRResult<PowerVRContext>;

// ===============
// ==== Enums ====
// ===============

/// Encapsulates the response of a PowerVR call that requires a response.
pub enum PowerVRResult<T> {
    /// Returned on a successful message call.
    Ok(T),
    /// Returned when an error is encountered during a message call.
    Error(&'static str)
}

// =================
// ==== Structs ====
// =================

/// Handles initializing, creating, and destroying the context for the PowerVR.
pub struct PowerVR { }

/// Responsible for communicating with the PowerVR chip while also persisting the current context of
/// the device.
pub struct PowerVRContext {
    pub(crate) valid: bool,
    pub(crate) transfer_protocol: dyn DataTransferProtocol
}