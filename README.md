# libanzen

**安全 (anzen)** - safety; security

This goal of this library is to provide a high-level abstraction layer in Rust for the Sega Dreamcast. Currently, this project
contains bindings to [libronin](https://github.com/spencerelliott/libronin) to access and interact with the hardware, but the ultimate 
goal of this project is to remove the dependency and provide a Rust-only abstraction layer.

## Getting Started

### Toolchain
GCC13 added support for the Rust programming language as a front-end. In order to be able to build binaries for the SuperH4
processor, a custom toolchain is required.

#### Build prerequisities

Using your package manager, make sure the following libraries are installed on your system:

```
sudo apt install gawk patch bzip2 tar make libgmp-dev libmpfr-dev libmpc-dev gettext wget libelf-dev texinfo bison flex sed git build-essential diffutils curl libjpeg-dev libpng-dev python3
```

#### Downloading the source

Create the folder `/opt/toolchains/dc-gcc13` in your filesystem. Navigate to your new folder and checkout the latest version
of the KallistiOS repository:

```shell
git clone -b gcc13 https://github.com/KallistiOS/KallistiOS.git kos
```

#### Creating the configuration file

Navigate to `/opt/toolchains/dc-gcc13/kos/utils/dc-chain` and run the following command:

```shell
mv config.mk.bleeding.sample config.mk
```

This will copy the default configuration for compiling GCC13. Next, change the following variables in `config.mk`:

```shell
toolchains_base=/opt/toolchains/dc-gcc13
pass2_languages=c,c++,objc,obj-c++,rust
thread_model=single
auto_fixup_sh4_newlib=0
```

#### Preparing the required libraries and source code

The dc-chain script requires the source for the libraries to be downloaded to specific folders. Multiple scripts are provided
in the repository to assist with setup. Run the following commands to prepare the workspace:

```shell
./download.sh
./unpack.sh
```

Once the sources have been downloaded and unpacked, the GCC12 sources can be safely deleted, if desired. Finally, the latest
GCC13 sources will need to be checked out:

```shell
git clone https://github.com/gcc-mirror/gcc.git gcc-13
```

#### Compiling the toolchain

Edit `config.mk` one last time and change the `sh_gcc_ver` to `13`:

```shell
sh_gcc_ver=13
```

Now, run `make` inside of the folder and a newly compiled toolchain should exist in
`/opt/toolchains/dc-gcc13/`. Add `/opt/toolchains/dc-gcc13/sh-elf/bin` and `/opt/toolchains/dc-gcc13/arm-eabi/bin` to 
your `PATH` variable and everything should be set up!

## Installing cargo-gccrs

In order to use the `cargo` command with GCCRS, a custom plugin for cargo has been developed. Using an existing Rust
run the following command to install the plugin:

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

Navigate to the libanzen folder using your terminal and run the following command:

```shell
./add_to_project.sh /path/to/my_cool_project
```

The script will handle copying over the required source files, libraries, and cargo configuration files needed to compile
the resulting ELF binary. Make sure to run the `add_to_project.sh` script every time this repository is updated to make sure
the latest sources are being used in a project.

Navigate to your project's directory and clean the project before building it for the first time:

```shell
GCCRS_CUSTOM_BIN="sh-elf-gccrs" cargo gccrs clean
```

After cleaning for the first time, it should be possible to build the project using:

```shell
GCCRS_CUSTOM_BIN="sh-elf-gccrs" cargo gccrs build
```

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
