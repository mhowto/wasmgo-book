pub enum Opcode {
    Unreachable       = 0x00, // unreachable
	Nop               = 0x01, // nop
	Block             = 0x02, // block rt in* end
	Loop              = 0x03, // loop rt in* end
	If                = 0x04, // if rt in* else in* end
	Else_             = 0x05, // else
	End_              = 0x0B, // end
	Br                = 0x0C, // br l
	BrIf              = 0x0D, // br_if l
	BrTable           = 0x0E, // br_table l* lN
	Return            = 0x0F, // return
	Call              = 0x10, // call x
	CallIndirect      = 0x11, // call_indirect x
	Drop              = 0x1A, // drop
	Select            = 0x1B, // select
	LocalGet          = 0x20, // local.get x
	LocalSet          = 0x21, // local.set x
	LocalTee          = 0x22, // local.tee x
	GlobalGet         = 0x23, // global.get x
	GlobalSet         = 0x24, // global.set x
	I32Load           = 0x28, // i32.load m
	I64Load           = 0x29, // i64.load m
	F32Load           = 0x2A, // f32.load m
	F64Load           = 0x2B, // f64.load m
	I32Load8S         = 0x2C, // i32.load8_s m
	I32Load8U         = 0x2D, // i32.load8_u m
	I32Load16S        = 0x2E, // i32.load16_s m
	I32Load16U        = 0x2F, // i32.load16_u m
	I64Load8S         = 0x30, // i64.load8_s m
	I64Load8U         = 0x31, // i64.load8_u m
	I64Load16S        = 0x32, // i64.load16_s m
	I64Load16U        = 0x33, // i64.load16_u m
	I64Load32S        = 0x34, // i64.load32_s m
	I64Load32U        = 0x35, // i64.load32_u m
	I32Store          = 0x36, // i32.store m
	I64Store          = 0x37, // i64.store m
	F32Store          = 0x38, // f32.store m
	F64Store          = 0x39, // f64.store m
	I32Store8         = 0x3A, // i32.store8 m
	I32Store16        = 0x3B, // i32.store16 m
	I64Store8         = 0x3C, // i64.store8 m
	I64Store16        = 0x3D, // i64.store16 m
	I64Store32        = 0x3E, // i64.store32 m
	MemorySize        = 0x3F, // memory.size
	MemoryGrow        = 0x40, // memory.grow
	I32Const          = 0x41, // i32.const n
	I64Const          = 0x42, // i64.const n
	F32Const          = 0x43, // f32.const z
	F64Const          = 0x44, // f64.const z
	I32Eqz            = 0x45, // i32.eqz
	I32Eq             = 0x46, // i32.eq
	I32Ne             = 0x47, // i32.ne
	I32LtS            = 0x48, // i32.lt_s
	I32LtU            = 0x49, // i32.lt_u
	I32GtS            = 0x4A, // i32.gt_s
	I32GtU            = 0x4B, // i32.gt_u
	I32LeS            = 0x4C, // i32.le_s
	I32LeU            = 0x4D, // i32.le_u
	I32GeS            = 0x4E, // i32.ge_s
	I32GeU            = 0x4F, // i32.ge_u
	I64Eqz            = 0x50, // i64.eqz
	I64Eq             = 0x51, // i64.eq
	I64Ne             = 0x52, // i64.ne
	I64LtS            = 0x53, // i64.lt_s
	I64LtU            = 0x54, // i64.lt_u
	I64GtS            = 0x55, // i64.gt_s
	I64GtU            = 0x56, // i64.gt_u
	I64LeS            = 0x57, // i64.le_s
	I64LeU            = 0x58, // i64.le_u
	I64GeS            = 0x59, // i64.ge_s
	I64GeU            = 0x5A, // i64.ge_u
	F32Eq             = 0x5B, // f32.eq
	F32Ne             = 0x5C, // f32.ne
	F32Lt             = 0x5D, // f32.lt
	F32Gt             = 0x5E, // f32.gt
	F32Le             = 0x5F, // f32.le
	F32Ge             = 0x60, // f32.ge
	F64Eq             = 0x61, // f64.eq
	F64Ne             = 0x62, // f64.ne
	F64Lt             = 0x63, // f64.lt
	F64Gt             = 0x64, // f64.gt
	F64Le             = 0x65, // f64.le
	F64Ge             = 0x66, // f64.ge
	I32Clz            = 0x67, // i32.clz
	I32Ctz            = 0x68, // i32.ctz
	I32PopCnt         = 0x69, // i32.popcnt
	I32Add            = 0x6A, // i32.add
	I32Sub            = 0x6B, // i32.sub
	I32Mul            = 0x6C, // i32.mul
	I32DivS           = 0x6D, // i32.div_s
	I32DivU           = 0x6E, // i32.div_u
	I32RemS           = 0x6F, // i32.rem_s
	I32RemU           = 0x70, // i32.rem_u
	I32And            = 0x71, // i32.and
	I32Or             = 0x72, // i32.or
	I32Xor            = 0x73, // i32.xor
	I32Shl            = 0x74, // i32.shl
	I32ShrS           = 0x75, // i32.shr_s
	I32ShrU           = 0x76, // i32.shr_u
	I32Rotl           = 0x77, // i32.rotl
	I32Rotr           = 0x78, // i32.rotr
	I64Clz            = 0x79, // i64.clz
	I64Ctz            = 0x7A, // i64.ctz
	I64PopCnt         = 0x7B, // i64.popcnt
	I64Add            = 0x7C, // i64.add
	I64Sub            = 0x7D, // i64.sub
	I64Mul            = 0x7E, // i64.mul
	I64DivS           = 0x7F, // i64.div_s
	I64DivU           = 0x80, // i64.div_u
	I64RemS           = 0x81, // i64.rem_s
	I64RemU           = 0x82, // i64.rem_u
	I64And            = 0x83, // i64.and
	I64Or             = 0x84, // i64.or
	I64Xor            = 0x85, // i64.xor
	I64Shl            = 0x86, // i64.shl
	I64ShrS           = 0x87, // i64.shr_s
	I64ShrU           = 0x88, // i64.shr_u
	I64Rotl           = 0x89, // i64.rotl
	I64Rotr           = 0x8A, // i64.rotr
	F32Abs            = 0x8B, // f32.abs
	F32Neg            = 0x8C, // f32.neg
	F32Ceil           = 0x8D, // f32.ceil
	F32Floor          = 0x8E, // f32.floor
	F32Trunc          = 0x8F, // f32.trunc
	F32Nearest        = 0x90, // f32.nearest
	F32Sqrt           = 0x91, // f32.sqrt
	F32Add            = 0x92, // f32.add
	F32Sub            = 0x93, // f32.sub
	F32Mul            = 0x94, // f32.mul
	F32Div            = 0x95, // f32.div
	F32Min            = 0x96, // f32.min
	F32Max            = 0x97, // f32.max
	F32CopySign       = 0x98, // f32.copysign
	F64Abs            = 0x99, // f64.abs
	F64Neg            = 0x9A, // f64.neg
	F64Ceil           = 0x9B, // f64.ceil
	F64Floor          = 0x9C, // f64.floor
	F64Trunc          = 0x9D, // f64.trunc
	F64Nearest        = 0x9E, // f64.nearest
	F64Sqrt           = 0x9F, // f64.sqrt
	F64Add            = 0xA0, // f64.add
	F64Sub            = 0xA1, // f64.sub
	F64Mul            = 0xA2, // f64.mul
	F64Div            = 0xA3, // f64.div
	F64Min            = 0xA4, // f64.min
	F64Max            = 0xA5, // f64.max
	F64CopySign       = 0xA6, // f64.copysign
	I32WrapI64        = 0xA7, // i32.wrap_i64
	I32TruncF32S      = 0xA8, // i32.trunc_f32_s
	I32TruncF32U      = 0xA9, // i32.trunc_f32_u
	I32TruncF64S      = 0xAA, // i32.trunc_f64_s
	I32TruncF64U      = 0xAB, // i32.trunc_f64_u
	I64ExtendI32S     = 0xAC, // i64.extend_i32_s
	I64ExtendI32U     = 0xAD, // i64.extend_i32_u
	I64TruncF32S      = 0xAE, // i64.trunc_f32_s
	I64TruncF32U      = 0xAF, // i64.trunc_f32_u
	I64TruncF64S      = 0xB0, // i64.trunc_f64_s
	I64TruncF64U      = 0xB1, // i64.trunc_f64_u
	F32ConvertI32S    = 0xB2, // f32.convert_i32_s
	F32ConvertI32U    = 0xB3, // f32.convert_i32_u
	F32ConvertI64S    = 0xB4, // f32.convert_i64_s
	F32ConvertI64U    = 0xB5, // f32.convert_i64_u
	F32DemoteF64      = 0xB6, // f32.demote_f64
	F64ConvertI32S    = 0xB7, // f64.convert_i32_s
	F64ConvertI32U    = 0xB8, // f64.convert_i32_u
	F64ConvertI64S    = 0xB9, // f64.convert_i64_s
	F64ConvertI64U    = 0xBA, // f64.convert_i64_u
	F64PromoteF32     = 0xBB, // f64.promote_f32
	I32ReinterpretF32 = 0xBC, // i32.reinterpret_f32
	I64ReinterpretF64 = 0xBD, // i64.reinterpret_f64
	F32ReinterpretI32 = 0xBE, // f32.reinterpret_i32
	F64ReinterpretI64 = 0xBF, // f64.reinterpret_i64
	I32Extend8S       = 0xC0, // i32.extend8_s
	I32Extend16S      = 0xC1, // i32.extend16_s
	I64Extend8S       = 0xC2, // i64.extend8_s
	I64Extend16S      = 0xC3, // i64.extend16_s
	I64Extend32S      = 0xC4, // i64.extend32_s
	TruncSat          = 0xFC, // <i32|64>.trunc_sat_<f32|64>_<s|u>
}

