// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
#[path = "../formats/mod.rs"] mod formats;
use formats::zlib_with_header_78::*;

#[test]
fn test_zlib_with_header_78() {
    let bytes = fs::read("../../src/zlib_with_header_78.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<ZlibWithHeader78>> = ZlibWithHeader78::read_into(&_io, None, None);
    let r : OptRc<ZlibWithHeader78>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.data(), vec![0x61u8, 0x20u8, 0x71u8, 0x75u8, 0x69u8, 0x63u8, 0x6bu8, 0x20u8, 0x62u8, 0x72u8, 0x6fu8, 0x77u8, 0x6eu8, 0x20u8, 0x66u8, 0x6fu8, 0x78u8, 0x20u8, 0x6au8, 0x75u8, 0x6du8, 0x70u8, 0x73u8, 0x20u8, 0x6fu8, 0x76u8, 0x65u8, 0x72u8]);
}
