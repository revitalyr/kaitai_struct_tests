// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
#[path = "../formats/mod.rs"] mod formats;
use formats::params_pass_array_int::*;

#[test]
fn test_params_pass_array_int() {
    let bytes = fs::read("../../src/position_to_end.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<ParamsPassArrayInt>> = ParamsPassArrayInt::read_into(&_io, None, None);
    let r : OptRc<ParamsPassArrayInt>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(r.pass_ints().nums().len(), 3);
    assert_eq!(r.pass_ints().nums()[0 as usize], 513);
    assert_eq!(r.pass_ints().nums()[1 as usize], 1027);
    assert_eq!(r.pass_ints().nums()[2 as usize], 1541);
    assert_eq!(r.pass_ints_calc().nums().len(), 2);
    assert_eq!(r.pass_ints_calc().nums()[0 as usize], 27643);
    assert_eq!(r.pass_ints_calc().nums()[1 as usize], 7);
}
