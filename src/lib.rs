//! # `caffe.rs`<a id="orgheadline5"></a>
//!
//! A Rust FFI wrapper for the [Caffe](http://caffe.berkeleyvision.org/) deep learning library, using [rust-bindgen](https://github.com/crabtw/rust-bindgen).
//!
//! ## Setup<a id="orgheadline1"></a>
//!
//! Requires a `caffe` distribution built with the patches in
//! `ajtulloch/caffe:caffe-ffi` (<https://github.com/ajtulloch/caffe/tree/caffe-ffi>)
//! to expose the necessary structures over FFI.
//!
//! You can clone and build that repository as usual. Set the `CAFFE_ROOT`
//! environment variable to allow the `build.rs` script to correctly generate
//! dependencies.
//!
//! ## Example<a id="orgheadline4"></a>
//!
//! ### Inference on a pre-trained network<a id="orgheadline2"></a>
//!
//! ```ignore
//! use std::path::Path;
//! // Create the newtork
//! let mut net = caffe::Net::new(Path::new("test-data/lenet.prototxt"),
//!                               caffe::Phase::Test);
//! // Initialize the weights
//! net.copy_trained_layers_from(Path::new("test-data/lenet.caffemodel"));
//!
//! // Fill in the input data blob.
//! let mut data_blob = net.blob("data");
//! let mut ones: Vec<_> = repeat(1.0 as f32)
//!                        .take(data_blob.len())
//!                        .collect();
//! data_blob.set_data(ones.as_mut_slice());
//!
//! // Run a foward pass.
//! net.forward_prefilled();
//! let prob_blob = net.blob("prob");
//!
//! // Process the output probabilities.
//! let probs = prob_blob.as_slice();
//! println!("{:?}", probs.to_vec());
//! assert_eq!(probs[0], 0.06494621)
//! ```
//!
//! ### Running a solver<a id="orgheadline3"></a>
//!
//! ```ignore
//! use std::path::Path;
//! let mut solver = caffe::Solver::new(
//!     Path::new("test-data/lenet_solver.prototxt"));
//! solver.solve();
//! ```

#![feature(convert)]
#![feature(libc)]
#![feature(plugin)]
#![feature(unique)]
#![deny(missing_docs,
        missing_copy_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unused_import_braces,
        unused_qualifications)]

extern crate libc;


mod solver;
mod util;
mod net;
mod blob;
mod common;

pub mod ffi;
pub use common::*;
pub use net::*;
pub use solver::*;
pub use blob::*;
