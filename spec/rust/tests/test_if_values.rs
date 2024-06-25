// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
use rust::formats::if_values::*;

#[test]
fn test_if_values() {
    let bytes = fs::read("../../src/fixed_struct.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<IfValues>> = IfValues::read_into(&_io, None, None);
    let r : OptRc<IfValues>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.codes()[0 as usize].opcode(), 80);
    assert_eq!(*r.codes()[0 as usize].half_opcode().expect("error reading"), 40);
    assert_eq!(*r.codes()[1 as usize].opcode(), 65);
    assert_eq!(*r.codes()[1 as usize].half_opcode().expect("error reading"), 0);
    assert_eq!(*r.codes()[2 as usize].opcode(), 67);
    assert_eq!(*r.codes()[2 as usize].half_opcode().expect("error reading"), 0);
}