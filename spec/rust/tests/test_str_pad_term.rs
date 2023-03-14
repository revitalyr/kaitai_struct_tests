// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
#[path = "../formats/mod.rs"] mod formats;
use formats::str_pad_term::*;

#[test]
fn test_str_pad_term() {
    let bytes = fs::read("../../src/str_pad_term.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<StrPadTerm>> = StrPadTerm::read_into(&_io, None, None);
    let r : OptRc<StrPadTerm>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.str_pad(), "str1");
    assert_eq!(*r.str_term(), "str2foo");
    assert_eq!(*r.str_term_and_pad(), "str+++3bar+++");
    assert_eq!(*r.str_term_include(), "str4baz@");
}
