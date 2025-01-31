# java-locator

[![crates.io](https://img.shields.io/crates/v/java-locator.svg)](https://crates.io/crates/java_locator)
![Build](https://github.com/astonbitecode/java-locator/actions/workflows/ci-workflow.yml/badge.svg)

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
    let libjsig  = java_locator::locate_file("libjsig.so")?;
    
    println!("The java home is {}", java_home);
    println!("The jvm dynamic library path is {}", dyn_lib_path);
    println!("The file libjsig.so is located in {}", libjsig);
    
    Ok(())
}
```

### Executable

Having rust [installed](https://www.rust-lang.org/tools/install), you may install the utility using cargo:

`cargo install java-locator --features build-binary`

And then, issuing

`java-locator`

you should have an output like:

> /usr/lib/jvm/java-11-openjdk-amd64

You may retrieve the location of the `jvm` shared library:

`java-locator --jvmlib`

should give an output like:

> /usr/lib/jvm/java-11-openjdk-amd64/lib/server

This may be used in cases when the `LD_LIBRARY_PATH` (or `PATH` in windows) should be populated.

You may also retrieve the location of any file inside the Java installation:

`java-locator --file libjsig.so`

and you can even use wildcards:

`java-locator --file libjsig*`

The latter two commands should return something like:

> /usr/lib/jvm/java-11-openjdk-amd64/lib

## Available Features

* `build-binary`: Generates a `java-locator` executable
* `locate-jdk-only`: Instructs `java-locator` to locate __only JDKs__.

    In a system that has only JREs installed, `java-locator` will not find any Java installation if this feature is enabled.

    This feature also solves issues when using JDK 8:  In usual installations, the symlinks of the `java` executable in the `$PATH`
    lead to the `jre` directory that lies inside the JDK 8. When `$JAVA_HOME` is not defined in the system, `java-locator` attempts to locate the
    Java installation following the symlinks of the `java` executable. Having done that, it cannot locate development artifacts like `jni.h` headers,
    `javac` etc. With this feature enabled though, `java-locator` will locate development artifacts normally.

## License

At your option, under: 

* Apache License, Version 2.0, (http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (http://opensource.org/licenses/MIT)