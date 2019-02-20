# java-locator

This is a small utility written in [Rust](https://www.rust-lang.org/).

It locates the active Java installation in the host.

## Usage

The utility can be used as a library, or as an executable:

### Library

```rust
extern crate java_locator;

fn main() -> java_locator::errors::Result<()> {
    let java_home = java_locator::locate_java_home()?;
    let dyn_lib_path = java_locator::locate_jvm_dyn_library()?;
    
    println!("The java home is {}", java_home);
    println!("The jvm dynamic library path is {}", dyn_lib_path);
    
    Ok(())
}
```

### Executable

Having rust [installed](https://www.rust-lang.org/tools/install), you may install the utility using cargo:

`cargo install java_locator`

And then, issuing

`java-locator`

you should have an output like:

> /usr/lib/jvm/java-11-openjdk-amd64

You may also retrieve the location of the `jvm` shared library:

`java-locator --dynlib`

should give an output like:

> /usr/lib/jvm/java-11-openjdk-amd64/lib/server

The latter may be used in cases when the `LD_LIBRARY_PATH` (or `PATH` in windows) should be populated.

## Licence

At your option, under: 

* Apache License, Version 2.0, (http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (http://opensource.org/licenses/MIT)