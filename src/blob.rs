use std::slice;
use ffi;
use std::ptr::Unique;

/// Wrapper onto a caffe::Blob.
pub struct Blob {
    blob: Unique<ffi::Struct_CaffeBlob>,
}

impl Blob {
    /// Take ownership of the FFI blob.
    pub fn wrap(blob: Unique<ffi::Struct_CaffeBlob>) -> Blob {
        assert!(!blob.is_null());
        Blob { blob: blob }
    }

    /// Number of elements in the blob.
    pub fn len(&self) -> usize {
        unsafe { ffi::CaffeBlobCount(self.blob.get()) as usize }
    }

    /// Blob empty?
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Represent the blob data as a slice.
    pub fn as_slice(&self) -> &[f32] {
        let count: usize = self.len();
        unsafe {
            let data = ffi::CaffeBlobCPUData(self.blob.get());
            slice::from_raw_parts(data, count)
        }
    }

    /// Represent the blob data as a mutable slice.
    pub fn as_mut_slice(&mut self) -> &mut [f32] {
        let count: usize = self.len();
        unsafe {
            let data = ffi::CaffeBlobMutableCPUData(self.blob.get_mut());
            slice::from_raw_parts_mut(data, count)
        }
    }

    /// Set the blob data to the data pointer.
    pub fn set_data(&mut self, data: &mut [f32]) -> () {
        assert_eq!(data.len(), self.len());
        unsafe {
            ffi::CaffeBlobSetCPUData(self.blob.get_mut(), data.as_mut_ptr())
        }
    }
}

impl Drop for Blob {
    fn drop(&mut self) {
        unsafe { ffi::CaffeBlobFree(self.blob.get_mut()) }
    }
}
