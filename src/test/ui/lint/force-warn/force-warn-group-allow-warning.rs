// compile-flags: --force-warns rust_2018_idioms -Zunstable-options
// check-pass

#![allow(bare_trait_objects)]

pub trait SomeTrait {}

pub fn function(_x: Box<SomeTrait>) {}
//~^ WARN trait objects without an explicit `dyn` are deprecated
//~| WARN this was previously accepted by the compiler

fn main() {}
