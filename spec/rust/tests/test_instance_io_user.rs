// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
#[path = "../formats/mod.rs"] mod formats;
use formats::instance_io_user::*;

#[test]
fn test_instance_io_user() {
    let bytes = fs::read("../../src/instance_io.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<InstanceIoUser>> = InstanceIoUser::read_into(&_io, None, None);
    let r : OptRc<InstanceIoUser>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.qty_entries(), 3);
    assert_eq!(*r.entries()[0 as usize].name().expect("error reading"), "the");
    assert_eq!(*r.entries()[1 as usize].name().expect("error reading"), "rainy");
    assert_eq!(*r.entries()[2 as usize].name().expect("error reading"), "day it is");
}
