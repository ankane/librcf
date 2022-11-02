use rcflib::rcf::{create_rcf, RCF};
use std::slice;

#[allow(non_camel_case_types)]
pub struct rcf_forest(Box<dyn RCF>);

#[no_mangle]
pub extern "C" fn rcf_create(dimensions: usize) -> *mut rcf_forest {
    let shingle_size = 1;
    let sample_size = 256;
    let number_of_trees = 100;
    let random_seed = 42;
    let store_attributes = false;
    let parallel_enabled = false;
    let internal_shingling = false;
    let internal_rotation = false;
    let time_decay = 0.0;
    let initial_accept_fraction = 1.0;
    let bounding_box_cache_fraction = 1.0;

    let forest = create_rcf(
        dimensions,
        shingle_size,
        sample_size,
        number_of_trees,
        random_seed,
        store_attributes,
        parallel_enabled,
        internal_shingling,
        internal_rotation,
        time_decay,
        initial_accept_fraction,
        bounding_box_cache_fraction,
    );

    Box::into_raw(Box::new(rcf_forest(forest)))
}

#[no_mangle]
pub unsafe extern "C" fn rcf_update(forest: *mut rcf_forest, point: *const f32) {
    let slice = slice::from_raw_parts(point, (*forest).0.dimensions());
    (*forest).0.update(slice, 0).unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn rcf_score(forest: *mut rcf_forest, point: *const f32) -> f64 {
    let slice = slice::from_raw_parts(point, (*forest).0.dimensions());
    (*forest).0.score(slice).unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn rcf_free(forest: *mut rcf_forest) {
    Box::from_raw(forest);
}
