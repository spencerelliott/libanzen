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

/// The maximum size of data that can be read from disc at a time.
pub const DATA_CHUNK_MAX_SIZE: usize = 1024;

/// Opens a file from disc as read-only.
pub const O_RDONLY: i32 = 0;

/// Opens as a directory.
pub const O_DIR: i32 = 4;

/// Used to seek to an exact offset.
pub const SEEK_SET: i32 = 0;

/// Used to seek from the current offset of the file.
pub const SEEK_CUR: i32 = 1;

/// Used to seek from the end of the file.
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

pub enum CdfsResult<T> {
    Ok(T),
    Error
}

type DirectoryResult = CdfsResult<Directory>;

/// Encapsulates the response given when reading data from disc.
type DataReadResult = CdfsResult<DataChunk>;

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
    ///
    /// # Arguments
    /// * `index` - The index of the byte in the buffer
    ///
    /// # Returns
    /// The value of the byte at the location in the buffer. This will return 0 if the `index`
    /// is greater than the actual size of the read buffer.
    pub fn get_byte(&self, index: usize) -> u8 {
        if index > self.size {
            return 0;
        }

        self.data[index]
    }

    /// Retrieves a single byte from this data chunk as a character. This will always return
    /// zero if the `index` exceeds `size`.
    ///
    /// # Arguments
    /// * `index` - The index of the character in the buffer
    ///
    /// # Returns
    /// The value of the character at the location in the buffer. This will return 0 if the `index`
    /// is greater than the actual size of the read buffer.
    pub fn get_char(&self, index: usize) -> char {
        self.get_byte(index) as char
    }
}

type CdfsOpenResult = CdfsResult<CdfsFile>;

/// Represents a currently open file from the disc.
pub struct CdfsFile {
    fd: i32
}

impl CdfsFile {
    /// Reads the desired amount of bytes from this file.
    ///
    /// # Arguments
    /// * `nbytes` - The number of bytes to read. Can be a maximum of [DATA_CHUNK_MAX_SIZE].
    ///
    /// # Returns
    /// [DataReadResult::Ok] containing the [DataChunk] when successful, [DataReadResult::Error] otherwise.
    pub fn read(&self, nbytes: usize) -> DataReadResult {
        let mut buf: [u8; DATA_CHUNK_MAX_SIZE] = [0; DATA_CHUNK_MAX_SIZE];
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

    /// Retrieves the size of this file.
    ///
    /// # Returns
    /// The size of the file in bytes or -1 if a size could not be determined.
    pub fn file_size(&self) -> i32 {
        let mut size: i32 = -1;

        unsafe {
            size = file_size(self.fd);
        }

        size
    }

    /// Seeks within the file based on the offset.
    ///
    /// # Arguments
    /// * `offset` - The offset to apply based on `from`
    /// * `from` - Can be either [SEEK_SET], [SEEK_CUR], or [SEEK_END]
    ///
    /// # Returns
    /// The new offset the file is pointing to, in bytes.
    pub fn seek(&self, offset: i64, from: i32) -> i32 {
        let mut new_offset: i32 = -1;

        unsafe {
            new_offset = lseek(self.fd, offset, from);
        }

        new_offset
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
    /// Attempts to open a file from the disc.
    ///
    /// # Arguments
    /// * `path` - The path to the file on the disc filesystem.
    /// * `oflags` - Can be either [O_RDONLY] or [O_DIR].
    ///
    /// # Returns
    /// [CdfsOpenResult::Ok] containing the [CdfsFile] on successful open, [CdfsOpenResult::Error] otherwise.
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

    pub fn open_dir(path: &str) -> *const Directory {
        let mut dir: Directory = Directory {
            dd_size: 0,
            dd_fd: 0,
            dd_loc: 0,
            dd_buf: 0 as *mut i8
        };

        unsafe {
            //dir = opendir(path);
        }

        &dir
    }
}
