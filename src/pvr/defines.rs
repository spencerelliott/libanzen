use crate::util::memory;
use crate::pvr::types::PowerVRList;

const TA_SQ_ADDR: memory::MemoryAddress = memory::mem_addr(&mut (0xe0000000 | (0x10000000 & 0x03ffffe0)));

const PVR_LIST_OP: PowerVRList = 0;