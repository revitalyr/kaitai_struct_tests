// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
#[path = "../formats/mod.rs"] mod formats;
use formats::valid_fail_range_int::*;

#[test]
fn test_valid_fail_range_int() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<ValidFailRangeInt>> = ValidFailRangeInt::read_into(&_io, None, None);
    let r : OptRc<ValidFailRangeInt>;

    if let Err(err) = res {
        println!("expected err: {:?}, exception: ValidationGreaterThanError(Int1Type(false))", err);
    } else {
        panic!("no expected exception: ValidationGreaterThanError(Int1Type(false))");
    }
}
