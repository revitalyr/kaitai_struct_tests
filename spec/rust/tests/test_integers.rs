// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
#[path = "../formats/mod.rs"] mod formats;
use formats::integers::*;

#[test]
fn test_integers() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<Integers>> = Integers::read_into(&_io, None, None);
    let r : OptRc<Integers>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.uint8(), 255);
    assert_eq!(*r.uint16(), 65535);
    assert_eq!(*r.uint32(), 4294967295);
    assert_eq!(*r.uint64(), 18446744073709551615);
    assert_eq!(*r.sint8(), -1);
    assert_eq!(*r.sint16(), -1);
    assert_eq!(*r.sint32(), -1);
    assert_eq!(*r.sint64(), -1);
    assert_eq!(*r.uint16le(), 66);
    assert_eq!(*r.uint32le(), 66);
    assert_eq!(*r.uint64le(), 66);
    assert_eq!(*r.sint16le(), -66);
    assert_eq!(*r.sint32le(), -66);
    assert_eq!(*r.sint64le(), -66);
    assert_eq!(*r.uint16be(), 66);
    assert_eq!(*r.uint32be(), 66);
    assert_eq!(*r.uint64be(), 66);
    assert_eq!(*r.sint16be(), -66);
    assert_eq!(*r.sint32be(), -66);
    assert_eq!(*r.sint64be(), -66);
}
