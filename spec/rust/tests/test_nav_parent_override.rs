// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
#[path = "../formats/mod.rs"] mod formats;
use formats::nav_parent_override::*;

#[test]
fn test_nav_parent_override() {
    let bytes = fs::read("../../src/nav_parent_codes.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<NavParentOverride>> = NavParentOverride::read_into(&_io, None, None);
    let r : OptRc<NavParentOverride>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.child_size(), 3);
    assert_eq!(*r.child_1().data(), vec![0x49u8, 0x31u8, 0x32u8]);
    assert_eq!(*r.mediator_2().child_2().data(), vec![0x33u8, 0x42u8, 0x62u8]);
}
