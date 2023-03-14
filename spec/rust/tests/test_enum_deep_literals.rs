// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
#[path = "../formats/mod.rs"] mod formats;
use formats::enum_deep_literals::*;

#[test]
fn test_enum_deep_literals() {
    let bytes = fs::read("../../src/enum_0.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<EnumDeepLiterals>> = EnumDeepLiterals::read_into(&_io, None, None);
    let r : OptRc<EnumDeepLiterals>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.is_pet_1_ok().expect("error reading"), true);
    assert_eq!(*r.is_pet_2_ok().expect("error reading"), true);
}
