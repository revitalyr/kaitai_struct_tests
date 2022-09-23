// Autogenerated from KST: please remove this line if doing any edits by hand!

use std::fs;

extern crate kaitai;
use self::kaitai::*;
mod formats;
use formats::switch_manual_enum::*;


#[test]
fn test_switch_manual_enum() {
    let bytes = fs::read("../../src/switch_opcodes.bin").unwrap();
    let reader = BytesReader::new(&bytes);
    let mut r = SwitchManualEnum::default();

    if let Err(err) = r.read(&reader, None, Some(KStructUnit::parent_stack())) {
        panic!("{:?}", err);
    }

    assert_eq!(r.opcodes().len(), 4);
    
    assert_eq!(*r.opcodes()[0 as usize].code(), SwitchManualEnum_Opcode_CodeEnum::Strval);
    if let SwitchManualEnum_Opcode_Body::SwitchManualEnum_Opcode_Strval(s) =  r.opcodes()[0].body() {
        assert_eq!(*s.value(), "foobar");
    } else {
        panic!("expected enum SwitchManualEnum_Opcode_Strval");
    }

    assert_eq!(*r.opcodes()[1 as usize].code(), SwitchManualEnum_Opcode_CodeEnum::Intval);
    if let SwitchManualEnum_Opcode_Body::SwitchManualEnum_Opcode_Intval(s) =  r.opcodes()[1].body() {
        assert_eq!(*s.value(), 66);
    } else {
        panic!("expected enum SwitchManualEnum_Opcode_Intval");
    }

    assert_eq!(*r.opcodes()[2 as usize].code(), SwitchManualEnum_Opcode_CodeEnum::Intval);
    if let SwitchManualEnum_Opcode_Body::SwitchManualEnum_Opcode_Intval(s) =  r.opcodes()[2].body() {
        assert_eq!(*s.value(), 55);
    } else {
        panic!("expected enum SwitchManualEnum_Opcode_Intval");
    }

    assert_eq!(*r.opcodes()[3 as usize].code(), SwitchManualEnum_Opcode_CodeEnum::Strval);
    if let SwitchManualEnum_Opcode_Body::SwitchManualEnum_Opcode_Strval(s) =  r.opcodes()[3].body() {
        assert_eq!(*s.value(), "");
    } else {
        panic!("expected enum SwitchManualEnum_Opcode_Strval");
    }
}
