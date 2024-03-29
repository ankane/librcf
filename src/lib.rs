#![allow(clippy::missing_safety_doc)]

use rcflib::rcf::{RCFBuilder, RCFOptionsBuilder, RCF};
use std::ffi::{c_char, c_int, CStr};
use std::slice;

#[allow(non_camel_case_types)]
pub struct rcf_forest {
    rcf: Option<Box<dyn RCF>>,
    dimensions: usize,
    shingle_size: usize,
    sample_size: usize,
    number_of_trees: usize,
    random_seed: u64,
    parallel: bool,
}

unsafe fn ensure_forest(forest: *mut rcf_forest) {
    if (*forest).rcf.is_none() {
        // TODO make parameters
        let store_attributes = false;
        let internal_shingling = false;
        let internal_rotation = false;
        let time_decay = 0.0;
        let initial_accept_fraction = 1.0;
        let bounding_box_cache_fraction = 1.0;

        (*forest).rcf = Some(
            RCFBuilder::<u64, u64>::new(
                (*forest).dimensions / (*forest).shingle_size,
                (*forest).shingle_size,
            )
            .tree_capacity((*forest).sample_size)
            .number_of_trees((*forest).number_of_trees)
            .random_seed((*forest).random_seed)
            .store_attributes(store_attributes)
            .parallel_enabled((*forest).parallel)
            .internal_shingling(internal_shingling)
            .internal_rotation(internal_rotation)
            .time_decay(time_decay)
            .initial_accept_fraction(initial_accept_fraction)
            .bounding_box_cache_fraction(bounding_box_cache_fraction)
            .build_default()
            .unwrap(),
        );
    }
}

#[no_mangle]
pub extern "C" fn rcf_create(dimensions: usize) -> *mut rcf_forest {
    let forest = rcf_forest {
        rcf: None,
        dimensions,
        shingle_size: 1,
        sample_size: 256,
        number_of_trees: 100,
        random_seed: 42,
        parallel: false,
    };
    Box::into_raw(Box::new(forest))
}

#[no_mangle]
pub unsafe extern "C" fn rcf_set_param(
    forest: *mut rcf_forest,
    param: *const c_char,
    value: *const c_char,
) -> c_int {
    if (*forest).rcf.is_some() {
        return -1; // bad forest
    }

    let param = CStr::from_ptr(param).to_str();
    if param.is_err() {
        return -1; // bad param
    }

    let value = CStr::from_ptr(value).to_str();
    if value.is_err() {
        return -1; // bad value
    }

    let param = param.unwrap();
    let value = value.unwrap();

    let success = if param == "shingle_size" {
        value.parse().map(|v| (*forest).shingle_size = v).is_ok()
    } else if param == "sample_size" {
        value.parse().map(|v| (*forest).sample_size = v).is_ok()
    } else if param == "number_of_trees" {
        value.parse().map(|v| (*forest).number_of_trees = v).is_ok()
    } else if param == "random_seed" {
        value.parse().map(|v| (*forest).random_seed = v).is_ok()
    } else if param == "parallel" {
        value.parse().map(|v| (*forest).parallel = v).is_ok()
    } else {
        return -1; // bad param
    };

    if success {
        0
    } else {
        -1 // bad value
    }
}

#[no_mangle]
pub unsafe extern "C" fn rcf_update(forest: *mut rcf_forest, point: *const f32) {
    ensure_forest(forest);
    let slice = slice::from_raw_parts(point, (*forest).dimensions);
    (*forest)
        .rcf
        .as_deref_mut()
        .unwrap()
        .update(slice, 0)
        .unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn rcf_score(forest: *mut rcf_forest, point: *const f32) -> f64 {
    ensure_forest(forest);
    let slice = slice::from_raw_parts(point, (*forest).dimensions);
    (*forest).rcf.as_ref().unwrap().score(slice).unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn rcf_free(forest: *mut rcf_forest) {
    if !forest.is_null() {
        drop(Box::from_raw(forest));
    }
}
