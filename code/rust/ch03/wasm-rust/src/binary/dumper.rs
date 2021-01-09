use super::module::*;

pub struct Dumper<'a> {
    module: &'a Module,
    func_count: usize,
    global_count: usize,
    mem_count: usize,
    table_count: usize,
}

impl<'a> Dumper<'a> {
    pub fn new(m: &'a Module) -> Self {
        Self {
            module: m,
            func_count: 0,
            global_count: 0,
            mem_count: 0,
            table_count: 0,
        }
    }

    pub fn dump(&mut self) {
        println!("Version: {:#04x}", self.module.version);

        self.dump_type_secs();
        self.dump_import_secs();
        self.dump_func_secs();
        self.dump_table_secs();
        self.dump_mem_secs();
        self.dump_global_secs();
        self.dump_export_secs();
        self.dump_start_secs();
        self.dump_elem_secs();
        self.dump_code_secs();
        self.dump_data_secs();
        self.dump_custom_secs();
    }
    pub fn dump_type_secs(&self) {
        println!("Type[{}]:", self.module.type_secs.len());
        for (i, sec) in self.module.type_secs.iter().enumerate() {
            println!("  type[{}]: {}", i, sec);
        }
    }
    pub fn dump_import_secs(&mut self) {
        println!("Import[{}]:", self.module.import_secs.len());
        for sec in self.module.import_secs.iter() {
            match &sec.desc {
                ImportDesc::ImportDescFuncType(type_idx) => {
                    println!("  func[{}]: {}.{}, sig={}", self.func_count, sec.module, sec.name, type_idx);
                    self.func_count += 1;
                },
                ImportDesc::ImportDescTable(table_type) => {
                    println!("  table[{}]: {}.{}, {:?}", self.table_count, sec.module, sec.name, table_type.limits);
                    self.table_count += 1;
                },
                ImportDesc::ImportDescMem(mem_type) => {
                    println!("  memory[{}]: {}.{}, {:?}", self.mem_count, sec.module, sec.name, mem_type);
                    self.mem_count += 1;
                },
                ImportDesc::ImportDescGlobal(global_type) => {
                    println!("  global[{}]: {}.{}, {:?}", self.global_count, sec.module, sec.name, global_type);
                    self.global_count += 1;
                },
            }
        }
    }
    pub fn dump_func_secs(&mut self) {
        println!("Function[{}]:", self.module.func_secs.len());
        for (i, sec) in self.module.func_secs.iter().enumerate() {
            println!("  func[{}]: sig={}", self.func_count+i, sec);
        }
    }
    pub fn dump_table_secs(&mut self) {
        println!("Table[{}]:", self.module.table_secs.len());
        for (i, sec) in self.module.table_secs.iter().enumerate() {
            println!("  table[{}]: {}", self.table_count+i, sec.limits);
        }
    }
    pub fn dump_mem_secs(&mut self) {
        println!("Memory[{}]:", self.module.mem_secs.len());
        for (i, sec) in self.module.mem_secs.iter().enumerate() {
            println!("  memory[{}]: {}", self.mem_count+i, sec);
        }
    }
    pub fn dump_global_secs(&mut self) {
        println!("Global[{}]:", self.module.global_secs.len());
        for (i, sec) in self.module.global_secs.iter().enumerate() {
            println!("  global[{}]: {}", self.global_count+i, sec.type_);
        }
    }
    pub fn dump_export_secs(&mut self) {
        println!("Import[{}]:", self.module.export_secs.len());
        for sec in self.module.export_secs.iter() {
            match &sec.desc.tag {
                ExportTag::Func => {
                    println!("  func[{}]: name={}", sec.desc.idx, sec.name);
                },
                ExportTag::Table => {
                    println!("  table[{}]: name={}", sec.desc.idx, sec.name);
                },
                ExportTag::Mem => {
                    println!("  memory[{}]: name={}", sec.desc.idx, sec.name);
                },
                ExportTag::Global => {
                    println!("  global[{}]: name={}", sec.desc.idx, sec.name);
                },
            }
        }
    }
    pub fn dump_start_secs(&mut self) {
        println!("Start:");
        if self.module.start_sec.is_some() {
            println!("  func={}", self.module.start_sec.unwrap());
        }
    }
    pub fn dump_elem_secs(&mut self) {
        println!("Element[{}]:", self.module.elem_secs.len());
        for (i, sec) in self.module.elem_secs.iter().enumerate() {
            println!("  elem[{}]: {}", i, sec.table); // TODO
        }
    }
    pub fn dump_code_secs(&mut self) {
        println!("Code[{}]:", self.module.code_secs.len());
        for (i, sec) in self.module.code_secs.iter().enumerate() {
            println!("  func[{}]: {}", self.func_count+i, sec); // TODO
        }
    }
    pub fn dump_data_secs(&mut self) {
        println!("Data[{}]:", self.module.data_secs.len());
        for (i, sec) in self.module.data_secs.iter().enumerate() {
            println!("  data[{}]: mem={}", i, sec.mem); // TODO
        }
    }
    pub fn dump_custom_secs(&mut self) {
        println!("Custom[{}]:", self.module.custom_secs.len());
        for (i, sec) in self.module.custom_secs.iter().enumerate() {
            println!("  custom[{}]: name={}", i, sec.name); // TODO
        }
    }
}