pub type TypeIdx = u32;
pub type FuncIdx = u32;
pub type TableIdx = u32;
pub type MemIdx = u32;
pub type GlobalIdx = u32; // 全局变量
pub type LocalIdx = u32; // 局部变量
pub type LabelIdx = u32; // 跳转标签

#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum ValType {
    I32 = 0x7F,
    I64 = 0x7E,
    F32 = 0x7D,
    F64 = 0x7C,
}

impl Default for ValType {
    fn default() -> Self {
        ValType::I32
    }
}

#[derive(Debug)]
pub struct FuncType {
    tag: u8,
    param_types: Vec<ValType>,
    result_types: Vec<ValType>,
}

impl FuncType {
    pub fn new(tag_: u8, param_types_: Vec<ValType>, result_types_: Vec<ValType>) -> Self {
        Self {
            tag: tag_,
            param_types: param_types_,
            result_types: result_types_,
        }
    }
}

/// 用于指定元素数量的上限
/// limits: tag|min|max?
/// tag是0，表示只指定下限。tag为1，同时指定下限，上限。
#[derive(Debug)]
pub enum Limits {
    LimitsMin(u32),
    LimitsMinMax(u32, u32),
}

impl Default for Limits {
    fn default() -> Self {
        Limits::LimitsMin(0)
    }
}

pub type MemType = Limits;

#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum ElemType {
    FuncRef = 0x70,
}

impl Default for ElemType {
    fn default() -> Self {
        ElemType::FuncRef
    }
}

/// 表段列出模块定义的表。最多只能定义一张表，
/// 且元素类型必须为函数引用。
/// table_type: 0x70|limits
/// limits    : tag|min|max?
#[derive(Debug, Default)]
pub struct TableType {
    elem_type: ElemType, //目前只能是0x70
    limits: Limits,
}

impl TableType {
    pub fn new(elem_type_: ElemType, limits_: Limits) -> Self {
        Self {
            elem_type: elem_type_,
            limits: limits_,
        }
    }
}

#[repr(u8)]
#[derive(Debug, FromPrimitive)]
pub enum MutType {
    Const, // 0
    Var,   // 1
}

impl Default for MutType {
    fn default() -> Self {
        MutType::Const
    }
}

#[derive(Debug, Default)]
pub struct GlobalType {
    val_type: ValType,
    mut_: MutType,
}

impl GlobalType {
    pub fn new(val_type_: ValType, mut__: MutType) -> Self {
        Self {
            val_type: val_type_,
            mut_: mut__,
        }
    }
}