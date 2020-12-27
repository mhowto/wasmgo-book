use super::module::*;
use super::types::*;
use super::instructions::*;

use super::errors::{Error, Result};
use std::convert::TryInto;
use std::fs;
use std::path::PathBuf;
extern crate num;

pub fn decode_file(filename: PathBuf) -> Result<Module> {
    match fs::read(filename) {
        Ok(value) => decode(value),
        Err(e) => Err(Error::Io(e)),
    }
}

fn decode(_data: Vec<u8>) -> Result<Module> {
    let mut reader = WasmReader::new(_data);
    Ok(reader.read_module())
}

// leb128
fn decode_var_uint(data_: &Vec<u8>, size: usize) -> (u64, usize) {
    let mut result: u64 = 0;
    for (i, b) in data_.iter().cloned().enumerate() {
        if i == size / 7 {
            if (b & 0x80) != 0 {
                panic!("int too long")
            }
            if (b >> (size - i * 7)) > 0 {
                panic!("int too long")
            }
        }
        result |= (b as u64 & 0x7f) << (i * 7);
        if (b & 0x80) == 0 {
            return (result, i + 1);
        }
    }
    panic!("unexpected end of LEB128")
}

fn decode_var_int(data_: &Vec<u8>, size: usize) -> (i64, usize) {
    let mut result: i64 = 0;
    for (i, b) in data_.iter().cloned().enumerate() {
        if i == size / 7 {
            if b & 0x80 != 0 {
                panic!("int too long")
            }
            if b & 0x40 == 0 && b >> (size - i * 7 - 1) != 0
                || b & 0x40 != 0 && (b | 0x80) as i8 >> (size - i * 7 - 1) != -1
            {
                panic!("int too long")
            }
        }
        result |= (b as i64 & 0x7f) << (i * 7);
        if (b & 0x80) == 0 {
            if (i * 7 < size) && (b & 0x40) != 0 {
                result = result | (-1 << ((i + 1) * 7))
            }
            return (result, i + 1);
        }
    }
    panic!("unexpected end of LEB128")
}

struct WasmReader {
    data: Vec<u8>,
}

impl WasmReader {
    fn new(data_: Vec<u8>) -> Self {
        WasmReader{
            data: data_
        }
    }
    fn read_byte(&mut self) -> u8 {
        let b = self.data.remove(0);
        b
    }

    fn read_u32(&mut self) -> u32 {
        let u32_bytes = self.data.drain(0..4);
        u32::from_le_bytes(u32_bytes.as_slice().try_into().unwrap())
    }

    fn read_f32(&mut self) -> f32 {
        let f32_bytes = self.data.drain(0..4);
        f32::from_le_bytes(f32_bytes.as_slice().try_into().unwrap())
    }

    fn read_f64(&mut self) -> f64 {
        let f64_bytes = self.data.drain(0..8);
        f64::from_le_bytes(f64_bytes.as_slice().try_into().unwrap())
    }

    fn read_var_u32(&mut self) -> u32 {
        let (number, n) = decode_var_uint(&self.data, 32);
        self.data.drain(0..n);
        number as u32
    }

    fn read_var_i32(&mut self) -> i32 {
        let (number, n) = decode_var_int(&self.data, 32);
        self.data.drain(0..n);
        number as i32
    }

    fn read_var_i64(&mut self) -> i64 {
        let (number, n) = decode_var_int(&self.data, 64);
        self.data.drain(0..n);
        number as i64
    }

    fn read_bytes(&mut self) -> Vec<u8> {
        let n = self.read_var_u32();
        self.data.drain(0..n as usize).as_ref().try_into().unwrap()
    }

    fn read_name(&mut self) -> String {
        String::from_utf8(self.read_bytes()).unwrap()
    }

    fn remaining(&self) -> usize {
        self.data.len()
    }

    fn read_val_type(&mut self) -> ValType {
        num::FromPrimitive::from_u8(self.read_byte()).unwrap()
    }

