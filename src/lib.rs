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

//! # java-locator
//!
//! This is a small utility written in [Rust](https://www.rust-lang.org/).
//!
//! It locates the active Java installation in the host.
//!
//! ## Usage
//!
//! The utility can be used as a library, or as an executable:
//!
//! ### Library
//!
//! ```rust
//! extern crate java_locator;
//!
//! fn main() -> java_locator::errors::Result<()> {
//!     let java_home = java_locator::locate_java_home()?;
//!     let dyn_lib_path = java_locator::locate_jvm_dyn_library()?;
//!
//!     println!("The java home is {}", java_home);
//!     println!("The jvm dynamic library path is {}", dyn_lib_path);
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Executable
//!
//! Having rust [installed](https://www.rust-lang.org/tools/install), you may install the utility using cargo:
//!
//! `cargo install java_locator`
//!
//! And then, issuing
//!
//! `java-locator`
//!
//! you should have an output like:
//!
//! > /usr/lib/jvm/java-11-openjdk-amd64
//!
//! You may also retrieve the location of the `jvm` shared library:
//!
//! `java-locator --dynlib`
//!
//! should give an output like:
//!
//! > /usr/lib/jvm/java-11-openjdk-amd64/lib/server
//!
//! The latter may be used in cases when the `LD_LIBRARY_PATH` (or `PATH` in windows) should be populated.
//!

use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

use glob::glob;
use lazy_static::lazy_static;

pub mod errors;

pub const WINDOWS: &'static str = "windows";
pub const MACOS: &'static str = "macos";
pub const ANDROID: &'static str = "android";
pub const UNIX: &'static str = "unix";

lazy_static! {
    static ref TARGET_OS: String = {
        let target_os_res = env::var("CARGO_CFG_TARGET_OS");
        let tos = target_os_res.as_ref().map(|x| &**x).unwrap_or_else(|_| {
            if cfg!(windows) {
                WINDOWS
            } else if cfg!(macos) {
                MACOS
            } else if cfg!(android) {
                ANDROID
            } else {
                UNIX
            }
        });

        tos.to_string()
    };
}

fn is_windows() -> bool {
    &*TARGET_OS == WINDOWS
}

#[allow(dead_code)]
fn is_macos() -> bool {
    &*TARGET_OS == MACOS
}

#[allow(dead_code)]
fn is_android() -> bool {
    &*TARGET_OS == ANDROID
}

#[allow(dead_code)]
fn is_unix() -> bool {
    &*TARGET_OS == UNIX
}

/// Returns the Java home path.
///
/// If `JAVA_HOME` env var is defined, the function returns it without any checks whether the var points to a valid directory or not.
///
/// If `JAVA_HOME` is not defined, the function tries to locate it using the `java` executable.
pub fn locate_java_home() -> errors::Result<String> {
    match &env::var("JAVA_HOME") {
        Ok(s) if s.is_empty() => {
            try_locate_java_home()
        }
        Ok(java_home_env_var) => Ok(java_home_env_var.clone()),
        Err(_) => {
            try_locate_java_home()
        }
    }
}

fn try_locate_java_home() -> errors::Result<String> {
    // Prepare the command depending on the host
    let command_str = if is_windows() {
        "where"
    } else {
        "which"
    };

    let mut command = Command::new(command_str);

    command.arg("java");

    let output = command.output().map_err(|error| {
        let message = format!("Command '{}' is not found in the system PATH ({})", command_str, error.description());
        errors::JavaLocatorError::new(&message)
    })?;
    let java_exec_path = String::from_utf8(output.stdout)?;

    // Return early in case that the java executable is not found
    if java_exec_path.is_empty() {
        Err(errors::JavaLocatorError::new("Java is not installed or not added in the system PATH"))?
    }

    let mut test_path = PathBuf::from(java_exec_path.trim());

    while let Ok(path) = test_path.read_link() {
        test_path = if path.is_absolute() {
            path
        } else {
            test_path.pop();
            test_path.push(path);
            test_path
        };
    }

    // Here we should have found ourselves in a directory like /usr/lib/jvm/java-8-oracle/jre/bin/java
    test_path.pop();
    test_path.pop();

    match test_path.to_str() {
        Some(s) => Ok(String::from(s)),
        None => Err(errors::JavaLocatorError::new(&format!("Could not convert path {:?} to String", test_path))),
    }
}

/// Returns the path that contains the `libjvm.so` (or `jvm.dll` in windows).
pub fn locate_jvm_dyn_library() -> errors::Result<String> {
    let jvm_dyn_lib_file_name = if is_windows() {
        "jvm.dll"
    } else {
        "libjvm.*"
    };

    // Find the JAVA_HOME
    let java_home = locate_java_home()?;

    let query = format!("{}/**/{}", java_home, jvm_dyn_lib_file_name);

    let paths_vec: Vec<String> = glob(&query)?
        .filter_map(Result::ok)
        .map(|path_buf| {
            let mut pb = path_buf.clone();
            pb.pop();
            pb.to_str().unwrap_or("").to_string()
        })
        .filter(|s: &String| !s.is_empty())
        .collect();

    if paths_vec.is_empty() {
        let name = if is_windows() {
            "jvm.lib"
        } else {
            "libjvm"
        };
        Err(errors::JavaLocatorError::new(&format!("Could not find the {} in any subdirectory of {}", name, java_home)))
    } else {
        Ok(paths_vec[0].clone())
    }
}
