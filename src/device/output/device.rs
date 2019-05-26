use crate::sdk;
use std::ptr::null_mut;
use std::sync::atomic::AtomicBool;

unsafe impl Send for DecklinkOutputDevicePtr {}
unsafe impl Sync for DecklinkOutputDevicePtr {}
pub struct DecklinkOutputDevicePtr {
    pub dev: *mut crate::sdk::cdecklink_output_t,
    pub video_active: AtomicBool,
    pub audio_active: AtomicBool,
}
impl Drop for DecklinkOutputDevicePtr {
    fn drop(&mut self) {
        if !self.dev.is_null() {
            unsafe { sdk::cdecklink_output_release(self.dev) };
            self.dev = null_mut();
        }
    }
}
