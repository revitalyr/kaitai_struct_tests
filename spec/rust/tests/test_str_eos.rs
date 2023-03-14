// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
#[path = "../formats/mod.rs"] mod formats;
use formats::str_eos::*;

#[test]
fn test_str_eos() {
    let bytes = fs::read("../../src/term_strz.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<StrEos>> = StrEos::read_into(&_io, None, None);
    let r : OptRc<StrEos>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.str(), "foo|bar|baz@");
}
