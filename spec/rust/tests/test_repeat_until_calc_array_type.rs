// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
#[path = "../formats/mod.rs"] mod formats;
use formats::repeat_until_calc_array_type::*;

#[test]
fn test_repeat_until_calc_array_type() {
    let bytes = fs::read("../../src/repeat_until_process.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<RepeatUntilCalcArrayType>> = RepeatUntilCalcArrayType::read_into(&_io, None, None);
    let r : OptRc<RepeatUntilCalcArrayType>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(r.records().len(), 3);
    assert_eq!(*r.records()[0 as usize].marker(), 232);
    assert_eq!(*r.records()[0 as usize].body(), 2863311546);
    assert_eq!(*r.records()[1 as usize].marker(), 250);
    assert_eq!(*r.records()[1 as usize].body(), 2863315102);
    assert_eq!(*r.records()[2 as usize].marker(), 170);
    assert_eq!(*r.records()[2 as usize].body(), 1431655765);
}