    fn read_val_types(&mut self) -> Vec<ValType> {
        let count = self.read_var_u32() as usize;

        let mut types = Vec::with_capacity(count);
        for i in 0..count {
            types.push(self.read_val_type());
        }
        types
    }

    fn read_func_type(&mut self) -> FuncType {
        FuncType::new(
            self.read_byte(),
            self.read_val_types(),
            self.read_val_types(),
        )
    }

    fn read_type_sec(&mut self) -> Vec<FuncType> {
        let count = self.read_var_u32() as usize;
        let mut types = Vec::with_capacity(count);
        for i in 0..count {
            types.push(self.read_func_type());
        }
        types
    }

    fn read_mut_type(&mut self) -> MutType {
        num::FromPrimitive::from_u8(self.read_byte()).unwrap()
    }

    fn read_global_type(&mut self) -> GlobalType {
        GlobalType::new(self.read_val_type(), self.read_mut_type())
    }

    fn read_limits(&mut self) -> Limits {
        let tag = self.read_byte();
        match tag {
            0 => Limits::LimitsMin(self.read_var_u32()),
            1 => Limits::LimitsMinMax(self.read_var_u32(), self.read_var_u32()),
            _ => panic!("invalid tag"),
        }
    }

    fn read_table_type(&mut self) -> TableType {
        let elem_type: ElemType = num::FromPrimitive::from_u8(self.read_byte()).unwrap();
        TableType::new(elem_type, self.read_limits())
    }

    fn read_import_desc(&mut self) -> ImportDesc {
        let tag: ImportTag = num::FromPrimitive::from_u8(self.read_byte()).unwrap();
        match tag {
            ImportTag::Func => ImportDesc::ImportDescFuncType(self.read_var_u32()),
            ImportTag::Global => ImportDesc::ImportDescGlobal(self.read_global_type()),
            ImportTag::Mem => ImportDesc::ImportDescMem(self.read_limits()),
            ImportTag::Table => ImportDesc::ImportDescTable(self.read_table_type()),
        }
    }


    pub fn read_module(&mut self) -> Module {
        let mut module = Module::default();
        module.magic = self.read_u32();
        module.version = self.read_u32();
        self.read_sections(&mut module);
        module
    }

    fn read_sections(&mut self, module: &mut Module) {
        let mut prev_sec_id = SecID::SecCustomID;
        while self.remaining() > 0 {
            let sec_id: SecID = num::FromPrimitive::from_u8(self.read_byte()).unwrap();

            if sec_id == SecID::SecCustomID {
                module.custom_secs.push(self.read_custom_sec());
                continue;
            }

            if sec_id > SecID::SecDataID || sec_id <= prev_sec_id {
                panic!("malformed section id: {:?}, prev_sec_id: {:?}", sec_id, prev_sec_id);
            }

            let n = self.read_var_u32();
            let remaining_before_read = self.remaining();
            self.read_non_custom_sec(&sec_id, module);
            if self.remaining() + n as usize != remaining_before_read {
                panic!("section size mismatch, id: {:?}", sec_id);
            }
            prev_sec_id = sec_id;
            // println!("remaining {} after read {:?}", self.remaining(), prev_sec_id);
        }
    }

    fn read_custom_sec(&mut self) -> CustomSec {
        let mut sec_wasm_reader = WasmReader {
            data: self.read_bytes(),
        };
        CustomSec::new(sec_wasm_reader.read_name(), sec_wasm_reader.data)
    }

    fn read_import_sec(&mut self) -> Vec<Import> {
        let cap = self.read_var_u32() as usize;
        let mut result: Vec<Import> = Vec::with_capacity(cap);
        for i in 0..cap {
            result.push(self.read_import());
        }
        result
    }
    fn read_import(&mut self) -> Import {
        Import {
            module: self.read_name(),
            name: self.read_name(),
            desc: self.read_import_desc(),
        }
    }

