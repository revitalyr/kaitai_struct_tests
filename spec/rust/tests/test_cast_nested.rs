// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
#[path = "../formats/mod.rs"] mod formats;
use formats::cast_nested::*;

#[test]
fn test_cast_nested() {
    let bytes = fs::read("../../src/switch_opcodes.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<CastNested>> = CastNested::read_into(&_io, None, None);
    let r : OptRc<CastNested>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.opcodes_0_str().expect("error reading").value(), "foobar");
    assert_eq!(*r.opcodes_0_str_value().expect("error reading"), "foobar");
    assert_eq!(*r.opcodes_1_int().expect("error reading").value(), 66);
    assert_eq!(*r.opcodes_1_int_value().expect("error reading"), 66);
}
