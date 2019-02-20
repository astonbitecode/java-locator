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
use std::env;

use java_locator;

fn main() -> java_locator::errors::Result<()> {
    let args: Vec<String> = env::args().into_iter()
        .skip(1)
        .collect();

    let invalid_args: Vec<String> = args.iter()
        .filter(|arg| !is_valid_argument(&arg))
        .map(|s| s.to_string())
        .collect();

    if !invalid_args.is_empty() {
        panic!("Unknown arguments: {}", invalid_args.join(","));
    }

    if args.is_empty() {
        java_locator::locate_java_home().map(|s| println!("{}", s))
    } else {
        java_locator::locate_jvm_dyn_library().map(|s| println!("{}", s))
    }
}

fn is_valid_argument(arg: &str) -> bool {
    arg == "--dynlib" || arg == "-d"
}