    fn read_indices(&mut self) -> Vec<TypeIdx>  {
        let cap = self.read_var_u32() as usize;
        let mut result: Vec<TypeIdx> = Vec::with_capacity(cap);
        for i in 0..cap {
            result.push(self.read_var_u32());
        }
        result
    }
    fn read_table_sec(&mut self) -> Vec<TableType> {
        let cap = self.read_var_u32() as usize;
        let mut result: Vec<TableType> = Vec::with_capacity(cap);
        for i in 0..cap {
            result.push(self.read_table_type());
        }
        result
    }
    fn read_mem_sec(&mut self) -> Vec<MemType> {
        let cap = self.read_var_u32() as usize;
        let mut result: Vec<MemType> = Vec::with_capacity(cap);
        for i in 0..cap {
            result.push(self.read_limits());
        }
        result
    }
    fn read_global_sec(&mut self) -> Vec<Global> {
        let cap = self.read_var_u32() as usize;
        let mut result: Vec<Global> = Vec::with_capacity(cap);
        for i in 0..cap {
            result.push(Global{
                type_: self.read_global_type(),
                init: self.read_expr(),
            });
        }
        result
    }
    fn read_expr(&mut self) -> Expr {
        while self.read_byte() != 0x0B {
        }
        Expr{}
    }

    fn read_export_sec(&mut self) -> Vec<Export> {
        let cap = self.read_var_u32() as usize;
        let mut result: Vec<Export> = Vec::with_capacity(cap);
        for i in 0..cap {
            result.push(self.read_export());
        }
        result
    }
    fn read_export(&mut self) -> Export {
        Export{
            name: self.read_name(),
            desc: self.read_export_desc(),
        }
    }
    fn read_export_desc(&mut self) -> ExportDesc {
        ExportDesc{
            tag: num::FromPrimitive::from_u8(self.read_byte()).unwrap(),
            idx: self.read_var_u32(),
        }
    }
    fn read_start_sec(&mut self) -> Option<u32> {
        Some(self.read_var_u32())
    }
    fn read_elem_sec(&mut self) -> Vec<Elem> {
        let cap = self.read_var_u32() as usize;
        let mut  result: Vec<Elem> = Vec::with_capacity(cap);
        for i in 0..cap {
            result.push(self.read_elem());
        }
        result
    }
    fn read_elem(&mut self) -> Elem {
        Elem {
            table: self.read_var_u32(),
            offset: self.read_expr(),
            init: self.read_indices(),
        }
    }
    fn read_code_sec(&mut self) -> Vec<Code> {
        let cap = self.read_var_u32() as usize;
        let mut result: Vec<Code> = Vec::with_capacity(cap);
        for i in 0..cap {
            result.push(self.read_code());
        }
        result
    }
    fn read_code(&mut self) -> Code {
        let mut code_reader = WasmReader::new(self.read_bytes());
        Code::new(code_reader.read_locals_vec(), Expr::default())
    }
    fn read_locals_vec(&mut self) -> Vec<Locals> {
        let cap = self.read_var_u32() as usize;
        let mut result: Vec<Locals> = Vec::with_capacity(cap);
        for i in 0..cap {
            result.push(self.read_locals());
        }
        result
    }
    fn read_locals(&mut self) -> Locals {
        Locals{
            n: self.read_var_u32(),
            type_: self.read_val_type(),
        }
    }
    fn read_data_sec(&mut self) -> Vec<Data> {
        let cap = self.read_var_u32() as usize;
        let mut  result: Vec<Data> = Vec::with_capacity(cap);
        for i in 0..cap {
            result.push(self.read_data());
        }
        result
    }

    fn read_data(&mut self) -> Data {
        Data {
            mem: self.read_var_u32(),
            offset: self.read_expr(),
            init: self.read_bytes(),
        }
    }

