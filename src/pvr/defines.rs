use crate::util::memory;
use crate::pvr::types::{DrawList, SubmissionList};

pub const TA_SQ_ADDR: memory::MemoryAddress = memory::mem_addr(&mut (0xe0000000 | (0x10000000 & 0x03ffffe0)));
