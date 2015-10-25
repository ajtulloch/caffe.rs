use ffi;

/// The computation phase that Caffe runs with.
#[derive(Debug, Copy, Clone)]
pub enum Phase {
    /// Train phase
    Train,
    /// Test phase
    Test,
}

/// The computation mode that Caffe runs with.
#[derive(Debug, Copy, Clone)]
pub enum Mode {
    /// CPU mode
    CPU,
    /// GPU mode
    GPU,
}

/// Set the computation mode to CPU/GPU
pub fn set_mode(mode: Mode) {
    let enum_mode = match mode {
        Mode::GPU => ffi::CAFFE_GPU,
        Mode::CPU => ffi::CAFFE_CPU,
    };
    unsafe { ffi::CaffeSetMode(enum_mode) }
}
