use std::io::{self, Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};

use byteorder::ReadBytesExt;

use termion::raw::{RawTerminal, IntoRawMode};
use termion;

use cpu::CPU;

fn get_time() -> i32 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i32
}

pub struct Devices<R: Read, W: Write> {
    keyboard: Keyboard<R>,
    char_screen: Screen<W>,
}

impl<R: Read, W: Write> Devices<R,W> {
    pub fn new(stdin: R, stdout: W) -> Devices<R,W> {
        Devices {
            keyboard: Keyboard { stdin: stdin },
            char_screen: Screen { stdout: stdout.into_raw_mode().unwrap() },
        }
    }

    pub fn do_io(&mut self, cpu: &mut CPU) -> bool {
        let cpu_info = cpu.get_info();
        let (mut ports, stack) = cpu.ports_and_stack();
        for (index, data) in ports.iter_mut().enumerate().skip(1) {
            if *data != 0 {
                match index {
                    1 => { *data = self.keyboard.read().unwrap(); }
                    2 => {
                        let value = stack.pop().expect("Data stack underflow.");
                        self.char_screen.display(value);
                        *data = 0;
                    }
                    3 => {
                        self.char_screen.flush();
                        *data = 0;
                    }
                    5 => {
                        match *data {
                            -1 => { *data = cpu_info.memory_size; }
                            -5 => { *data = cpu_info.data_stack_depth; }
                            -6 => { *data = cpu_info.address_stack_depth; }
                            -8 => { *data = get_time(); }
                            -9 => { return true; } // Exit
                            -11 => { *data = termion::terminal_size().unwrap().0 as i32 }
                            -12 => { *data = termion::terminal_size().unwrap().1 as i32 }
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

struct Keyboard<R: Read> {
    stdin: R,
}

impl<R: Read> Keyboard<R> {
    fn read(&mut self) -> io::Result<i32> {
        self.stdin.read_u8().map(|x| x as i32)
    }
}

struct Screen<W: Write> {
    stdout: RawTerminal<W>,
}

impl<W: Write> Screen<W> {
    fn display(&mut self, value: i32) {
        let output = [value as u8];
        let output: &[u8] = match value as u8 {
            b'\n' => {
                b"\r\n"
            },
            b'\x08' => { // Backspace
                b"\x08 \x08"
            },
            _ => {
                &output
            }
        };
        self.stdout.write_all(output).unwrap();
        self.flush();
    }

    fn flush(&mut self) {
        self.stdout.flush().ok();
    }
}
