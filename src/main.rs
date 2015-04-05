extern crate rustc_serialize;
extern crate docopt;
extern crate time;
extern crate byteorder;
extern crate fdstream;

use std::path::Path;

use docopt::Docopt;

use cpu::CPU;
use cpu::Action as A;
use devices::Devices;

mod cpu;
mod devices;

static USAGE: &'static str = "
Usage:
  ngaro-rs <image>
  ngaro-rs -h | --help
  ngaro-rs --version

Options:
    -h, --help      Show this message.
    --version   Display the version.
";

#[derive(RustcDecodable)]
struct Args {
    arg_image: String,
    flag_help: bool,
    flag_version: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    if args.flag_help {
        println!("{}", USAGE);
        return;
    }

    if args.flag_version {
        println!("ngaro-rs 0.0.1 dev");
        return;
    }

    let mut cpu = CPU::new(Path::new(&*args.arg_image));
    let mut devices = Devices::new();
    // Can't use a for loop, due to issues with borrowing scope.
    while let Some(action) = cpu.next() {
        match action {
            A::Wait => if devices.do_io(&mut cpu) { break; },
            _ => {}
        }
    }
}
