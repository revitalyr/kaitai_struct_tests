// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::params_call::*;

#[test]
fn test_params_call() {
    let bytes = fs::read("../../src/term_strz.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<ParamsCall>> = ParamsCall::read_into(&_io, None, None);
    let r : OptRc<ParamsCall>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.buf1().body(), "foo|b");
    assert_eq!(*r.buf2().body(), "ar|ba");
    assert_eq!(*r.buf2().trailer(), 122);
}