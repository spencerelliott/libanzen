use crate::util::types::{OkResult, ValidResult};
use crate::pvr::transfer_protocol::DataTransferProtocol;

// =====================
// ==== Basic Types ====
// =====================

pub type DrawList = u8;

/// Encapsulates the response given when creating a PowerVR context
pub type ContextResult = OkResult<Context>;

/// Encapsulates the response of starting
pub type PassResult = ValidResult<RenderPass>;

// =================
// ==== Structs ====
// =================

/// Handles initializing, creating, and destroying the context for the PowerVR.
pub struct PowerVR { }

/// Responsible for communicating with the PowerVR chip while also persisting the current context of
/// the device.
pub struct Context {
    pub(crate) valid: bool,
    pub(crate) transfer_protocol: dyn DataTransferProtocol
}

pub struct RenderPass {

}

pub struct DisplayList { }