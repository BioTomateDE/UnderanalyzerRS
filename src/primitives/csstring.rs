use core::slice;

use libgm::error::Context;

use crate::dynlib::free_cs_string;

#[repr(C)]
pub struct CsString {
    ptr: *const u8,
    len: usize,
}

impl CsString {
    /// Converts this [`CsString`] into an owned [`String`].
    /// This consumes the value to prevent use-after-free bugs and more.
    ///
    /// Empty strings should use a non-null pointer and specify a length of 0.
    /// Their pointer value will be ignored, so don't actually allocate anything
    /// in the C# side (otherwise memory leak).
    ///
    /// # Safety assertions
    /// * The `self.ptr` pointer must point to a valid UTF-8 string.
    /// * `self.len` must be the exact byte count of this buffer.
    /// * The buffer must have been allocated by CSharp's Marshal.
    pub unsafe fn to_str(&self) -> libgm::Result<&str> {
        let slice: &[u8] = unsafe { slice::from_raw_parts(self.ptr, self.len) };
        let string: &str = str::from_utf8(slice)
            .map_err(|e| e.to_string())
            .context("validating UTF-8")?;

        Ok(string)
    }
}

impl Drop for CsString {
    fn drop(&mut self) {
        // In order to drop the CsString properly, it needs to
        // call a Marshal function exported by CSharp.
        free_cs_string(self.ptr);
    }
}
