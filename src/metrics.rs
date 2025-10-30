use crate::PUZZLE_DIGITS;

pub struct Metrics {
    square_edits: [u64; PUZZLE_DIGITS],
    square_views: [u64; PUZZLE_DIGITS],
}

impl Metrics {
    pub fn edit(&mut self, idx: usize) {
        self.square_edits[idx] += 1;
    }

    pub fn view(&mut self, idx: usize) {
        self.square_views[idx] += 1;
    }

    pub fn log(&self) {
        log::info!("Total Square Edits: {}", self.square_edits.into_iter().sum::<u64>());
        log::info!("Total Square Views: {}", self.square_views.into_iter().sum::<u64>());
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self { square_edits: std::array::from_fn(|_| 0), square_views: std::array::from_fn(|_| 0) }
    }
}
