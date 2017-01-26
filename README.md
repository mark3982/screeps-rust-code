# screeps-rust-code
A skeleton framework for writing a script for Screeps in Rust. Can be used on conjunction with existing code.

## Current State

Alpha; Likely broke. The current state is brand new. It is likely broken. Missing critical features.

## Missing Features

There is no heap allocation and deallocation routines. The libc library has been excluded to reduce the code size. There are
also likely other missing features such as proper handling of panics and printing of output. I believe these can all be implemented
more efficiently by leveraging the specifics of the environment provided by Screeps rather than attemping to bolt libc and
friends onto and exploding the code size.

## How to Use

The Rust code is written using `\src\lib.rs` like a normal Rust project; however, it leverages Screeps specific glue code. The names
of certain attributes have been changed to match the Rust style. The attribute `hits` is still `hits`, but `hitsMax` is now `hits_max`
hence all lowercase letters and the underscore seperating parts of a name.

Python is required to execute the build script. I wished to integrate everything into Cargo but have not been successful at this point.
The build script can be executed, while in the directory, `python build.py`. 

The build script requires `emcc` which is theEemscripten compiler. You can get Emscripten at:

  * https://kripken.github.io/emscripten-site/index.html
  
The build script will first execute cargo to compile the source in release mode targeting the `asmjs-unknown-emscripten` build tuple. This
will produce an rlib as `/target/asmjs-unknown-emscripten/release/librust_screeps_code.rlib` in the project directory.

Next, the build script executes `emcc` to take this and produce ASMJS output; however, that output is not in quite the perfect format so
it will be read in and modified as needed. Finally, the needed files are moved to `/output/` and the tempoary files are deleted.

The result is two files `/output/rust.boot.js` and `/output/rust.asm.js`. The `rust.asm.js` contains all of the ASMJS output with a slight
modification. The `rust.boot.js` contains the code needed to initialize the environment for the ASMJS code.

How your Screeps development environment is setup will dictate how you get those two files (`rust.boot.js` and `rust.asm.js`) uploaded; 
however, once uploaded you need to, if not existing already, create a file with the main loop in it then require the `rust.boot.js` as a 
a module. You code would look like this:

```
var rust = require('rust.boot');

module.exports.loop = function() {
    // You can put any code here.
    rust.run();
    // You can put any code here.
    
    // You can even call rust.run() in other modules in other places.
    other_stuff();
}
```

This allows you to leverage Rust code in your existing codebase. You just need to integrate the upload of
the `rust.boot.js` and `rust.asm.js` files into your current Screep's project.
