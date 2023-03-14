// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
#[path = "../formats/mod.rs"] mod formats;
use formats::floating_points::*;

#[test]
fn test_floating_points() {
    let bytes = fs::read("../../src/floating_points.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<FloatingPoints>> = FloatingPoints::read_into(&_io, None, None);
    let r : OptRc<FloatingPoints>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.single_value(), (0.5 as f32));
    assert_eq!(*r.single_value_be(), (0.5 as f32));
    assert_eq!(*r.double_value(), 0.25);
    assert_eq!(*r.double_value_be(), 0.25);
    assert_eq!(*r.approximate_value(), 1.2345);
    assert_eq!(*r.single_value_plus_int().expect("error reading"), 1.5);
    assert_eq!(*r.single_value_plus_float().expect("error reading"), 1.0);
    assert_eq!(*r.double_value_plus_float().expect("error reading"), 0.3);
}
