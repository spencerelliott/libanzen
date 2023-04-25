use crate::util::types::{OkResult, ValidResult};
use crate::pvr::transfer_protocol::DataTransferProtocol;

// =====================
// ==== Basic Types ====
// =====================

pub type DrawList = u8;
pub type SubmissionList = u8;

/// Encapsulates the response given when creating a PowerVR context
pub type ContextResult = OkResult<Context>;

/// Encapsulates the response of starting a render pass
pub type PassResult = ValidResult<RenderPass<'_>>;

/// Encapsulates the response of creating a display list to submit vertices
pub type DisplayListResult = ValidResult<DisplayList<'_, '_>>;

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

pub struct RenderPass<'a> {
    pub(crate) context: &'a Context,
    pub(crate) submitted_lists: u8,
    pub(crate) open_lists: u8
}

pub struct DisplayList<'a, 'b> {
    pub(crate) pass: &'b RenderPass<'a>,
    pub(crate) list: DrawList
}

#[repr(C)]
pub struct Vertex {
    x: f32,
    y: f32,
    z: f32
}
