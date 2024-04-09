#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
#![warn(missing_docs)]

use core::time::Duration;

mod input;
pub use input::{Input, InputState};

/// Implemented based on <https://gafferongames.com/post/fix_your_timestep>.
#[derive(Clone, Debug)]
pub struct WinLoop {
    target_frame_time: Duration,
    max_frame_time: Duration,
    accumulated_time: Duration,

    total_num_updates: u64,
    total_time_passed: Duration,
}

impl WinLoop {
    #[inline]
    pub fn new(target_frame_time: Duration, max_frame_time: Duration) -> Self {
        Self {
            target_frame_time,
            max_frame_time,
            accumulated_time: Duration::ZERO,

            total_num_updates: 0,
            total_time_passed: Duration::ZERO,
        }
    }

    /// Set the desired (minimum) time between application updates.
    #[inline]
    pub fn set_target_frame_time(&mut self, time: Duration) {
        self.target_frame_time = time;
    }

    /// Set the maximum time between application updates.
    /// The real frame time can be longer, but `frame_time` will not exceed this value.
    #[inline]
    pub fn set_max_frame_time(&mut self, time: Duration) {
        self.max_frame_time = time;
    }

    pub fn update(&mut self, elapsed: Duration) -> UpdateResult {
        self.total_time_passed += elapsed;

        self.accumulated_time += if elapsed > self.max_frame_time {
            self.max_frame_time
        } else {
            elapsed
        };

        let mut num_updates = 0;

        while self.accumulated_time > self.target_frame_time {
            self.accumulated_time -= self.target_frame_time;
            num_updates += 1;
        }

        self.total_num_updates += num_updates;

        let blending_factor =
            self.accumulated_time.as_secs_f64() / self.target_frame_time.as_secs_f64();

        UpdateResult {
            num_updates,
            total_num_updates: self.total_num_updates,

            frame_time: self.target_frame_time,
            blending_factor,

            total_time_passed: self.total_time_passed,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UpdateResult {
    pub num_updates: u64,
    pub total_num_updates: u64,

    /// Time between previous and current update.
    pub frame_time: Duration,
    pub blending_factor: f64,

    pub total_time_passed: Duration,
}

impl UpdateResult {
    #[inline]
    pub fn run<F>(self, mut func: F)
    where
        F: FnMut(Self, f64),
    {
        let dt = self.frame_time.as_secs_f64();

        for _i in 0..self.num_updates {
            (func)(self, dt);
        }
    }

    #[inline]
    pub fn run_result<F, E>(self, mut func: F) -> Result<(), E>
    where
        F: FnMut(Self, f64) -> Result<(), E>,
    {
        let dt = self.frame_time.as_secs_f64();

        for _i in 0..self.num_updates {
            (func)(self, dt)?;
        }

        Ok(())
    }
}
