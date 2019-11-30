//! Contains definitions of various mathematical constructs.

/// Represents the various shapes that an object may have.
#[derive(Clone, Copy, Debug)]
pub enum Shape {
    /// Represents a cuboid defined by the lengths from the central point to
    /// each side.
    Cuboid(f64, f64, f64),

    /// Represents a dimensionless point.
    Point,
    
    /// Represents a sphere with a particular radius.
    Sphere(f64),
}

/// Implements `std::default::Default` for `Shape`.
impl std::default::Default for Shape {
    fn default() -> Self { Shape::Point }
}


/// Represents a 3D mathematical vector.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct Vector(pub f64, pub f64, pub f64);

impl Vector {
    /// Returns the cross product between this vector and another one.
    pub fn cross(&self, other: Vector) -> Vector {
        Vector(
            (self.1 * other.2) - (self.2 * other.1),
            (self.2 * other.0) - (self.0 * other.2),
            (self.0 * other.1) - (self.1 * other.0)
        )
    }
    
    /// Returns the direction of this vector (as a unit vector).
    pub fn direction(&self) -> Vector {
        let mag = self.magnitude();
        if mag != 0.0 {
            *self / mag
        } else {
            Vector::default()
        }
    }

    /// Returns the dot product between this vector and another one.
    pub fn dot(&self, other: Vector) -> f64 {
        (self.0 * other.0) + (self.1 * other.1) + (self.2 * other.2)
    }

    /// Returns the magnitude of this vector.
    pub fn magnitude(&self) -> f64 {
        ((self.0 * self.0) + (self.1 * self.1) + (self.2 * self.2)).sqrt()
    }

    /// Returns a new random vector with the specified length restrictions.
    pub fn random(min: f64, max: f64) -> Vector {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        Vector(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0)).direction() * rng.gen_range(min, max)
    }
}

/// Implements `std::default::Default` for `Vector`.
impl std::default::Default for Vector {
    fn default() -> Self { Vector(0.0, 0.0, 0.0) }
}

// ----- Operator Implementations -----

/// Implements `std::ops::Add` between `Vector` and `Vector`.
impl std::ops::Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, other: Vector) -> Vector {
        Vector(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2
        )
    }
}

/// Implements `std::ops::AddAssign` between `Vector` and `Vector`.
impl std::ops::AddAssign<Vector> for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

/// Implements `std::ops::Add` between `Vector` and `f64`.
impl std::ops::Add<f64> for Vector {
    type Output = Vector;
    fn add(self, other: f64) -> Vector {
        Vector(
            self.0 + other,
            self.1 + other,
            self.2 + other
        )
    }
}

/// Implements `std::ops::AddAssign` between `Vector` and `f64`.
impl std::ops::AddAssign<f64> for Vector {
    fn add_assign(&mut self, rhs: f64) {
        self.0 += rhs;
        self.1 += rhs;
        self.2 += rhs;
    }
}

/// Implements `std::ops::Div` between `Vector` and `Vector`.
impl std::ops::Div<Vector> for Vector {
    type Output = Vector;
    fn div(self, other: Vector) -> Vector {
        Vector(
            self.0 / other.0,
            self.1 / other.1,
            self.2 / other.2
        )
    }
}

/// Implements `std::ops::DivAssign` between `Vector` and `Vector`.
impl std::ops::DivAssign<Vector> for Vector {
    fn div_assign(&mut self, rhs: Vector) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
        self.2 /= rhs.2;
    }
}

/// Implements `std::ops::Div` between `Vector` and `f64`.
impl std::ops::Div<f64> for Vector {
    type Output = Vector;
    fn div(self, other: f64) -> Vector {
        Vector(
            self.0 / other,
            self.1 / other,
            self.2 / other
        )
    }
}

/// Implements `std::ops::DivAssign` between `Vector` and `f64`.
impl std::ops::DivAssign<f64> for Vector {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

/// Implements `std::ops::Mul` between `Vector` and `Vector`.
impl std::ops::Mul<Vector> for Vector {
    type Output = Vector;
    fn mul(self, other: Vector) -> Vector {
        Vector(
            self.0 * other.0,
            self.1 * other.1,
            self.2 * other.2
        )
    }
}

/// Implements `std::ops::MulAssign` between `Vector` and `Vector`.
impl std::ops::MulAssign<Vector> for Vector {
    fn mul_assign(&mut self, rhs: Vector) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}

/// Implements `std::ops::Mul` between `Vector` and `f64`.
impl std::ops::Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, other: f64) -> Vector {
        Vector(
            self.0 * other,
            self.1 * other,
            self.2 * other
        )
    }
}

/// Implements `std::ops::MulAssign` between `Vector` and `f64`.
impl std::ops::MulAssign<f64> for Vector {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

/// Implements `std::ops::Neg` for `Vector`.
impl std::ops::Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Self::Output {
        Vector(
            -self.0,
            -self.1,
            -self.2
        )
    }
}

/// Implements `std::ops::Sub` between `Vector` and `Vector`.
impl std::ops::Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, other: Vector) -> Vector {
        Vector(
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2
        )
    }
}

/// Implements `std::ops::SubAssign` between `Vector` and `Vector`.
impl std::ops::SubAssign<Vector> for Vector {
    fn sub_assign(&mut self, rhs: Vector) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

/// Implements `std::ops::Sub` between `Vector` and `f64`.
impl std::ops::Sub<f64> for Vector {
    type Output = Vector;
    fn sub(self, other: f64) -> Vector {
        Vector(
            self.0 - other,
            self.1 - other,
            self.2 - other
        )
    }
}

/// Implements `std::ops::SubAssign` between `Vector` and `f64`.
impl std::ops::SubAssign<f64> for Vector {
    fn sub_assign(&mut self, rhs: f64) {
        self.0 -= rhs;
        self.1 -= rhs;
        self.2 -= rhs;
    }
}


// ---------- Other Implementations ----------

/// Implements `std::iter::Sum` for `Vector`.
impl<'a> std::iter::Sum<&'a Self> for Vector {
    fn sum<I>(iter: I) -> Self where I: Iterator<Item = &'a Self> {
        let mut result = Vector::default();
        for i in iter {
            result += *i;
        }
        return result;
    }
}

///// Implements `Serialize` for `Vector`.
//impl serde::Serialize for Vector {
//    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
//        serializer.serialize_str(&format!("[{}, {}, {}]", self.0, self.1, self.2))
//    }
//}
