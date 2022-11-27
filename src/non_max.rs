#[repr(transparent)]
#[rustc_layout_scalar_valid_range_start(0)]
#[rustc_layout_scalar_valid_range_end(0xFF_FF_FF_FE)]
#[rustc_nonnull_optimization_guaranteed] // actually means non-niche not non-null
pub struct NonMaxU32(u32);
