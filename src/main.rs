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
use docopt::Docopt;

const USAGE: &str = "
java-locator locates the active Java installation in the host.

Usage:
  java-locator
  java-locator (-j | --jvmlib)
  java-locator (-f | --file) <name>
  java-locator (-h | --help)

Options:
  -h --help     Show this screen.
";

fn main() -> java_locator::errors::Result<()> {
    let args = Docopt::new(USAGE)
        .and_then(|dopt| dopt.parse())
        .unwrap_or_else(|e| e.exit());

    if args.find("--jvmlib").unwrap().as_bool() || args.find("-j").unwrap().as_bool() {
        java_locator::locate_jvm_dyn_library().map(|s| println!("{}", s))?;
    } else if args.find("--file").unwrap().as_bool() || args.find("-f").unwrap().as_bool() {
        java_locator::locate_file(args.get_str("<name>")).map(|s| println!("{}", s))?;
    } else {
        java_locator::locate_java_home().map(|s| println!("{}", s))?;
    }

    Ok(())
}
