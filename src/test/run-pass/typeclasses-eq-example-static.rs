// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


// Example from lkuper's intern talk, August 2012 -- now with static
// methods!
use Color::{cyan, magenta, yellow, black};
use ColorTree::{leaf, branch};

trait Equal {
    fn isEq(a: &Self, b: &Self) -> bool;
}

enum Color { cyan, magenta, yellow, black }

impl Equal for Color {
    fn isEq(a: &Color, b: &Color) -> bool {
        match (*a, *b) {
          (cyan, cyan)       => { true  }
          (magenta, magenta) => { true  }
          (yellow, yellow)   => { true  }
          (black, black)     => { true  }
          _                  => { false }
        }
    }
}

enum ColorTree {
    leaf(Color),
    branch(Box<ColorTree>, Box<ColorTree>)
}

impl Equal for ColorTree {
    fn isEq(a: &ColorTree, b: &ColorTree) -> bool {
        match (a, b) {
          (&leaf(x), &leaf(y)) => { Equal::isEq(&x, &y) }
          (&branch(ref l1, ref r1), &branch(ref l2, ref r2)) => {
            Equal::isEq(&**l1, &**l2) && Equal::isEq(&**r1, &**r2)
          }
          _ => { false }
        }
    }
}

pub fn main() {
    assert!(Equal::isEq(&cyan, &cyan));
    assert!(Equal::isEq(&magenta, &magenta));
    assert!(!Equal::isEq(&cyan, &yellow));
    assert!(!Equal::isEq(&magenta, &cyan));

    assert!(Equal::isEq(&leaf(cyan), &leaf(cyan)));
    assert!(!Equal::isEq(&leaf(cyan), &leaf(yellow)));

    assert!(Equal::isEq(&branch(box leaf(magenta), box leaf(cyan)),
                &branch(box leaf(magenta), box leaf(cyan))));

    assert!(!Equal::isEq(&branch(box leaf(magenta), box leaf(cyan)),
                 &branch(box leaf(magenta), box leaf(magenta))));

    println!("Assertions all succeeded!");
}
