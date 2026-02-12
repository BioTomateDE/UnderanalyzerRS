#[repr(C)]
pub struct RawArray<T> {
    ptr: *const T,
    len: usize,
    cap: usize,
}

impl<T> RawArray<T> {
    #[must_use]
    pub const fn from_vec(vector: Vec<T>) -> Self {
        const { assert!(size_of::<T>() != 0, "ZSTs are not supported") }

        let ptr: *const T = vector.as_ptr();
        let len: usize = vector.len();
        let cap: usize = vector.capacity();

        // Leak the vector so that it doesn't drop immediately
        std::mem::forget(vector);

        Self { ptr, len, cap }
    }
}

impl<T> Drop for RawArray<T> {
    fn drop(&mut self) {
        let ptr: *mut T = self.ptr.cast_mut();
        let len: usize = self.len;
        let cap: usize = self.cap;
        drop(unsafe { Vec::from_raw_parts(ptr, len, cap) });
    }
}

unsafe impl<T: Send> Send for RawArray<T> {}
unsafe impl<T: Sync> Sync for RawArray<T> {}
