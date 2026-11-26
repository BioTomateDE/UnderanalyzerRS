#[repr(C)]
pub struct RawArray<T> {
    ptr: *mut T,
    len: u32,
}

impl<T> RawArray<T> {
    #[must_use]
    pub fn from_boxed_slice(boxed_slice: Box<[T]>) -> Self {
        const { assert!(size_of::<T>() > 0, "ZSTs are not supported") }
        let len = u32::try_from(boxed_slice.len()).expect("Extremely massive slice");
        let ptr = Box::into_raw(boxed_slice).cast();
        Self { ptr, len }
    }
}

impl<T> Drop for RawArray<T> {
    fn drop(&mut self) {
        let Self { ptr, len } = *self;
        // TODO: is this sound?
        let slice: &mut [T] = unsafe { core::slice::from_raw_parts_mut(ptr, len as usize) };
        let boxed_slice: Box<[T]> = unsafe { Box::from_raw(slice) };
        drop(boxed_slice);
    }
}

unsafe impl<T: Send> Send for RawArray<T> {}
unsafe impl<T: Sync> Sync for RawArray<T> {}
