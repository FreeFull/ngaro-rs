extern crate docopt;
extern crate byteorder;
extern crate termion;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::path::Path;
use std::fs::File;
use std::io::{stdin, stdout, Read};

use docopt::Docopt;

use cpu::CPU;
use cpu::Action as A;
use devices::Devices;

mod cpu;
mod devices;

static USAGE: &'static str = "
Usage:
  ngaro-rs <image> [<script>]
  ngaro-rs -h | --help
  ngaro-rs --version

Options:
    -h, --help      Show this message.
    --version   Display the version.
";

#[derive(Deserialize)]
struct Args {
    arg_image: String,
    arg_script: Option<String>,
    flag_help: bool,
    flag_version: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|args| args.parse())
        .and_then(|args| args.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.flag_help {
        println!("{}", USAGE);
        return;
    }

    if args.flag_version {
        println!("ngaro-rs 0.0.2 dev");
        return;
    }

    let mut cpu = CPU::new(Path::new(&*args.arg_image));
    let (stdin, stdout) = (stdin(), stdout());
    let mut stdin: Box<Read> = Box::new(stdin.lock());
    if let Some(script_path) = args.arg_script {
        let script = File::open(script_path).unwrap();
        stdin = Box::new(script.chain(stdin));
    }
    let mut devices = Devices::new(stdin, stdout.lock());
    // Can't use a for loop, due to issues with borrowing scope.
    while let Some(action) = cpu.next() {
        match action {
            A::Wait => if devices.do_io(&mut cpu) { break; },
            _ => {}
        }
    }
}
