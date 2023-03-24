//! # CDFS
//!
//! This module provides an interface to open and read files from the GD-ROM drive.
//!
//! ## Usage
//!
//! ```rust
//! if let CdfsOpenResult::Ok(file) = Cdfs::open("some_file.txt", O_RDONLY) {
//!     if let DataReadResult::Ok(data) = file.read(DATA_CHUNK_MAX_SIZE) {
//!         // Do something with your data
//!     }
//!
//!     file.close();
//! }
//! ```

extern "C" {
    fn open(path: &str, oflag: i32) -> i32;
    fn close(fd: i32) -> i32;
    fn pread(fd: i32, buffer: *mut u8, nbytes: u32, offset: i64) -> i32;
    fn read(fd: i32, buffer: *mut u8, nbytes: u32) -> i32;
    fn lseek(fd: i32, offset: i64, whence: i32) -> i32;
    fn opendir(dirname: &str) -> *mut Directory;
    fn closedir(dir: *mut Directory) -> i32;
    fn file_size(fd: i32) -> i32;
    fn readdir_r(dirp: *mut Directory, entry: *mut DirectoryEntry, res: *mut *mut DirectoryEntry);
    fn readdir(dirp: *mut Directory) -> *mut DirectoryEntry;
    fn chdir(path: &str) -> i32;
    fn cdfs_init();
    fn cdfs_reinit();
    fn cdfs_diskchanges() -> i32;
    fn cdfs_get_volume_id(buffer: *mut u8, length: u32) -> i32;
    fn play_cdda_tracks(start: i32, stop: i32, reps: i32) -> i32;
    fn play_cdda_sectors(start: i32, stop: i32, reps: i32) -> i32;
    fn stop_cdda();
    fn cdfs_gettoc() -> *mut TableOfContents;
}

/// The maximum size of data that can be read from disc at a time
pub const DATA_CHUNK_MAX_SIZE: usize = 1024;

/// Opens a file from disc as read-only
pub const O_RDONLY: i32 = 0;

/// Opens as a directory
pub const O_DIR: i32 = 4;

pub const SEEK_SET: i32 = 0;
pub const SEEK_CUR: i32 = 1;
pub const SEEK_END: i32 = 2;

#[repr(C)]
struct TableOfContents {
    entry: [u32; 99],
    first: u32,
    last: u32,
    dummy: u32
}

#[repr(C)]
struct Directory {
    dd_fd: i32,
    dd_loc: i32,
    dd_size: i32,
    dd_buf: *mut i8,
}

#[repr(C)]
struct DirectoryEntry {
    d_size: i32,
    d_name: [i8; 256]
}

/// Encapsulates the response given when reading data from disc.
pub enum DataReadResult {
    /// Returned when data has successfully been read from the discs.
    Ok(DataChunk),
    /// Returned if data could not be read.
    Error
}

/// Represents a chunk of data read from a disc.
#[repr(C)]
pub struct DataChunk {
    data: [u8; DATA_CHUNK_MAX_SIZE],
    size: usize,
}

impl DataChunk {
    /// Returns the actual size of the data in the buffer.
    pub fn actual_size(&self) -> usize {
        self.size
    }

    /// Returns the max size of this buffer.
    pub fn max_size(&self) -> usize {
        DATA_CHUNK_MAX_SIZE
    }

    /// Retrieves a single byte from this data chunk. This will always return
    /// zero if the `index` exceeds `size`.
    pub fn get_byte(&self, index: usize) -> u8 {
        if index > self.size {
            return 0;
        }

        self.data[index]
    }

    /// Retrieves a single byte from this data chunk as a character. This will always return
    /// zero if the `index` exceeds `size`.
    pub fn get_char(&self, index: usize) -> char {
        self.get_byte(index) as char
    }
}

pub enum CdfsOpenResult {
    Ok(CdfsFile),
    Error
}

/// Represents a currently open file from the disc.
pub struct CdfsFile {
    fd: i32
}

impl CdfsFile {
    /// Reads the desired amount of bytes from this file. The max amount of data that can be read in
    /// a single read is `cdfs::DATA_CHUNK_MAX_SIZE`.
    pub fn read(&self, nbytes: usize) -> DataReadResult {
        let mut buf: [u8; 1024] = [0; DATA_CHUNK_MAX_SIZE];
        let mut read_bytes = -1;

        // Make sure the user is not requesting more than is possible
        if nbytes > DATA_CHUNK_MAX_SIZE {
            return DataReadResult::Error;
        }

        unsafe {
            read_bytes = read(self.fd, (&mut buf[0]) as *mut u8, nbytes as u32);
        }

        // Make sure that we actually received some data back from CDFS
        if read_bytes <= 0 {
            return DataReadResult::Error;
        }

        DataReadResult::Ok(DataChunk {
            data: buf,
            size: read_bytes as usize
        })
    }

    /// Closes the file while also consuming the variable so it can no longer be used.
    pub fn close(self) {
        unsafe {
            close(self.fd);
        }
    }
}

pub struct Cdfs { }

impl Cdfs {
    /// Attempts to open a file from the disc. If the file is not available, `CdfsOpenResult::Error`
    /// will be returned.
    pub fn open(path: &str, oflags: i32) -> CdfsOpenResult {
        let mut fd = -1;

        unsafe {
            fd = open(path, oflags);
        }

        if fd < 0 {
            return CdfsOpenResult::Error;
        }

        CdfsOpenResult::Ok(CdfsFile {
            fd
        })
    }
}
