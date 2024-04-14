#![no_std]
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
#![warn(missing_docs)]

use core::time::Duration;

/// Implemented based on <https://gafferongames.com/post/fix_your_timestep>.
#[derive(Clone, Debug)]
pub struct GameLoop {
    target_frame_time: Duration,
    max_frame_time: Duration,
    accumulated_time: Duration,

    total_num_updates: u64,
    total_time_passed: Duration,
}

impl GameLoop {
    /// Create a new `GameLoop` instance.
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
    /// The real time can still be longer.
    #[inline]
    pub fn set_max_frame_time(&mut self, time: Duration) {
        self.max_frame_time = time;
    }

    /// Perform all calculations for an update.
    /// 
    /// You can do something like:
    /// 
    /// ```
    /// loop {
    ///     // handling events, input or whatever
    /// 
    ///     let elapsed = instance.elapsed(); // using `std::time::Instance` to measure time between updates
    ///     instance = Instance::now();
    /// 
    ///     let update_result = game_loop.update(elapsed).run(|update_result| {
    ///         // your actual update logic
    ///     });
    /// 
    ///     // rendering logic
    /// }
    /// ```
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

            exit: false,
        }
    }
}

/// The result of calling [`GameLoop::update`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UpdateResult {
    /// The number of updates.
    pub num_updates: u64,
    /// Total number of updates since [`GameLoop`]'s creation.
    pub total_num_updates: u64,

    /// Time between previous and current update.
    pub frame_time: Duration,
    /// Blending between current and next frames. Primarily useful for rendering.
    pub blending_factor: f64,

    /// Total time passed since [`GameLoop`]'s creation.
    /// This is a sum of the provided `elapsed` arguments.
    pub total_time_passed: Duration,

    /// Whether to exit next iteration.
    /// This is only useful in [`UpdateResult::run()`] or [`UpdateResult::run_result()`].
    pub exit: bool,
}

impl UpdateResult {
    /// Run the provided function [`UpdateResult::num_updates`] times.
    /// Aborts early if [`UpdateResult::exit`] is true.
    /// 
    /// Returns `self` for convenience.
    #[inline]
    pub fn run<F>(mut self, mut func: F) -> Self
    where
        F: FnMut(&mut Self),
    {
        for _i in 0..self.num_updates {
            (func)(&mut self);

            if self.exit {
                break;
            }
        }

        self
    }

    /// Run the provided function [`UpdateResult::num_updates`] times.
    /// Aborts early if `func` returns `Err` or [`UpdateResult::exit`] is true.
    /// 
    /// Returns `self` for convenience.
    #[inline]
    pub fn run_result<F, E>(mut self, mut func: F) -> Result<Self, E>
    where
        F: FnMut(&mut Self) -> Result<(), E>,
    {
        for _i in 0..self.num_updates {
            (func)(&mut self)?;

            if self.exit {
                break;
            }
        }

        Ok(self)
    }
}
