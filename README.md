ngaro-rs
========

A Rust implementation of the Ngaro Virtual Machine.

Usage
-----

The virtual machine's primary software is the [Retro Forth](http://www.forthworks.com/retro) distribution's retroImage file. Once you have downloaded it, you can run it using this command:

``` sh
stty -icanon -echo; cargo run --release /path/to/retroImage
```

Once it's done, you'll likely want to run `reset` to set your terminal back to default.

Thanks
------

Thanks go to Charles Childers for finding bugs and allowing this code to reach a working state much faster than would be otherwise possible
