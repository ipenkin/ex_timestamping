// Copyright 2018 Ivan Penkin <grek.penkin@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
#![feature(test)]
#![allow(dead_code)]

#[macro_use]
extern crate failure;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate exonum;
extern crate exonum_time;
extern crate router;
extern crate bodyparser;
extern crate iron;
extern crate chrono;
extern crate base64;
#[macro_use]
extern crate base64_serde;

extern crate futures;
extern crate tempdir;
extern crate test;

pub mod timestamping;
pub mod blockchain;
#[cfg(test)]
mod benches;





