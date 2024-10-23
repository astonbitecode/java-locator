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
use std::error::Error;
use std::{fmt, result};

use glob;

pub type Result<T> = result::Result<T, JavaLocatorError>;

#[derive(Debug)]
pub struct JavaLocatorError {
    description: String,
}

impl JavaLocatorError {
    pub(crate) fn new(description: String) -> JavaLocatorError {
        JavaLocatorError { description }
    }
}

impl fmt::Display for JavaLocatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl Error for JavaLocatorError {
    fn description(&self) -> &str {
        self.description.as_str()
    }
}

impl From<std::io::Error> for JavaLocatorError {
    fn from(err: std::io::Error) -> JavaLocatorError {
        JavaLocatorError {
            description: format!("{:?}", err),
        }
    }
}

impl From<std::str::Utf8Error> for JavaLocatorError {
    fn from(err: std::str::Utf8Error) -> JavaLocatorError {
        JavaLocatorError {
            description: format!("{:?}", err),
        }
    }
}

impl From<glob::PatternError> for JavaLocatorError {
    fn from(err: glob::PatternError) -> JavaLocatorError {
        JavaLocatorError {
            description: format!("{:?}", err),
        }
    }
}
