// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//~^^^^^^^^^^ ERROR overflow
//
// We get an error message at the top of file (dummy span).
// This is not helpful, but also kind of annoying to prevent,
// so for now just live with it.
// This test case was originally for issue #2258.

trait ToOpt {
    fn to_option(&self) -> Option<Self>;
}

impl ToOpt for usize {
    fn to_option(&self) -> Option<usize> {
        Some(*self)
    }
}

impl<T:Clone> ToOpt for Option<T> {
    fn to_option(&self) -> Option<Option<T>> {
        Some((*self).clone())
    }
}

fn function<T:ToOpt + Clone>(counter: usize, t: T) {
    if counter > 0_usize {
        function(counter - 1_usize, t.to_option());
        // FIXME(#4287) Error message should be here. It should be
        // a type error to instantiate `test` at a type other than T.
    }
}

fn main() {
    function(22_usize, 22_usize);
}
