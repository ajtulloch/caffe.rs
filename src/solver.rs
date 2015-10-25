use ffi;
use util;
use net::Net;
use std::path::Path;
use std::ptr::Unique;

/// A wrapper around a caffe::Solver for training networks.
pub struct Solver {
    solver: Unique<ffi::Struct_CaffeSolver>,
}

impl Solver {
    /// Construct a solver from a serialized caffe::SolverParameter.
    pub fn new(path: &Path) -> Solver {
        let path = util::to_c_str(path.to_str().unwrap());
        let solver = unsafe { Unique::new(ffi::CaffeSolverInit(path)) };
        Solver { solver: solver }
    }

    /// Solve the net.
    pub fn solve(&mut self) {
        unsafe { ffi::CaffeSolverSolve(self.solver.get_mut()) }
    }

    /// Extract a pointer to the net used by the solver.
    pub fn net(&mut self) -> Net {
        let net = unsafe {
            Unique::new(ffi::CaffeSolverNet(self.solver.get_mut()))
        };
        Net::wrap(net)
    }
}

impl Drop for Solver {
    fn drop(&mut self) {
        unsafe { ffi::CaffeSolverFree(self.solver.get_mut()) }
    }
}
