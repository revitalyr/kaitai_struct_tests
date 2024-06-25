// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::cast_to_top::*;

#[test]
fn test_cast_to_top() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<CastToTop>> = CastToTop::read_into(&_io, None, None);
    let r : OptRc<CastToTop>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.code(), 80);
    assert_eq!(*r.header().expect("error reading").code(), 65);
    assert_eq!(*r.header_casted().expect("error reading").code(), 65);
}