    fn read_non_custom_sec(&mut self, sec_id: &SecID, module: &mut Module) {
        match sec_id {
            SecID::SecCustomID => panic!("custom sec should not be read here"), // 0
            SecID::SecTypeID => module.type_secs = self.read_type_sec(),
            SecID::SecImportID => module.import_secs = self.read_import_sec(), // 2
            SecID::SecFuncID => module.func_secs = self.read_indices(),   // 3
            SecID::SecTableID => module.table_secs = self.read_table_sec(),  // 4
            SecID::SecMemID => module.mem_secs = self.read_mem_sec(),    // 5
            SecID::SecGlobalID => module.global_secs = self.read_global_sec(), // 6
            SecID::SecExportID => module.export_secs = self.read_export_sec(), // 7
            SecID::SecStartID => module.start_sec = self.read_start_sec(),  // 8
            SecID::SecElemID => module.elem_secs = self.read_elem_sec(),   // 9
            SecID::SecCodeID => module.code_secs = self.read_code_sec(),   // 10
            SecID::SecDataID => module.data_secs = self.read_data_sec(),   // 11
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_decode_var_uint32(data_: &Vec<u8>, want_n: u32, want_w: usize) {
        let (n, w) = decode_var_uint(data_, 32);
        assert_eq!(want_n, n as u32);
        assert_eq!(want_w, w);
    }

    #[test]
    fn test_decode_var_uint() {
        let data: Vec<u8> = vec![
            0b1_0111111,
            0b1_0011111,
            0b1_0001111,
            0b1_0000111,
            0b1_0000011,
            0b0_0000001,
        ];
        test_decode_var_uint32(&data[5..].to_vec(), 0b00000001, 1);
        test_decode_var_uint32(&data[4..].to_vec(), 0b1_0000011, 2);
        test_decode_var_uint32(&data[3..].to_vec(), 0b1_0000011_0000111, 3);
        test_decode_var_uint32(&data[2..].to_vec(), 0b1_0000011_0000111_0001111, 4);
        test_decode_var_uint32(&data[1..].to_vec(), 0b1_0000011_0000111_0001111_0011111, 5);
    }

    #[test]
    #[should_panic(expected = "int too long")]
    fn test_decode_too_long_var_uint() {
        let data: Vec<u8> = vec![
            0b1_0111111,
            0b1_0011111,
            0b1_0001111,
            0b1_0000111,
            0b1_0000011,
            0b0_0000001,
        ];
        decode_var_uint(&data, 32);
    }

    #[test]
    fn test_decode_var_int() {
        let data: Vec<u8> = vec![0xC0, 0xBB, 0x78];
        let (n, w) = decode_var_int(&data, 32);
        assert_eq!(n as i32, -123456 as i32);
        assert_eq!(w, 3);
    }

    #[test]
    fn test_reads() {
        #[rustfmt::skip]
        let mut reader = WasmReader {
            data: vec![
                0x01,
        		0x02, 0x03, 0x04, 0x05,
		        0x00, 0x00, 0xc0, 0x3f,
		        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf8, 0x3f,
		        0xE5, 0x8E, 0x26, // https://en.wikipedia.org/wiki/LEB128#Unsigned_LEB128
		        0xC0, 0xBB, 0x78, // https://en.wikipedia.org/wiki/LEB128#Signed_LEB128
		        0xC0, 0xBB, 0x78,
		        0x03, 0x01, 0x02, 0x03,
		        0x03, 0x66, 0x6f, 0x6f,
            ],
        };
        assert_eq!(0x01 as u8, reader.read_byte());
        assert_eq!(0x05040302 as u32, reader.read_u32());
        assert_eq!(1.5 as f32, reader.read_f32());
        assert_eq!(1.5 as f64, reader.read_f64());
        assert_eq!(624485 as u32, reader.read_var_u32());
        assert_eq!(-123456 as i32, reader.read_var_i32());
        assert_eq!(-123456 as i64, reader.read_var_i64());
        assert_eq!(vec!(0x01, 0x02, 0x03), reader.read_bytes());
        assert_eq!("foo", reader.read_name());
        assert_eq!(0, reader.remaining());
    }
}
