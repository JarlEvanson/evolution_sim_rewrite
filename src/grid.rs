use std::num::NonZeroU32;

use crate::non_max::NonMaxU32;

struct Grid {
    width: NonZeroU32,
    height: NonZeroU32,
    grid: Box<[Option<NonMaxU32>]>,
}
