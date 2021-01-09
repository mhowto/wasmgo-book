use std::path::PathBuf;
use structopt::StructOpt;

mod binary;

#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate lazy_static;

// use backtrace::Backtrace;
// use std::panic;

#[derive(StructOpt, Debug)]
#[structopt(name = "wasm-rust")]
struct Opt {
    #[structopt(short, long)]
    dump: bool,

    /// file to process
    #[structopt(name = "FILE", parse(from_os_str))]
    file: PathBuf,
}

fn main() {
    println!("{}", binary::OPNAMES[0]);
    let opt = Opt::from_args();

    let module = binary::decode_file(opt.file).expect("error to read file");

    if opt.dump {
        binary::Dumper::new(&module).dump();
    }
}