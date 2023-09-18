# libanzen

**安全 (anzen)** - safety; security

This goal of this library is to provide a high-level abstraction layer in Rust for the Sega Dreamcast. Currently, this project
contains bindings to [libronin](https://github.com/spencerelliott/libronin) to access and interact with the hardware, but the ultimate 
goal of this project is to remove the dependency and provide a Rust-only abstraction layer.

## Getting Started

### Toolchain
GCC13 added support for the Rust programming language as a front-end. In order to be able to build binaries for the SuperH4
processor, a custom build of the GCC toolchain is required.

#### Build prerequisities

Make sure the following libraries are installed on the host system:

```
sudo apt install gawk patch bzip2 tar make libgmp-dev libmpfr-dev libmpc-dev gettext wget libelf-dev texinfo bison flex sed git build-essential diffutils curl libjpeg-dev libpng-dev python3
```

#### Downloading the source

Create the folder `/opt/toolchains/dc-gccrs` to hold all the source code and resulting binaries. Navigate to the new 
folder and checkout the latest version of the KallistiOS repository:

```shell
git clone -b gccrs https://github.com/KallistiOS/KallistiOS.git kos
```

#### Creating the configuration file

Navigate to `/opt/toolchains/dc-gccrs/kos/utils/dc-chain` and run the following command:

```shell
cp config/config.mk.gccrs.sample config.mk
```

This will copy the default configuration for compiling GCCRS. Next, change the following variables in `config.mk`:

```shell
toolchains_base=/opt/toolchains/dc-gccrs
thread_model=single
auto_fixup_sh4_newlib=0
```
**Note:** If you would only like to use KallistiOS with Rust support, do not modify `thread_model` or `auto_fixup_sh4_newlib` in your configuration file.

#### Compiling the toolchain

Run `make` inside of the folder and a newly compiled toolchain should exist in
`/opt/toolchains/dc-gccrs/`. Add `/opt/toolchains/dc-gccrs/sh-elf/bin` and `/opt/toolchains/dc-gccrs/arm-eabi/bin` to 
your `PATH` variable and everything should be set up!

If GCCRS was compiled for use with KallistiOS and not libanzen, [follow these steps to compile KOS](https://dreamcast.wiki/Getting_Started_with_Dreamcast_development#Configuring_and_compiling_KOS_and_kos-ports)

## Installing cargo-gccrs

In order to use the `cargo` command with GCCRS, a custom plugin has been developed to use with cargo. Using an existing Rust
installation, run the following command to install the plugin:

```shell
cargo install --git https://github.com/spencerelliott/cargo-gccrs.git --branch main cargo-gccrs
```

This modified cargo-gccrs install allows the ability to modify the GCCRS binary used when compiling through
environment variables.

## Using libanzen

Create a new binary cargo project:

```shell
cargo gccrs new --bin my_cool_project
```

The default `main.rs` generated will not be able to compile for our target, so replace it with the following code:

```rust
#![no_std]

#[no_mangle]
pub extern "C" fn main() -> u32 {
    return 0;
}
```

Additionally, add the following lines to `Cargo.toml` to make sure cargo will output an
actual ELF binary:

```toml
[[bin]]
name = "my_cool_project.elf"
path = "src/main.rs"
```

Navigate to the libanzen folder using your terminal and run the following command:

```shell
./add_to_project.sh /path/to/my_cool_project
```

The script will handle copying over the required source files, libraries, and cargo configuration files needed to compile
the resulting ELF binary. Make sure to run the `add_to_project.sh` script every time this repository is updated to make sure
the latest sources are being used in a project.

Navigate to your project's directory and clean the project before building it for the first time:

```shell
GCCRS_INCOMPLETE_AND_EXPERIMENTAL_COMPILER_DO_NOT_USE=1 GCCRS_CUSTOM_BIN="sh-elf-gccrs" cargo gccrs clean
```

After cleaning for the first time, it should be possible to build the project using:

```shell
GCCRS_INCOMPLETE_AND_EXPERIMENTAL_COMPILER_DO_NOT_USE=1 GCCRS_CUSTOM_BIN="sh-elf-gccrs" cargo gccrs build
```

Both `GCCRS_INCOMPLETE_AND_EXPERIMENTAL_COMPILER_DO_NOT_USE` and `GCCRS_CUSTOM_BIN` can be added to your environment
variables if you do not want to pass them to `cargo` each time.

## Accessing modules

GCCRS is currently very limited in path resolution for crates. In order to get around the limitations, libanzen copies
all of the sources to your project directly. To use a specific module, add the following line to the top of your source file:

```rust
mod serial;
mod time;
...
```

Once a module has been added to your file, the specific structs and methods can be included like so:

```rust
use serial::Serial;
```

## Limitations

Currently, Rust support for the SuperH4 processor found in the Dreamcast is only available through [GCCRS](https://github.com/Rust-GCC/gccrs).
Many of the features that are standard in Rust such as the standard library, borrow checking, and crate support are currently
not available in GCCRS. 

To get around these limitations, the source files for `libanzen` need to be copied directly into
a project's source directory. A helper script (`add_to_project.sh`) has been provided in this repository to handle copying
the required files into a project. In the future, when GCCRS has the ability to handle external crates, the helper script
will be removed and this library will be able to be added to a project simply by adding a line to a project's `Cargo.toml`
file.
