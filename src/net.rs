use ffi;
use util;
use std::path::Path;
use blob::Blob;
use common::Phase;
use std::ptr::Unique;

/// A wrapper around a caffe::Net
pub struct Net {
    net: Unique<ffi::Struct_CaffeNet>,
}

impl Drop for Net {
    fn drop(&mut self) {
        unsafe { ffi::CaffeNetFree(self.net.get_mut()) }
    }
}

impl Net {
    /// Construct a caffe::Net from a serialized caffe::NetParameter and a Phase.
    pub fn new(path: &Path, phase: Phase) -> Net {
        let path = util::to_c_str(path.to_str().unwrap());
        let phase = match phase {
            Phase::Train => ffi::CAFFE_TRAIN,
            Phase::Test => ffi::CAFFE_TEST,
        };
        let net = unsafe { Unique::new(ffi::CaffeNetInit(path, phase)) };
        Net::wrap(net)
    }

    /// Wrap an existing caffe::Net over FFI.
    pub fn wrap(net: Unique<ffi::Struct_CaffeNet>) -> Net {
        assert!(!net.is_null());
        Net { net: net }
    }

    /// Set the weights of the network from the binary serialized caffe::NetParameter.
    pub fn copy_trained_layers_from(&mut self, path: &Path) {
        let path = util::to_c_str(path.to_str().unwrap());
        unsafe { ffi::CaffeNetCopyTrainedLayersFrom(self.net.get_mut(), path) }
    }

    /// Extract a pointer to the blob in the net.
    pub fn blob(&mut self, name: &str) -> Blob {
        let name = util::to_c_str(name);
        let blob = unsafe {
            Unique::new(ffi::CaffeNetGetBlobByName(self.net.get_mut(), name))
        };
        Blob::wrap(blob)
    }

    /// Run a forward pass on the prefilled input blobs.
    pub fn forward_prefilled(&mut self) {
        unsafe { ffi::CaffeNetForwardPrefilled(self.net.get_mut()) }
    }
}
