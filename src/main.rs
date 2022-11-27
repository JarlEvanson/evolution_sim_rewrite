#![feature(rustc_attrs)]

extern crate rand;

extern crate custom_dst;

mod grid_id;

mod grid;
use grid::{Grid, OccupantType};

mod gene;

mod compass;

mod neural_net;

mod population;

fn main() {
    println!("Size of {}", std::mem::size_of::<OccupantType>());

    let x = unsafe { Grid::new_unchecked(11, 11) };
}
