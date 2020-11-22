use std::path::PathBuf;
use structopt::StructOpt;

mod binary;

#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate num_derive;

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
    let opt = Opt::from_args();

    let decoded_contents = binary::decode_file(opt.file).expect("error to read file");

    if opt.dump {
        dump(decoded_contents);
    }
}

fn dump(module: binary::Module) {
    println!("{:?}", module)
}
