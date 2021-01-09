use std::fmt;

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
impl fmt::Display for ValType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_string = match self {
            ValType::I32 => "i32",
            ValType::I64 => "i64",
            ValType::F32 => "f32",
            ValType::F64 => "f64",
        };
        write!(f, "{}", type_string)
    }
}

#[derive(Debug)]
pub struct FuncType {
    tag: u8,
    param_types: Vec<ValType>,
    result_types: Vec<ValType>,
}

impl fmt::Display for FuncType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let params: Vec<String> = self.param_types.iter().map(|x| x.to_string()).collect();
        let results: Vec<String> = self.result_types.iter().map(|x| x.to_string()).collect();
        write!(f, "({}) -> ({})", params.join(","), results.join(","))
    }
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

impl fmt::Display for Limits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Limits::LimitsMin(min) => write!(f, "{{min: {}}}", min),
            Limits::LimitsMinMax(min, max) => write!(f, "{{min: {}, max: {}}}", min, max),
        }
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
    pub elem_type: ElemType, //目前只能是0x70
    pub limits: Limits,
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

impl fmt::Display for MutType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MutType::Const => write!(f, "const"),
            MutType::Var => write!(f, "var"),
        }
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

impl fmt::Display for GlobalType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.mut_, self.val_type)
    }
}