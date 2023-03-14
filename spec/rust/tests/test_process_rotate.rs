// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
#[path = "../formats/mod.rs"] mod formats;
use formats::process_rotate::*;

#[test]
fn test_process_rotate() {
    let bytes = fs::read("../../src/process_rotate.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<ProcessRotate>> = ProcessRotate::read_into(&_io, None, None);
    let r : OptRc<ProcessRotate>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.buf1(), vec![0x48u8, 0x65u8, 0x6cu8, 0x6cu8, 0x6fu8]);
    assert_eq!(*r.buf2(), vec![0x57u8, 0x6fu8, 0x72u8, 0x6cu8, 0x64u8]);
    assert_eq!(*r.buf3(), vec![0x54u8, 0x68u8, 0x65u8, 0x72u8, 0x65u8]);
}
