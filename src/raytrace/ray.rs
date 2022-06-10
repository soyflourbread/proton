use crate::types::Float;
use crate::vector::Vector3D;

#[derive(Debug, Clone, Copy)]
pub struct Ray<F: Float> {
    origin: Vector3D<F>,
    direction: Vector3D<F>,
}

impl<F: Float> Ray<F> {
    pub fn new(origin: Vector3D<F>, direction: Vector3D<F>) -> Self {
        Self::new_unchecked(origin, direction.norm())
    }

    pub fn new_unchecked(origin: Vector3D<F>, direction: Vector3D<F>) -> Self {
        Self {
            origin,
            direction,
        }
    }
}

impl<F: Float> Ray<F> {
    pub fn origin(&self) -> Vector3D<F> {
        self.origin
    }

    pub fn direction(&self) -> Vector3D<F> {
        self.direction
    }
}
