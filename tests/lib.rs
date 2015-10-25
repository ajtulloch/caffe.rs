#![feature(convert)]

extern crate caffe;

#[test]
fn test_alexnet() {
    use std::path::Path;
    use std::iter::repeat;
    let mut net = caffe::Net::new(Path::new("test-data/lenet.prototxt"),
                                  caffe::Phase::Test);
    net.copy_trained_layers_from(Path::new("test-data/lenet.caffemodel"));
    let mut data_blob = net.blob("data");
    let mut ones: Vec<_> = repeat(1.0 as f32).take(data_blob.len()).collect();
    data_blob.set_data(ones.as_mut_slice());
    net.forward_prefilled();
    let prob_blob = net.blob("prob");
    let probs = prob_blob.as_slice();
    println!("{:?}", probs.to_vec());
    assert_eq!(probs.len(), 10); // 10 classes
    assert!((probs[0] - 0.06494621).abs() < 1E-4)
}

#[test]
fn test_solver() {
    use std::path::Path;
    let mut solver = caffe::Solver::new(Path::new("test-data/lenet_solver.prototxt"));
    solver.solve();
}
