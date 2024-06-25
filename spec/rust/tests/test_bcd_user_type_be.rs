// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::bcd_user_type_be::*;

#[test]
fn test_bcd_user_type_be() {
    let bytes = fs::read("../../src/bcd_user_type_be.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<BcdUserTypeBe>> = BcdUserTypeBe::read_into(&_io, None, None);
    let r : OptRc<BcdUserTypeBe>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.ltr().as_int().expect("error reading"), 12345678);
    assert_eq!(*r.ltr().as_str().expect("error reading"), "12345678");
    assert_eq!(*r.rtl().as_int().expect("error reading"), 87654321);
    assert_eq!(*r.rtl().as_str().expect("error reading"), "87654321");
    assert_eq!(*r.leading_zero_ltr().as_int().expect("error reading"), 123456);
    assert_eq!(*r.leading_zero_ltr().as_str().expect("error reading"), "00123456");
}