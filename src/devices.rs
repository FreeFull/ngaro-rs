use std::io::{Read, Write};

use fdstream::FdStream;

use byteorder::ReadBytesExt;

use time::get_time;

use cpu::CPU;

pub struct Devices {
    keyboard: Keyboard,
    char_screen: Screen,
}

impl Devices {
    pub fn new() -> Devices {
        Devices {
            keyboard: Keyboard { stdin: FdStream::new(0) },
            char_screen: Screen { stdout: FdStream::new(1) },
        }
    }

    pub fn do_io(&mut self, cpu: &mut CPU) -> bool {
        let cpu_info = cpu.get_info();
        let (mut ports, mut stack) = cpu.ports_and_stack();
        for (index, data) in ports.iter_mut().enumerate().skip(1) {
            if *data != 0 {
                match index {
                    1 => { *data = self.keyboard.read().unwrap(); }
                    2 => {
                        let value = stack.pop().expect("Data stack underflow.");
                        self.char_screen.display(value);
                        *data = 0;
                    }
                    5 => {
                        match *data {
                            -1 => { *data = cpu_info.memory_size; }
                            -5 => { *data = cpu_info.data_stack_depth; }
                            -6 => { *data = cpu_info.address_stack_depth; }
                            -8 => { *data = get_time().sec as i32; }
                            -9 => { return true; } // Exit
                            -11 => { *data = 80 } // TODO: Get actual width and height of terminal
                            -12 => { *data = 24 }
                            _ => { *data = 0; }
                        }
                    }
                    _ => { *data = 0 }
                }
            }
        }
        false // Don't exit
    }
}

struct Keyboard {
    stdin: FdStream,
}

impl Keyboard {
    fn read(&mut self) -> ::byteorder::Result<i32> {
        self.stdin.read_u8().map(|x| x as i32)
    }
}

struct Screen {
    stdout: FdStream,
}

impl Screen {
    fn display(&mut self, value: i32) {
        self.stdout.write_all(&[value as u8]).unwrap();
    }
}
