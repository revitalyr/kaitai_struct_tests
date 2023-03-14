// Autogenerated from KST: please remove this line if doing any edits by hand!

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(overflowing_literals)]
use std::fs;
extern crate kaitai;
use self::kaitai::*;
#[path = "../formats/mod.rs"] mod formats;
use formats::switch_repeat_expr::*;

#[test]
fn test_switch_repeat_expr() {
    let bytes = fs::read("../../src/switch_tlv.bin").unwrap();
    let _io = BytesReader::from(bytes);
    let res: KResult<OptRc<SwitchRepeatExpr>> = SwitchRepeatExpr::read_into(&_io, None, None);
    let r : OptRc<SwitchRepeatExpr>;

    if let Err(err) = res {
        panic!("{:?}", err);
    } else {
        r = res.unwrap();
    }

    assert_eq!(*r.code(), 17);
    assert_eq!(*r.size(), 9);
    assert_eq!(*Into::<OptRc<SwitchRepeatExpr_One>>::into(&r.body()[0 as usize]).first(), vec![0x53u8, 0x74u8, 0x75u8, 0x66u8, 0x66u8, 0x0u8, 0x4du8, 0x65u8, 0x0u8]);
}
