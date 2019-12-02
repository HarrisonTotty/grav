//! Defines some resources.
//!
//! Resources are common sets of data which is shared between systems.

/// Represents the various limits involving collision detection.
#[derive(Clone, Debug)]
pub struct CollisionLimits {
    /// The maximum distance two entities can be from each other and still be
    /// subject to collision detection.
    ///
    /// A distance greater than this value is automatically considered not
    /// collided.
    pub maximum_detection_theshold: f64,

    /// The minimum distance two entities can be from each other and still be
    /// subject to collision detection.
    ///
    /// A distance less than this value is automatically considered collided.
    pub minimum_detection_theshold: f64
}

/// Implements `std::default::Default` for `CollisionLimits`.
impl std::default::Default for CollisionLimits {
    fn default() -> Self {
        CollisionLimits {
            maximum_detection_theshold: 100.0,
            minimum_detection_theshold: 1.0,
        }
    }
}


/// Represents the amount of time between iterations.
#[derive(Clone, Debug)]
pub struct DeltaTime(pub f64);

/// Implements `std::default::Default` for `DeltaTime`.
impl std::default::Default for DeltaTime {
    fn default() -> Self { DeltaTime(1.0) }
}

/// Represents the maximum and minimum magnitudes for acceleration, position,
/// and velocity.
#[derive(Clone, Debug)]
pub struct DynamicsLimits {
    /// The maximum acceleration magnitude.
    pub maximum_acceleration: f64,

    /// The maximum position magnitude (the radius of the universe).
    pub maximum_position: f64,
 
    /// The maximum velocity magnitude (speed).
    pub maximum_velocity: f64,

    /// The minimum acceleration magnitude.
    pub minimum_acceleration: f64,

    /// The minimum position magnitude (the radius of the universe).
    pub minimum_position: f64,
 
    /// The minimum velocity magnitude (speed).
    pub minimum_velocity: f64,
}

/// Implements `std::default::Default` for `DynamicsLimits`.
impl std::default::Default for DynamicsLimits {
    fn default() -> Self {
        DynamicsLimits {
            maximum_acceleration: std::f64::INFINITY,
            maximum_position: std::f64::INFINITY,
            maximum_velocity: std::f64::INFINITY,
            minimum_acceleration: 0.0,
            minimum_position: 0.0,
            minimum_velocity: 0.0
        }
    }
}


/// Represents the electrostatic constant.
#[derive(Clone, Debug)]
pub struct ElectrostaticConstant(pub f64);

/// Implements `std::default::Default` for `ElectrostaticConstant`.
impl std::default::Default for ElectrostaticConstant {
    fn default() -> Self { ElectrostaticConstant(1.0) }
}


/// Represents the universal gravitational constant.
#[derive(Clone, Debug)]
pub struct GravitationalConstant(pub f64);

/// Implements `std::default::Default` for `GravitationalConstant`.
impl std::default::Default for GravitationalConstant {
    fn default() -> Self { GravitationalConstant(1.0) }
}


/// Represents the maximum and minimum magnitudes for angular acceleration,
/// and velocity.
///
/// Note that position is not included here since it is always made to be a unit
/// vector.
#[derive(Clone, Debug)]
pub struct OrientationLimits {
    /// The maximum angular acceleration magnitude.
    pub maximum_angular_acceleration: f64,
 
    /// The maximum angular velocity magnitude (speed).
    pub maximum_angular_velocity: f64,

    /// The minimum angular acceleration magnitude.
    pub minimum_angular_acceleration: f64,

    /// The minimum angular velocity magnitude (speed).
    pub minimum_angular_velocity: f64,
}

/// Implements `std::default::Default` for `OrientationLimits`.
impl std::default::Default for OrientationLimits {
    fn default() -> Self {
        OrientationLimits {
            maximum_angular_acceleration: std::f64::INFINITY,
            maximum_angular_velocity: std::f64::INFINITY,
            minimum_angular_acceleration: 0.0,
            minimum_angular_velocity: 0.0
        }
    }
}


/// Represents the output file path.
#[derive(Clone, Debug)]
pub struct OutputFile(pub String);

/// Implements `std::default::Default` for `OutputFile`.
impl std::default::Default for OutputFile {
    fn default() -> Self { OutputFile(String::from("output.yaml")) }
}


/// Represents splitting settings.
#[derive(Clone, Debug)]
pub struct SplittingSettings {
    /// The maximum lifetime an entity may be before it divides.
    pub maximum_lifetime: u128,

    /// The minimum lifetime an entity may be before it divides.
    pub minimum_lifetime: u128,

    /// The resulting particle pairs will be moved to this number multiplied by
    /// the original particle's radius.
    pub separation_multiplier: f64,

    /// Multiplies the magnitude of the velocity vectors of the resulting
    /// particles.
    pub velocity_multiplier: f64
}

/// Implements `std::default::Default` for `SplittingSettings`.
impl std::default::Default for SplittingSettings {
    fn default() -> Self {
        SplittingSettings {
            maximum_lifetime: 1000,
            minimum_lifetime: 100,
            separation_multiplier: 2.0,
            velocity_multiplier: 1.0
        }
    }
}
