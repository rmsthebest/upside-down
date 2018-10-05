//! Silly program to flip arguments upside down
// This file is part of "upside-down", which is free software: you
// can redistribute it and/or modify it under the terms of the GNU General
// Public License version 3 as published by the Free Software Foundation. See
// <https://www.gnu.org/licenses/> for a copy.

use std::env;
mod upside_down;
use crate::upside_down::upside_down;

fn main() {
    let args = env::args().skip(1);
    if args.len() != 0 {
        for s in env::args().skip(1) {
            println!("{}", upside_down(&s));
        }
    } else {
        println!("How to use:\nInput: upside_down <text to flip>\nOutput: <word flipped>");
    }
}
