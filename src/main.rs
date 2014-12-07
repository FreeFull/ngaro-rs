#![feature(macro_rules, globs, unsafe_destructor)]

extern crate docopt;
extern crate serialize;
extern crate time;

use docopt::Docopt;

use cpu::CPU;
use cpu::Action as A;
use devices::Devices;

mod cpu;
mod devices;

static USAGE: &'static str = "
Usage: ngaro-rs <image>

Options:
    -h, --help      Show this message.
    -v, --version   Display the version.
";

#[deriving(Decodable)]
struct Args {
    arg_image: String,
    flag_help: bool,
    flag_version: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    let mut cpu = CPU::new(&Path::new(&*args.arg_image));
    let mut devices = Devices::new();
    // Can't use a for loop, due to issues with borrowing scope.
    loop {
        if let Some(action) = cpu.next() {
            match action {
                A::Wait => if devices.do_io(&mut cpu) { break; },
                _ => {}
            }
        } else { break; }
    }
}
