
use crate::pvr::types::{PowerVR, PowerVRContext, PowerVRContextResult};
use crate::pvr::transfer_protocol::{DataTransferProtocol, SQDataTransferProtocol};

extern "C" {
    fn dc_setup_ta();
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

impl PowerVRContext {

}