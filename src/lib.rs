//! # libanzen
//!
//! This goal of this library is to provide a high-level abstraction layer in Rust for the Sega
//! Dreamcast. Currently, this project contains bindings to [libronin](https://github.com/spencerelliott/libronin)
//! to access and interact with the hardware, but the ultimate goal of this project is to remove the
//! dependency and provide a Rust-only abstraction layer.
//!
//! ## Usage
//!
//! To get started with libanzen, create a new binary Rust project with Cargo:
//!
//! ```shell
//! cargo gccrs new --bin my_cool_project
//! ```
//!
//! The default `main.rs` generated will not be able to compile for our target, so replace it with
//! the following code:
//!
//! ```rust
//! #![no_std]
//!
//! #[no_mangle]
//! pub extern "C" fn main() -> u32 {
//!     return 0;
//! }
//! ```
//!
//! Additionally, add the following lines to `Cargo.toml` to make sure cargo will output an
//! actual ELF binary:
//!
//! ```toml
//! [[bin]]
//! name = "my_cool_project.elf"
//! path = "src/main.rs"
//! ```
//!
//! Navigate to the libanzen folder using your terminal and run the following command:
//!
//! ```shell
//! ./add_to_project.sh /path/to/my_cool_project
//! ```
//!
//! The script will handle copying over the required source files, libraries, and cargo configuration files needed to compile
//! the resulting ELF binary. Make sure to run the `add_to_project.sh` script every time this repository is updated to make sure
//! the latest sources are being used in a project.
//!
//! ## Modules
//! Currently, `libanzen` supports the following modules:
//! * `serial` - Communication through the serial port
//! * `time` - Handles sleep calls
//! * `cdfs` - Reading data from the GD-ROM drive and playing CDDA audio
//!
//! To use a module in a project, add the following line(s) to the top of your source file:
//!
//! ```rust
//! mod serial;
//! mod time;
//! ...
//! ```
//!
//! Once a module has been added, the specific structs and methods can be included like so:
//!
//! ```rust
//! use serial::Serial;
//! ```

pub mod serial;
pub mod time;
pub mod cdfs;
