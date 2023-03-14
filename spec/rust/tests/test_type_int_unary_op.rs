// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
#[path = "../formats/mod.rs"] mod formats;
use formats::type_int_unary_op::*;

#[test]
fn test_type_int_unary_op() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<TypeIntUnaryOp>> = TypeIntUnaryOp::read_into(&_io, None, None);
    let r : OptRc<TypeIntUnaryOp>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.value_s2(), 16720);
    assert_eq!(*r.value_s8(), 4706543082108963651);
    assert_eq!(*r.unary_s2().expect("error reading"), -16720);
    assert_eq!(*r.unary_s8().expect("error reading"), -4706543082108963651);
}
