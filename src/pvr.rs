//! # PowerVR
//!
//! This modules provides an interface to the PowerVR found within the Holly chip inside of the
//! Dreamcast.
//!
//! ## Usage
//!
//! ```rust
//! let ctx_result: PowerVRContextResult = PowerVR::create_context();
//! let mut pvr_ctx;
//!
//! match ctx_result {
//!     PowerVRContextResult::Ok(ctx) => {
//!         pvr_ctx = ctx;
//!     },
//!     _ => {
//!         // Report some error
//!     }
//! }
//!
//! PowerVR::destroy_context(pvr_ctx);
//! ```

use crate::util;

extern "C" {
    fn dc_setup_ta();
}

/// Determines whether the PowerVR device needs to be initialized on context creation.
static mut POWERVR_INITIALIZED: bool = false;

/// Determines whether a new [PowerVRContext] can be created when [PowerVR::create_context()] is called.
static mut POWERVR_CONTEXT_TAKEN: bool = false;

/// Encapsulates the response of a PowerVR call that requires a response.
pub enum PowerVRResult<T> {
    /// Returned on a successful message call.
    Ok(T),
    /// Returned when an error is encountered during a message call.
    Error(&'static str)
}

/// Handles initializing, creating, and destroying the context for the PowerVR.
struct PowerVR { }

/// Responsible for communicating with the PowerVR chip while also persisting the current context of
/// the device.
struct PowerVRContext { }

type PowerVRContextResult = PowerVRResult<PowerVRContext>;

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

        })
    }

    /// Destroys the current [PowerVRContext]. This consumes the parameter passed into it and
    /// allows for a new context to be created using [PowerVR::create_context()].
    ///
    /// # Arguments
    /// * `ctx` - The current [PowerVRContext]
    pub fn destroy_context(ctx: PowerVRContext) {
        unsafe { POWERVR_CONTEXT_TAKEN = false; }
    }
}

impl PowerVRContext {

}
