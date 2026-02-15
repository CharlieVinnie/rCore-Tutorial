const BIG_STRIDE: usize = usize::MAX;

pub struct Priority {
    pub stride: usize,
    pub pass: usize,
}

impl Priority {
    pub fn new() -> Self {
        Self {
            stride: BIG_STRIDE / 16,
            pass: 0,
        }
    }
    pub fn set_level(&mut self, level: usize) {
        self.stride = BIG_STRIDE / level;
    }
    pub fn step(&mut self) {
        self.pass = self.pass.wrapping_add(self.stride);
    }
}

impl Eq for Priority {}

impl PartialEq for Priority {
    fn eq(&self, other: &Self) -> bool {
        self.pass == other.pass
    }
}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Priority {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let diff = other.pass.wrapping_sub(self.pass);
        if diff == 0 {
            core::cmp::Ordering::Equal
        } else if diff <= BIG_STRIDE / 2 {
            core::cmp::Ordering::Greater
        } else {
            core::cmp::Ordering::Less
        }
    }
}
