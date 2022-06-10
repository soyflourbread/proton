use crate::types::Float;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3D<F: Float> {
    pub x: F,
    pub y: F,
    pub z: F,
}

impl<F: Float> Vector3D<F> {
    pub fn new(x: F, y: F, z: F) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    pub fn zero() -> Self {
        Self {
            x: F::zero(),
            y: F::zero(),
            z: F::zero(),
        }
    }

    pub fn one() -> Self {
        Self {
            x: F::one(),
            y: F::one(),
            z: F::one(),
        }
    }

    pub fn min_value() -> Self {
        Self {
            x: F::min_value(),
            y: F::min_value(),
            z: F::min_value(),
        }
    }

    pub fn max_value() -> Self {
        Self {
            x: F::max_value(),
            y: F::max_value(),
            z: F::max_value(),
        }
    }
}

impl<F: Float> Vector3D<F> {
    pub fn norm(self) -> Self {
        self / self.magnitude()
    }

    pub fn magnitude(self) -> F {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl<F: Float> Vector3D<F> {
    pub fn dot(self, op: Self) -> F {
        self.x * op.x + self.y * op.y + self.z * op.z
    }

    pub fn cross(self, op: Self) -> Self {
        Self {
            x: self.y * op.z - self.z * op.y,
            y: self.z * op.x - self.x * op.z,
            z: self.x * op.y - self.y * op.x,
        }
    }
}

impl<F: Float> Vector3D<F> {
    pub fn min(self, op: Self) -> Self {
        Self {
            x: self.x.min(op.x),
            y: self.y.min(op.y),
            z: self.z.min(op.z),
        }
    }

    pub fn max(self, op: Self) -> Self {
        Self {
            x: self.x.max(op.x),
            y: self.y.max(op.y),
            z: self.z.max(op.z),
        }
    }
}

impl<F: Float> std::ops::Neg for Vector3D<F> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl<F: Float> std::ops::Add<F> for Vector3D<F> {
    type Output = Self;

    fn add(self, op: F) -> Self {
        Self::new(self.x + op, self.y + op, self.z + op)
    }
}

impl<F: Float> std::ops::Sub<F> for Vector3D<F> {
    type Output = Self;

    fn sub(self, op: F) -> Self {
        Self::new(self.x - op, self.y - op, self.z - op)
    }
}

impl<F: Float> std::ops::Mul<F> for Vector3D<F> {
    type Output = Self;

    fn mul(self, op: F) -> Self {
        Self::new(self.x * op, self.y * op, self.z * op)
    }
}

impl<F: Float> std::ops::Div<F> for Vector3D<F> {
    type Output = Self;

    fn div(self, op: F) -> Self {
        Self::new(self.x / op, self.y / op, self.z / op)
    }
}

impl<F: Float> std::ops::Add<Vector3D<F>> for Vector3D<F> {
    type Output = Self;

    fn add(self, op: Vector3D<F>) -> Self {
        Self::new(self.x + op.x, self.y + op.y, self.z + op.z)
    }
}

impl<F: Float> std::ops::Sub<Vector3D<F>> for Vector3D<F> {
    type Output = Self;

    fn sub(self, op: Vector3D<F>) -> Self {
        Self::new(self.x - op.x, self.y - op.y, self.z - op.z)
    }
}

impl<F: Float> std::ops::Mul<Vector3D<F>> for Vector3D<F> {
    type Output = Self;

    fn mul(self, op: Vector3D<F>) -> Self {
        Self::new(self.x * op.x, self.y * op.y, self.z * op.z)
    }
}

impl<F: Float> std::ops::Div<Vector3D<F>> for Vector3D<F> {
    type Output = Self;

    fn div(self, op: Vector3D<F>) -> Self {
        Self::new(self.x / op.x, self.y / op.y, self.z / op.z)
    }
}

impl<F: Float> std::ops::AddAssign for Vector3D<F> {
    fn add_assign(&mut self, op: Self) {
        self.x = self.x + op.x;
        self.y = self.y + op.y;
        self.z = self.z + op.z;
    }
}

