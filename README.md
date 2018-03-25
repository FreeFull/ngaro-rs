ngaro-rs
========

A Rust implementation of the Ngaro Virtual Machine.

Usage
-----

The virtual machine's primary software is the [RETRO 11 Forth](http://forthworks.com/retro/) distribution's retroImage file. Once you have downloaded it, you can run it using this command:

``` sh
cargo run --release path/to/retroImage
```

Optionally, you can also specify a path to the script you wanted to load:

``` sh
cargo run --release path/to/retroImage path/to/script.rx
```

Thanks
------

Thanks go to Charles Childers for finding bugs and allowing this code to reach a working state much faster than would be otherwise possible
