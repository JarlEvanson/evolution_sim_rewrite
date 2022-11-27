use crate::neural_net::{ACTION_COUNT, SENSOR_COUNT};

const SENSOR: u32 = 1;
const TAIL: u32 = 1;

///# Layout of gene
/// Weight -> 0..16
///
/// Output -> 16..24
///
/// Input -> 24..32
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Gene(u32);

impl Gene {
    pub fn new(value: u32, inner_count: usize) -> Self {
        Self(value).normalize(inner_count)
    }

    fn normalize(self, inner_count: usize) -> Self {
        let head_id = (self.get_head() % (SENSOR_COUNT + inner_count)) as u32;
        let tail_id = (self.get_head() % (ACTION_COUNT + inner_count)) as u32;
        let weight = self.0 & 0xFF_FF;

        Self(weight | (tail_id << 16) | (head_id << 24))
    }

    pub fn get_head(self) -> usize {
        ((self.0 >> 24) & 0xFF) as usize
    }

    pub fn get_tail(self) -> usize {
        ((self.0 >> 16) & 0xFF) as usize
    }

    pub fn get_weight(self) -> f32 {
        (self.0 & 0xFF_FF) as f32 / (u32::MAX >> 3) as f32 - 4.0
    }
}
