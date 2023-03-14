// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
#[path = "../formats/mod.rs"] mod formats;
use formats::instance_std::*;

#[test]
fn test_instance_std() {
    let bytes = fs::read("../../src/str_encodings.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<InstanceStd>> = InstanceStd::read_into(&_io, None, None);
    let r : OptRc<InstanceStd>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.header().expect("error reading"), "Some ");
}
