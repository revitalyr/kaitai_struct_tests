// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
#[path = "../formats/mod.rs"] mod formats;
use formats::position_in_seq::*;

#[test]
fn test_position_in_seq() {
    let bytes = fs::read("../../src/position_in_seq.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<PositionInSeq>> = PositionInSeq::read_into(&_io, None, None);
    let r : OptRc<PositionInSeq>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.numbers(), vec![(0 + 1), 2, 3]);
}