lazy_static! {
	pub static ref OPNAMES: [&'static str; 256 as usize] = {
		let mut names: [&str; 256 as usize] = [""; 256];
			names[Opcode::Unreachable as usize] = "unreachable";
			names[Opcode::Nop as usize] = "nop";
			names[Opcode::Block as usize] = "block";
			names[Opcode::Loop as usize] = "loop";
			names[Opcode::If as usize] = "if";
			names[Opcode::Else_ as usize] = "else";
			names[Opcode::End_ as usize] = "end";
			names[Opcode::Br as usize] = "br";
			names[Opcode::BrIf as usize] = "br_if";
			names[Opcode::BrTable as usize] = "br_table";
			names[Opcode::Return as usize] = "return";
			names[Opcode::Call as usize] = "call";
			names[Opcode::CallIndirect as usize] = "call_indirect";
			names[Opcode::Drop as usize] = "drop";
			names[Opcode::Select as usize] = "select";
			names[Opcode::LocalGet as usize] = "local.get";
			names[Opcode::LocalSet as usize] = "local.set";
			names[Opcode::LocalTee as usize] = "local.tee";
			names[Opcode::GlobalGet as usize] = "global.get";
			names[Opcode::GlobalSet as usize] = "global.set";
			names[Opcode::I32Load as usize] = "i32.load";
			names[Opcode::I64Load as usize] = "i64.load";
			names[Opcode::F32Load as usize] = "f32.load";
			names[Opcode::F64Load as usize] = "f64.load";
			names[Opcode::I32Load8S as usize] = "i32.load8_s";
			names[Opcode::I32Load8U as usize] = "i32.load8_u";
			names[Opcode::I32Load16S as usize] = "i32.load16_s";
			names[Opcode::I32Load16U as usize] = "i32.load16_u";
			names[Opcode::I64Load8S as usize] = "i64.load8_s";
			names[Opcode::I64Load8U as usize] = "i64.load8_u";
			names[Opcode::I64Load16S as usize] = "i64.load16_s";
			names[Opcode::I64Load16U as usize] = "i64.load16_u";
			names[Opcode::I64Load32S as usize] = "i64.load32_s";
			names[Opcode::I64Load32U as usize] = "i64.load32_u";
			names[Opcode::I32Store as usize] = "i32.store";
			names[Opcode::I64Store as usize] = "i64.store";
			names[Opcode::F32Store as usize] = "f32.store";
			names[Opcode::F64Store as usize] = "f64.store";
			names[Opcode::I32Store8 as usize] = "i32.store8";
			names[Opcode::I32Store16 as usize] = "i32.store16";
			names[Opcode::I64Store8 as usize] = "i64.store8";
			names[Opcode::I64Store16 as usize] = "i64.store16";
			names[Opcode::I64Store32 as usize] = "i64.store32";
			names[Opcode::MemorySize as usize] = "memory.size";
			names[Opcode::MemoryGrow as usize] = "memory.grow";
			names[Opcode::I32Const as usize] = "i32.const";
			names[Opcode::I64Const as usize] = "i64.const";
			names[Opcode::F32Const as usize] = "f32.const";
			names[Opcode::F64Const as usize] = "f64.const";
			names[Opcode::I32Eqz as usize] = "i32.eqz";
			names[Opcode::I32Eq as usize] = "i32.eq";
			names[Opcode::I32Ne as usize] = "i32.ne";
			names[Opcode::I32LtS as usize] = "i32.lt_s";
			names[Opcode::I32LtU as usize] = "i32.lt_u";
			names[Opcode::I32GtS as usize] = "i32.gt_s";
			names[Opcode::I32GtU as usize] = "i32.gt_u";
			names[Opcode::I32LeS as usize] = "i32.le_s";
			names[Opcode::I32LeU as usize] = "i32.le_u";
			names[Opcode::I32GeS as usize] = "i32.ge_s";
			names[Opcode::I32GeU as usize] = "i32.ge_u";
			names[Opcode::I64Eqz as usize] = "i64.eqz";
			names[Opcode::I64Eq as usize] = "i64.eq";
			names[Opcode::I64Ne as usize] = "i64.ne";
			names[Opcode::I64LtS as usize] = "i64.lt_s";
			names[Opcode::I64LtU as usize] = "i64.lt_u";
			names[Opcode::I64GtS as usize] = "i64.gt_s";
			names[Opcode::I64GtU as usize] = "i64.gt_u";
			names[Opcode::I64LeS as usize] = "i64.le_s";
			names[Opcode::I64LeU as usize] = "i64.le_u";
			names[Opcode::I64GeS as usize] = "i64.ge_s";
			names[Opcode::I64GeU as usize] = "i64.ge_u";
			names[Opcode::F32Eq as usize] = "f32.eq";
			names[Opcode::F32Ne as usize] = "f32.ne";
			names[Opcode::F32Lt as usize] = "f32.lt";
			names[Opcode::F32Gt as usize] = "f32.gt";
			names[Opcode::F32Le as usize] = "f32.le";
			names[Opcode::F32Ge as usize] = "f32.ge";
			names[Opcode::F64Eq as usize] = "f64.eq";
			names[Opcode::F64Ne as usize] = "f64.ne";
			names[Opcode::F64Lt as usize] = "f64.lt";
			names[Opcode::F64Gt as usize] = "f64.gt";
			names[Opcode::F64Le as usize] = "f64.le";
			names[Opcode::F64Ge as usize] = "f64.ge";
			names[Opcode::I32Clz as usize] = "i32.clz";
			names[Opcode::I32Ctz as usize] = "i32.ctz";
			names[Opcode::I32PopCnt as usize] = "i32.popcnt";
			names[Opcode::I32Add as usize] = "i32.add";
			names[Opcode::I32Sub as usize] = "i32.sub";
			names[Opcode::I32Mul as usize] = "i32.mul";
			names[Opcode::I32DivS as usize] = "i32.div_s";
			names[Opcode::I32DivU as usize] = "i32.div_u";
			names[Opcode::I32RemS as usize] = "i32.rem_s";
			names[Opcode::I32RemU as usize] = "i32.rem_u";
			names[Opcode::I32And as usize] = "i32.and";
			names[Opcode::I32Or as usize] = "i32.or";
			names[Opcode::I32Xor as usize] = "i32.xor";
			names[Opcode::I32Shl as usize] = "i32.shl";
			names[Opcode::I32ShrS as usize] = "i32.shr_s";
			names[Opcode::I32ShrU as usize] = "i32.shr_u";
			names[Opcode::I32Rotl as usize] = "i32.rotl";
			names[Opcode::I32Rotr as usize] = "i32.rotr";
			names[Opcode::I64Clz as usize] = "i64.clz";
			names[Opcode::I64Ctz as usize] = "i64.ctz";
			names[Opcode::I64PopCnt as usize] = "i64.popcnt";
			names[Opcode::I64Add as usize] = "i64.add";
			names[Opcode::I64Sub as usize] = "i64.sub";
			names[Opcode::I64Mul as usize] = "i64.mul";
			names[Opcode::I64DivS as usize] = "i64.div_s";
			names[Opcode::I64DivU as usize] = "i64.div_u";
			names[Opcode::I64RemS as usize] = "i64.rem_s";
			names[Opcode::I64RemU as usize] = "i64.rem_u";
			names[Opcode::I64And as usize] = "i64.and";
			names[Opcode::I64Or as usize] = "i64.or";
			names[Opcode::I64Xor as usize] = "i64.xor";
			names[Opcode::I64Shl as usize] = "i64.shl";
			names[Opcode::I64ShrS as usize] = "i64.shr_s";
			names[Opcode::I64ShrU as usize] = "i64.shr_u";
			names[Opcode::I64Rotl as usize] = "i64.rotl";
			names[Opcode::I64Rotr as usize] = "i64.rotr";
			names[Opcode::F32Abs as usize] = "f32.abs";
			names[Opcode::F32Neg as usize] = "f32.neg";
			names[Opcode::F32Ceil as usize] = "f32.ceil";
			names[Opcode::F32Floor as usize] = "f32.floor";
			names[Opcode::F32Trunc as usize] = "f32.trunc";
			names[Opcode::F32Nearest as usize] = "f32.nearest";
			names[Opcode::F32Sqrt as usize] = "f32.sqrt";
			names[Opcode::F32Add as usize] = "f32.add";
			names[Opcode::F32Sub as usize] = "f32.sub";
			names[Opcode::F32Mul as usize] = "f32.mul";
			names[Opcode::F32Div as usize] = "f32.div";
			names[Opcode::F32Min as usize] = "f32.min";
			names[Opcode::F32Max as usize] = "f32.max";
			names[Opcode::F32CopySign as usize] = "f32.copysign";
			names[Opcode::F64Abs as usize] = "f64.abs";
			names[Opcode::F64Neg as usize] = "f64.neg";
			names[Opcode::F64Ceil as usize] = "f64.ceil";
			names[Opcode::F64Floor as usize] = "f64.floor";
			names[Opcode::F64Trunc as usize] = "f64.trunc";
			names[Opcode::F64Nearest as usize] = "f64.nearest";
			names[Opcode::F64Sqrt as usize] = "f64.sqrt";
			names[Opcode::F64Add as usize] = "f64.add";
			names[Opcode::F64Sub as usize] = "f64.sub";
			names[Opcode::F64Mul as usize] = "f64.mul";
			names[Opcode::F64Div as usize] = "f64.div";
			names[Opcode::F64Min as usize] = "f64.min";
			names[Opcode::F64Max as usize] = "f64.max";
			names[Opcode::F64CopySign as usize] = "f64.copysign";
			names[Opcode::I32WrapI64 as usize] = "i32.wrap_i64";
			names[Opcode::I32TruncF32S as usize] = "i32.trunc_f32_s";
			names[Opcode::I32TruncF32U as usize] = "i32.trunc_f32_u";
			names[Opcode::I32TruncF64S as usize] = "i32.trunc_f64_s";
			names[Opcode::I32TruncF64U as usize] = "i32.trunc_f64_u";
			names[Opcode::I64ExtendI32S as usize] = "i64.extend_i32_s";
			names[Opcode::I64ExtendI32U as usize] = "i64.extend_i32_u";
			names[Opcode::I64TruncF32S as usize] = "i64.trunc_f32_s";
			names[Opcode::I64TruncF32U as usize] = "i64.trunc_f32_u";
			names[Opcode::I64TruncF64S as usize] = "i64.trunc_f64_s";
			names[Opcode::I64TruncF64U as usize] = "i64.trunc_f64_u";
			names[Opcode::F32ConvertI32S as usize] = "f32.convert_i32_s";
			names[Opcode::F32ConvertI32U as usize] = "f32.convert_i32_u";
			names[Opcode::F32ConvertI64S as usize] = "f32.convert_i64_s";
			names[Opcode::F32ConvertI64U as usize] = "f32.convert_i64_u";
			names[Opcode::F32DemoteF64 as usize] = "f32.demote_f64";
			names[Opcode::F64ConvertI32S as usize] = "f64.convert_i32_s";
			names[Opcode::F64ConvertI32U as usize] = "f64.convert_i32_u";
			names[Opcode::F64ConvertI64S as usize] = "f64.convert_i64_s";
			names[Opcode::F64ConvertI64U as usize] = "f64.convert_i64_u";
			names[Opcode::F64PromoteF32 as usize] = "f64.promote_f32";
			names[Opcode::I32ReinterpretF32 as usize] = "i32.reinterpret_f32";
			names[Opcode::I64ReinterpretF64 as usize] = "i64.reinterpret_f64";
			names[Opcode::F32ReinterpretI32 as usize] = "f32.reinterpret_i32";
			names[Opcode::F64ReinterpretI64 as usize] = "f64.reinterpret_i64";
			names[Opcode::I32Extend8S as usize] = "i32.extend8_s";
			names[Opcode::I32Extend16S as usize] = "i32.extend16_s";
			names[Opcode::I64Extend8S as usize] = "i64.extend8_s";
			names[Opcode::I64Extend16S as usize] = "i64.extend16_s";
			names[Opcode::I64Extend32S as usize] = "i64.extend32_s";
			names[Opcode::TruncSat as usize] = "trunc_sat";
		names
	};
}