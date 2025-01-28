// Copyright 2019 astonbitecode
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/*!

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

## License

At your option, under:

* Apache License, Version 2.0, (http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (http://opensource.org/licenses/MIT)

 */

use std::env;
use std::path::PathBuf;
use std::process::Command;

use errors::{JavaLocatorError, Result};
use glob::{glob, Pattern};

pub mod errors;

/// Returns the name of the jvm dynamic library:
///
/// * libjvm.so for Linux
///
/// * libjvm.dlyb for Macos
///
/// * jvm.dll for Windows
pub fn get_jvm_dyn_lib_file_name() -> &'static str {
    if cfg!(target_os = "windows") {
        "jvm.dll"
    } else if cfg!(target_os = "macos") {
        "libjvm.dylib"
    } else {
        "libjvm.so"
    }
}

/// Returns the Java home path.
///
/// If `JAVA_HOME` env var is defined, the function returns it without any checks whether the var points to a valid directory or not.
///
/// If `JAVA_HOME` is not defined, the function tries to locate it using the `java` executable.
pub fn locate_java_home() -> Result<String> {
    match &env::var("JAVA_HOME") {
        Ok(s) if s.is_empty() => do_locate_java_home(),
        Ok(java_home_env_var) => Ok(java_home_env_var.clone()),
        Err(_) => do_locate_java_home(),
    }
}

#[cfg(target_os = "windows")]
fn do_locate_java_home() -> Result<String> {
    let output = Command::new("where")
        .arg("java")
        .output()
        .map_err(|e| JavaLocatorError::new(format!("Failed to run command `where` ({e})")))?;

    let java_exec_path_raw = std::str::from_utf8(&output.stdout)?;
    java_exec_path_validation(java_exec_path_raw)?;

    // Windows will return multiple lines if there are multiple `java` in the PATH.
    let paths_found = java_exec_path_raw.lines().count();
    if paths_found > 1 {
        eprintln!("WARNING: java_locator found {paths_found} possible java locations. Using the first one. To silence this warning set JAVA_HOME env var.")
    }

    let java_exec_path = java_exec_path_raw
        .lines()
        // The first line is the one that would be run, so take just that line.
        .next()
        .expect("gauranteed to have at least one line by java_exec_path_validation")
        .trim();

    let mut home_path = follow_symlinks(java_exec_path);

    home_path.pop();
    home_path.pop();

    home_path
        .into_os_string()
        .into_string()
        .map_err(|path| JavaLocatorError::new(format!("Java path {path:?} is invalid utf8")))
}

#[cfg(target_os = "macos")]
fn do_locate_java_home() -> Result<String> {
    let output = Command::new("/usr/libexec/java_home")
        .output()
        .map_err(|e| {
            JavaLocatorError::new(format!(
                "Failed to run command `/usr/libexec/java_home` ({e})"
            ))
        })?;

    let java_exec_path = std::str::from_utf8(&output.stdout)?.trim();

    java_exec_path_validation(java_exec_path)?;
    let home_path = follow_symlinks(java_exec_path);

    home_path
        .into_os_string()
        .into_string()
        .map_err(|path| JavaLocatorError::new(format!("Java path {path:?} is invalid utf8")))
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))] // Unix
fn do_locate_java_home() -> Result<String> {
    let output = Command::new("which")
        .arg("java")
        .output()
        .map_err(|e| JavaLocatorError::new(format!("Failed to run command `which` ({e})")))?;
    let java_exec_path = std::str::from_utf8(&output.stdout)?.trim();

    java_exec_path_validation(java_exec_path)?;
    let mut home_path = follow_symlinks(java_exec_path);

    // Here we should have found ourselves in a directory like /usr/lib/jvm/java-8-oracle/jre/bin/java
    home_path.pop();
    home_path.pop();

    // Java 8(aka 1.8) has a slightly different directory structure,
    // where java is in the ${JAVA_HOME}/jre/bin/java directory, and ${JAVA_HOME}/bin/java is just a symlink.
    // Since we recursively follow symlinks, we end up in the wrong directory,
    // so we need to pop one more time.
    #[cfg(feature = "legacy-java-compat")]
    if let Some(last_section) = home_path.file_name() {
        if last_section == "jre" {
            home_path.pop();
        }
    }

    home_path
        .into_os_string()
        .into_string()
        .map_err(|path| JavaLocatorError::new(format!("Java path {path:?} is invalid utf8")))
}

fn java_exec_path_validation(path: &str) -> Result<()> {
    if path.is_empty() {
        return Err(JavaLocatorError::new(
            "Java is not installed or not in the system PATH".into(),
        ));
    }

    Ok(())
}

fn follow_symlinks(path: &str) -> PathBuf {
    let mut test_path = PathBuf::from(path);
    while let Ok(path) = test_path.read_link() {
        test_path = if path.is_absolute() {
            path
        } else {
            test_path.pop();
            test_path.push(path);
            test_path
        };
    }
    test_path
}

/// Returns the path that contains the `libjvm.so` (or `jvm.dll` in windows).
pub fn locate_jvm_dyn_library() -> Result<String> {
    if cfg!(target_os = "windows") {
        locate_file("jvm.dll")
    } else {
        locate_file("libjvm.*")
    }
}

/// Returns the path that contains the file with the provided name.
///
/// This function argument can be a wildcard.
pub fn locate_file(file_name: &str) -> Result<String> {
    // Find the JAVA_HOME
    let java_home = locate_java_home()?;

    let query = format!("{}/**/{}", Pattern::escape(&java_home), file_name);

    let path = glob(&query)?.filter_map(|x| x.ok()).next().ok_or_else(|| {
        JavaLocatorError::new(format!(
            "Could not find the {file_name} library in any subdirectory of {java_home}",
        ))
    })?;

    let parent_path = path.parent().unwrap();
    match parent_path.to_str() {
        Some(parent_path) => Ok(parent_path.to_owned()),
        None => Err(JavaLocatorError::new(format!(
            "Java path {parent_path:?} is invalid utf8"
        ))),
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn locate_java_home_test() {
        println!("locate_java_home: {}", locate_java_home().unwrap());
        println!(
            "locate_jvm_dyn_library: {}",
            locate_jvm_dyn_library().unwrap()
        );
    }

    #[test]
    fn locate_java_from_exec_test() {
        println!("do_locate_java_home: {}", do_locate_java_home().unwrap());
    }

    #[test]
    fn jni_headers_test() {
        let java_home = do_locate_java_home().unwrap();
        assert!(PathBuf::from(java_home)
            .join("include")
            .join("jni.h")
            .exists());
    }
}
