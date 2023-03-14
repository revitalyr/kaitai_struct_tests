// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
#[path = "../formats/mod.rs"] mod formats;
use formats::meta_tags::*;

#[test]
fn test_meta_tags() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<MetaTags>> = MetaTags::read_into(&_io, None, None);
    let r : OptRc<MetaTags>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

}
