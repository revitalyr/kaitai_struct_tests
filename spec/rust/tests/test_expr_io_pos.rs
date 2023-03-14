// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
#[path = "../formats/mod.rs"] mod formats;
use formats::expr_io_pos::*;

#[test]
fn test_expr_io_pos() {
    let bytes = fs::read("../../src/expr_io_pos.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<ExprIoPos>> = ExprIoPos::read_into(&_io, None, None);
    let r : OptRc<ExprIoPos>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.substream1().my_str(), "CURIOSITY");
    assert_eq!(*r.substream1().body(), vec![0x11u8, 0x22u8, 0x33u8, 0x44u8]);
    assert_eq!(*r.substream1().number(), 66);
    assert_eq!(*r.substream2().my_str(), "KILLED");
    assert_eq!(*r.substream2().body(), vec![0x61u8, 0x20u8, 0x63u8, 0x61u8, 0x74u8]);
    assert_eq!(*r.substream2().number(), 103);
}
