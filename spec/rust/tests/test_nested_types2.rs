// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::nested_types2::*;

#[test]
fn test_nested_types2() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<NestedTypes2>> = NestedTypes2::read_into(&_io, None, None);
    let r : OptRc<NestedTypes2>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.one().typed_at_root().value_b(), 80);
    assert_eq!(*r.one().typed_here1().value_c(), 65);
    assert_eq!(*r.one().typed_here1().typed_here().value_d(), 67);
    assert_eq!(*r.one().typed_here1().typed_parent().value_cc(), 75);
    assert_eq!(*r.one().typed_here1().typed_root().value_b(), 45);
    assert_eq!(*r.one().typed_here2().value_cc(), 49);
    assert_eq!(*r.two().value_b(), -1);
}