use std::path;

#[derive(Default)]
pub struct LinearScale {
    domain: (f64, f64),
    breakpoints: Vec<f64>,
}

impl LinearScale {
    pub fn with_breakpoints(self, breakpoints: Vec<f64>) -> Self {
        let mut range = (f64::INFINITY, f64::NEG_INFINITY);
        for b in breakpoints.iter() {
            range.0 = b.min(range.0);
            range.1 = b.max(range.1);
        }
        Self {
            breakpoints,
            ..self
        }
    }

    pub fn scale(&self, input: f64) -> f64 {
        let clamped_input = input.clamp(self.domain.0, self.domain.1);
        let frac = normalize_progress(clamped_input, &self.domain);

        let num_slices = (self.breakpoints.len() - 1) as f64;
        let curr_index = frac * num_slices;
        let slice_bounds = (curr_index.floor(), curr_index.floor() + 1.0);
        let slice_frac = normalize_progress(curr_index, &slice_bounds);
        let slice = (
            self.breakpoints[slice_bounds.0 as usize],
            self.breakpoints[slice_bounds.1 as usize],
        );
        slice.0 + slice_frac * (slice.1 - slice.0)
    }
}

fn normalize_progress(input: f64, domain: &(f64, f64)) -> f64 {
    (input - domain.0) / (domain.1 - domain.0)
}

pub struct Animator {
    pub frame_count: usize,
}

pub struct Frame {
    pub current: usize,
    count: usize,
}

impl Frame {
    pub fn new(current: usize, count: usize) -> Self {
        Self { current, count }
    }

    pub fn filename(&self, path: &str, name: &str, ext: &str) -> String {
        format!(
            "{}{}{}{:06}{}",
            path,
            path::MAIN_SEPARATOR,
            name,
            self.current,
            ext
        )
    }

    pub fn linear_scale(&self) -> LinearScale {
        LinearScale {
            domain: (0.0, self.count as f64),
            ..LinearScale::default()
        }
    }
}

impl Animator {
    pub fn new(frame_count: usize) -> Self {
        Self { frame_count }
    }

    pub fn animate<F>(&self, animate: F)
    where
        F: Fn(Frame),
    {
        for current_frame in 0..self.frame_count {
            animate(Frame::new(current_frame, self.frame_count))
        }
    }
}
