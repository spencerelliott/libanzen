use crate::util::types::{OkResult, ValidResult};
use crate::pvr::transfer_protocol::DataTransferProtocol;

// =====================
// ==== Basic Types ====
// =====================

pub type PowerVRList = u8;

/// Encapsulates the response given when creating a PowerVR context
pub type PowerVRContextResult = OkResult<PowerVRContext>;

/// Encapsulates the response of starting
pub type PowerVRPassResult = ValidResult<PowerVRPass>;

// ===============
// ==== Enums ====
// ===============


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

pub struct PowerVRPass {

}