// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
#[path = "../formats/mod.rs"] mod formats;
use formats::zlib_surrounded::*;

#[test]
fn test_zlib_surrounded() {
    let bytes = fs::read("../../src/zlib_surrounded.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<ZlibSurrounded>> = ZlibSurrounded::read_into(&_io, None, None);
    let r : OptRc<ZlibSurrounded>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.zlib().num(), -1);
}
