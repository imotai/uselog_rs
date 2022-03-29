//
//
// lib.rs
// Copyright (C) 2022 rtstore.io Author imrtstore <rtstore_dev@outlook.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

///
///# uselog_rs
///
///before using `uselog_rs`, you must use four lines code to use log for outputing log in `test mode` and `not test mode`
///
///```
///#[cfg(not(test))]
///use log::{debug, info, warn};
///#[cfg(test)]
///use std::{println as debug, println as info, println as warn};
///```
///now just one line code 
///
///```
///uselog!(debug, info, warn)
///```
///
///but you need to add `uselog_rs` to your parent module first like
///```
///#[macro_use(uselog)]
///extern crate uselog_rs;
///```
///
#[macro_export]
macro_rules! uselog {
    ($( $arg:tt ),* ) => {
        $(
#[cfg(not(test))]
use log::$arg;
#[cfg(test)]
use std::{println as $arg};
         )*
    };
}
