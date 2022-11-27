#![feature(rustc_attrs)]

use crate::non_max::NonMaxU32;

mod non_max;

mod grid;

fn main() {
    println!("{}", std::mem::size_of::<Option<NonMaxU32>>())
}
