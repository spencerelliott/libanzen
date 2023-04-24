use crate::util;
use crate::pvr::types::{PowerVRResult, PowerVRList};

const TA_SQ_ADDR: util::MemoryAddress = util::mem_addr(&mut (0xe0000000 | (0x10000000 & 0x03ffffe0)));

const PVR_LIST_OP: PowerVRList = 0;