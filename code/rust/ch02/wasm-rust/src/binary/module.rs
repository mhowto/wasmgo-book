use super::instructions::*;
use super::types::*;
use std::fmt;

#[derive(Debug, FromPrimitive, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum SecID {
    SecCustomID, // 0
    SecTypeID,   // 1
    SecImportID, // 2
    SecFuncID,   // 3
    SecTableID,  // 4
    SecMemID,    // 5
    SecGlobalID, // 6
    SecExportID, // 7
    SecStartID,  // 8
    SecElemID,   // 9
    SecCodeID,   // 10
    SecDataID,   // 11
}

impl fmt::Display for SecID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}



const MAGIC_NUMBER: u32 = 0x6D736100; // `/asm`
const VERSION: u32 = 0x00000001; // 1

#[derive(Default, Debug)]
pub struct Module {
    pub magic: u32,
    pub version: u32,
    pub custom_secs: Vec<CustomSec>,
    pub type_secs: Vec<FuncType>,
    pub import_secs: Vec<Import>,
    pub func_secs: Vec<TypeIdx>,
    pub table_secs: Vec<TableType>,
    pub mem_secs: Vec<MemType>,
    pub global_secs: Vec<Global>,
    pub export_secs: Vec<Export>,
    pub start_sec: Option<FuncIdx>,
    pub elem_secs: Vec<Elem>,
    pub code_secs: Vec<Code>,
    pub data_secs: Vec<Data>,
}

/// 字符串作为自定义段的名称起到标识作用
#[derive(Debug)]
pub struct CustomSec {
    name: String,
    bytes: Vec<u8>,
}

impl CustomSec {
    pub fn new(name_: String, bytes_: Vec<u8>) -> CustomSec {
        CustomSec {
            name: name_,
            bytes: bytes_,
        }
    }
}

#[derive(Debug)]
pub struct Import {
    pub module: String,
    pub name: String,
    pub desc: ImportDesc,
}

#[derive(Debug, FromPrimitive)]
#[repr(u8)]
pub enum ImportTag {
    Func,   // 0
    Table,  // 1
    Mem,    // 2
    Global, // 3
}

impl Default for ImportTag {
    fn default() -> Self {
        ImportTag::Func
    }
}

#[derive(Debug)]
pub enum ImportDesc {
    ImportDescFuncType(TypeIdx),
    ImportDescTable(TableType),
    ImportDescMem(MemType),
    ImportDescGlobal(GlobalType),
}

impl Default for ImportDesc {
    fn default() -> Self {
        ImportDesc::ImportDescFuncType(TypeIdx::default())
    }
}

/// 全局段列出模块内定义的所有全局变量。每一项需要指定全局变量的
/// 类型以及初始值
/// global_sec : 0x06|byte_count|vec<global>
/// global     : global_type|init_expr
/// global_type: val_type|mut
/// expr       : byte*|0x0B
#[derive(Debug, Default)]
pub struct Global {
    pub type_: GlobalType,
    pub init: Expr,
}

#[derive(Debug, Default)]
pub struct Export {
    pub name: String,
    pub desc: ExportDesc,
}

#[derive(Debug)]
pub enum ExportTag {
    Func,   //0
    Table,  //1
    Mem,    //2
    Global, //3
}
impl Default for ExportTag {
    fn default() -> Self {
        ExportTag::Func
    }
}

#[derive(Debug, Default)]
pub struct ExportDesc {
    tag: ExportTag,
    idx: u32,
}

#[derive(Debug, Default)]
pub struct Elem {
    pub table: TableIdx,
    pub offset: Expr,
    pub init: Vec<FuncIdx>,
}

#[derive(Debug, Default)]
pub struct Code {
    locals: Locals,
    expr: Expr, // 字节码
}

impl Code {
    pub fn new(l: Locals, e: Expr) -> Self {
        Self { locals: l, expr: e }
    }
}

#[derive(Debug, Default)]
pub struct Locals {
    n: u32,
    type_: ValType,
}

#[derive(Debug, Default)]
pub struct Data {
    pub mem: MemIdx,
    pub offset: Expr,
    pub init: Vec<u8>,
}
