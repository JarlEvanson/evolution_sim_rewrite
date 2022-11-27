#[repr(transparent)]
#[rustc_layout_scalar_valid_range_start(0)]
#[rustc_layout_scalar_valid_range_end(0xFF_FF_FF_FD)]
#[rustc_nonnull_optimization_guaranteed] // actually means non-niche not non-null
#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct GridID(u32);

#[allow(unused)]
impl GridID {
    pub const fn new(value: u32) -> Self {
        assert!(value < u32::MAX - 1);

        unsafe { Self(value) }
    }

    /// # Safety
    /// value must not be equal to u32::MAX or u32::MAX - 1
    pub const unsafe fn new_unchecked(value: u32) -> Self {
        unsafe { Self(value) }
    }

    pub const fn get(self) -> u32 {
        self.0
    }
}
#[cfg(test)]
mod test {
    use super::GridID;

    #[test]
    #[should_panic]
    fn invalid_values() {
        let x = GridID::new(u32::MAX);
        let y = GridID::new(u32::MAX - 1);
    }
}
