//! # PowerVR
//!
//! This modules provides an interface to the PowerVR found within the Holly chip inside of the
//! Dreamcast.
//!
//! ## Usage
//!
//! ```rust
//! let mut pvr_ctx;
//!
//! if let PowerVRContextResult::Ok(ctx) = PowerVR::create_context() {
//!     pvr_ctx = ctx;
//! }
//!
//! PowerVR::destroy_context(pvr_ctx);
//! ```

pub mod defines;
pub mod types;
pub mod context;
pub mod transfer_protocol;
