// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_camel_case_types)]
#![allow(dead_code)]

// pretty-expanded FIXME #23616

enum colour { red, green, blue, }

enum tree { children(Box<list>), leaf(colour), }

enum list { cons(Box<tree>, Box<list>), nil, }

enum small_list { kons(isize, Box<small_list>), neel, }

pub fn main() { }
