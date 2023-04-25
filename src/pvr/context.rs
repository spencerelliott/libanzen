//! ```rust
//! let mut pvr_ctx;
//!
//! if let ContextResult::Ok(ctx) = PowerVR::create_context() {
//!     pvr_ctx = ctx;
//! }
//!
//! pvr_ctx.wait_ready();
//! pvr_ctx.start_frame();
//!
//! if let RenderPassResult::Valid(pass1) = pvr_ctx.create_pass() {
//!   if let DisplayListResult::Valid(op_list) = pass1.list(PVR_LIST_OP) { // Can only be called for PVR_LIST_OP once per pass
//!     // Draw all vertices in OP list
//!     op_list.submit();
//!   }
//!   // op_list will finalize the list when going out of scope (when core lib is supported)
//!
//!   if let DisplayListResult::Valid(tr_list) = pass1.list(PVR_LIST_TR) {
//!     // Draw all vertices in TR list
//!     tr_list.submit();
//!   }
//!   // tr_list will finalize the list when going out of scope (when core lib is supported)
//!   pass1.submit();
//! }
//! // Pass 1 will write to the needed registers when going out of scope (when core lib is supported)
//!
//! // The pvr module will handle setting up the TA registers for the second pass
//! if let RenderPassResult::Valid(pass2) = pvr_ctx.create_pass() {
//!   if let DisplayListResult::Valid(pt_list) = pass2.list(PVR_LIST_PT) {
//!     // Draw all vertices in PT list
//!     pt_list.submit();
//!   }
//!   // pt_list will finalize the list when going out of scope (when core lib is supported)
//!   pass2.submit();
//! }
//! // Pass 2 will write to the needed registers when going out of scope (when core lib is supported)
//!
//! pvr_ctx.end_frame();
//! ```


use crate::pvr::types::{PowerVR, Context, ContextResult, RenderPass, PassResult};
use crate::pvr::transfer_protocol::{DataTransferProtocol, SQDataTransferProtocol};

extern "C" {
    fn dc_setup_ta();
}

/// Determines if a PowerVR context is valid and returns if the context is invalid.
macro_rules! return_if_context_invalid {
    ( $ctx:expr ) => {
        {
            if !$ctx.valid {
                return;
            }
        }
    };
    ( $ctx:expr, $ret:expr ) => {
        {
            if !$ctx.valid {
                return $ret;
            }
        }
    };
}

/// Determines whether the PowerVR device needs to be initialized on context creation.
static mut POWERVR_INITIALIZED: bool = false;

/// Determines whether a new [Context] can be created when [PowerVR::create_context()] is called.
static mut POWERVR_CONTEXT_TAKEN: bool = false;

impl PowerVR {
    /// Creates and returns a new [Context] if one does not already exist.
    ///
    /// # Returns
    /// A new [Context] if one has not already been created, otherwise this will return
    /// [ContextResult::Error].
    pub fn create_context() -> ContextResult {
        if unsafe { POWERVR_CONTEXT_TAKEN } {
            return ContextResult::Error("PowerVR context is already in use")
        }

        unsafe {
            if !POWERVR_INITIALIZED {
                dc_setup_ta();
            }
            POWERVR_INITIALIZED = true;
            POWERVR_CONTEXT_TAKEN = true;
        }

        ContextResult::Ok(Context {
            valid: true,
            transfer_protocol: SQDataTransferProtocol {}
        })
    }

    /// Destroys the current [Context]. This consumes the parameter passed into it and
    /// allows for a new context to be created using [PowerVR::create_context()].
    ///
    /// # Arguments
    /// * `ctx` - The current [Context]
    pub fn destroy_context(ctx: &mut Context) {
        unsafe {
            POWERVR_CONTEXT_TAKEN = false;
        }

        ctx.valid = false;
    }
}

impl Context {
    fn wait_ready(&self) {
        return_if_context_invalid!(self);

    }

    fn start_frame(&self) {
        return_if_context_invalid!(self);

    }

    fn create_pass(&self) -> PassResult {
        return_if_context_invalid!(self, PassResult::Invalid("Context is not valid"));

        PassResult::Valid(RenderPass {

        })
    }

    fn submit(&self) {
        return_if_context_invalid!(self);

    }
}
