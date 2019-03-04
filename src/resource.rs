//! Contains ECS resource definitions.

use specs::prelude::*;

/// Represents the amount of time passed between iterations.
#[derive(Default)]
struct DT(f64);

/// Represents the value of the gravitiational constant.
#[derive(Default)]
struct G(f64);
