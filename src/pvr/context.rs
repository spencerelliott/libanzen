//! let mut pvr_ctx;
//
// if let PowerVRContextResult::Ok(ctx) = PowerVR::create_context() {
//     pvr_ctx = ctx;
// }
//
// pvr_ctx.wait_ready();
// pvr_ctx.start_frame();
//
// if let PowerVRPassResult::Valid(pass1) = pvr_ctx.create_pass() {
//   if let PowerVRDisplayListResult::Valid(op_list) = pass1.list(PVR_LIST_OP) { // Can only be called for PVR_LIST_OP once per pass
//     // Draw all vertices in OP list
//   }
//   // op_list will finalize the list when going out of scope
//
//   if let PowerVRDisplayListResult::Valid(tr_list) = pass1.list(PVR_LIST_TR) {
//     // Draw all vertices in TR list
//   }
//   // tr_list will finalize the list when going out of scope
// }
// // Pass 1 will write to the needed registers when going out of scope
//
// // The pvr module will handle setting up the TA registers for the second pass
// if let PowerVRPass::Valid(pass2) = pvr_ctx.create_pass() {
//   if let PowerVRDisplayListResult::Valid(pt_list) = pass2.list(PVR_LIST_PT) {
//     // Draw all vertices in PT list
//   }
//   // pt_list will finalize the list when going out of scope
// }
// // Pass 2 will write to the needed registers when going out of scope
//
// pvr_ctx.end_frame();


use crate::pvr::types::{PowerVR, PowerVRContext, PowerVRContextResult};
use crate::pvr::transfer_protocol::{DataTransferProtocol, SQDataTransferProtocol};

extern "C" {
    fn dc_setup_ta();
}

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

/// Determines whether a new [PowerVRContext] can be created when [PowerVR::create_context()] is called.
static mut POWERVR_CONTEXT_TAKEN: bool = false;

impl PowerVR {
    /// Creates and returns a new [PowerVRContext] if one does not already exist.
    ///
    /// # Returns
    /// A new [PowerVRContext] if one has not already been created, otherwise this will return
    /// [PowerVRContextResult::Error].
    pub fn create_context() -> PowerVRContextResult {
        if unsafe { POWERVR_CONTEXT_TAKEN } {
            return PowerVRContextResult::Error("PowerVR context is already in use")
        }

        unsafe {
            if !POWERVR_INITIALIZED {
                dc_setup_ta();
            }
            POWERVR_INITIALIZED = true;
            POWERVR_CONTEXT_TAKEN = true;
        }

        PowerVRContextResult::Ok(PowerVRContext {
            valid: true,
            transfer_protocol: SQDataTransferProtocol {}
        })
    }

    /// Destroys the current [PowerVRContext]. This consumes the parameter passed into it and
    /// allows for a new context to be created using [PowerVR::create_context()].
    ///
    /// # Arguments
    /// * `ctx` - The current [PowerVRContext]
    pub fn destroy_context(ctx: &mut PowerVRContext) {
        unsafe {
            POWERVR_CONTEXT_TAKEN = false;
        }

        ctx.valid = false;
    }
}

impl PowerVRContext {
    fn wait_ready(&self) {
        return_if_context_invalid!(self);

    }

    fn start_frame(&self) {
        return_if_context_invalid!(self);
    }
}