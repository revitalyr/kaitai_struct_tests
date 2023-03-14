// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
#[path = "../formats/mod.rs"] mod formats;
use formats::position_abs::*;

#[test]
fn test_position_abs() {
    let bytes = fs::read("../../src/position_abs.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<PositionAbs>> = PositionAbs::read_into(&_io, None, None);
    let r : OptRc<PositionAbs>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.index_offset(), 32);
    assert_eq!(*r.index().expect("error reading").entry(), "foo");
}
