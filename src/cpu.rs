use std::io::File;
use std::io::IoErrorKind::EndOfFile;
use std::cmp::max;

struct Memory {
    data_stack: Vec<i32>,
    address_stack: Vec<i32>,
    memory_space: Vec<i32>,
}

impl Memory {
    fn new(path: &Path) -> Memory {
        let mut file = File::open(path).unwrap();
        let size = file.stat().unwrap().size / 4;
        let mut memory = Vec::with_capacity(max(size as uint, 1024*1024)); // 4MB or image size

        loop {
            match file.read_le_i32() {
                Ok(x) => memory.push(x),
                Err(ref e) if e.kind == EndOfFile => break,
                Err(e) => panic!(e),
            }
        }
        Memory {
            data_stack: Vec::new(),
            address_stack: Vec::new(),
            memory_space: memory
        }
    }
}

struct CPU {
    memory: Memory,
    ip: i32,
}

impl CPU {
    fn new(path: &Path) -> CPU {
        CPU {
            memory: Memory::new(path),
            ip: 0,
        }
    }
}
