// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use kaitai::*;
use std::{fs, path::PathBuf, convert::{TryFrom, TryInto}};

#[derive(Default, Debug, PartialEq)]
pub struct BitsUnalignedB64Be {
    pub a: bool,
    pub b: u64,
    pub c: u64,
}
impl<'r, 's: 'r> KStruct<'r, 's> for BitsUnalignedB64Be {
    type Root = Self;
    type ParentStack = KStructUnit;

    fn read<S: KStream>(
        &mut self,
        _io: &'s S,
        _root: Option<&'r Self::Root>,
        _parent: Option<TypedStack<Self::ParentStack>>
    ) -> KResult<()> {
        self.a = _io.read_bits_int(1)? != 0;
        self.b = _io.read_bits_int(64)?;
        self.c = _io.read_bits_int(7)?;
        Ok(())
    }
}
impl<'r, 's: 'r> BitsUnalignedB64Be {
    pub fn from_file(path: &str) -> Self {
        let bytes = fs::read(path).unwrap();
        let reader = BytesReader::new(&bytes);
        let mut obj = BitsUnalignedB64Be::default();

        if let Err(err) = obj.read(&reader, None, None) {
            panic!("error '{:?}' reading from file '{}'", err, path);
        }

        obj
    }

}
