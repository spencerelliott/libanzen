extern "C" {
    pub fn usleep(us: u32);
}

pub struct Time { }

impl Time {
    /// Sleep the console for the desired amount of microseconds
    pub fn usleep(us: u32) {
        unsafe {
            usleep(us);
        }
    }
}