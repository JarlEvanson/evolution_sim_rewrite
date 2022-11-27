#![feature(rustc_attrs)]

extern crate rand;

use grid::OccupantType;
use rand::thread_rng;

use crate::{grid::Grid, grid_id::GridID};

mod grid_id;

mod grid;

fn main() {
    println!("Size of {}", std::mem::size_of::<OccupantType>());

    let x = unsafe { Grid::new_unchecked(11, 11) };

    x.visit_neighborhood((10, 10), 1.0, |(x, y)| println!("({}, {})", x, y));

    //println!("{:?}", x.find_empty_loc(&mut thread_rng()));
}
