use std::num::NonZeroU32;

use rand::{distributions::Uniform, rngs::ThreadRng, Rng};

use crate::grid_id::GridID;

pub struct Grid {
    pub width: NonZeroU32,
    pub height: NonZeroU32,
    grid: Box<[OccupantType]>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum OccupantType {
    Cell(GridID),
    Wall,
    None,
}

type GridLength = NonZeroU32;

impl Grid {
    pub fn new(width: u32, height: u32) -> Grid {
        assert!(width != 0 && height != 0);
        let cells = width.checked_mul(height);
        assert!(cells.is_some());
        Grid {
            width: unsafe { NonZeroU32::new_unchecked(width) },
            height: unsafe { NonZeroU32::new_unchecked(height) },
            grid: vec![OccupantType::None; unsafe { cells.unwrap_unchecked() as usize }]
                .into_boxed_slice(),
        }
    }

    /// # Safety
    /// If width * height overflow, then it results in UB
    pub unsafe fn new_unchecked(width: u32, height: u32) -> Grid {
        Grid {
            width: unsafe { NonZeroU32::new_unchecked(width) },
            height: unsafe { NonZeroU32::new_unchecked(height) },
            grid: vec![OccupantType::None; (width * height) as usize].into_boxed_slice(),
        }
    }

    pub fn get_occupant(&self, coords: (u32, u32)) -> OccupantType {
        self.grid[coords.0 as usize + coords.1 as usize * self.width.get() as usize]
    }

    pub fn set_occupant(&mut self, coords: (u32, u32), occupant: OccupantType) {
        self.grid[coords.0 as usize + coords.1 as usize * self.width.get() as usize] = occupant;
    }

    pub fn is_valid_loc(&self, coords: (u32, u32)) -> bool {
        return coords.0 < self.width.get() && coords.1 < self.height.get();
    }

    pub fn is_border(&self, coords: (u32, u32)) -> bool {
        return coords.0 == 0
            || coords.0 == self.width.get() - 1
            || coords.1 == 0
            || coords.1 == self.height.get() - 1;
    }

    pub fn visit_neighborhood<F>(&self, coords: (u32, u32), radius: f32, mut f: F)
    where
        F: FnMut((u32, u32)),
    {
        for x in -radius.floor() as isize..=radius.floor() as isize {
            if (x + coords.0 as isize).is_negative()
                || (x + coords.0 as isize) as u32 >= self.width.get()
            {
                continue;
            }
            let y_max = ((radius * radius) - (x * x) as f32).sqrt().floor() as u32;
            for y in coords.1.saturating_sub(y_max)..=coords.1.saturating_add(y_max) {
                let coords = ((x + coords.0 as isize) as u32, y);

                if self.is_valid_loc(coords) {
                    f(coords);
                }
            }
        }
    }

    pub fn find_empty_loc(&self, rng: &mut ThreadRng) -> (u32, u32) {
        loop {
            let coords = (
                rng.gen_range(0..self.width.get()),
                rng.gen_range(0..self.height.get()),
            );

            if self.get_occupant(coords) == OccupantType::None {
                return coords;
            }
        }
    }
}
