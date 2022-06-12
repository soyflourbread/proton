use std::sync::Arc;
use crate::raytrace::RayTraceable;
use crate::types::Float;
use crate::vector::Vector3D;

#[derive(Debug, Clone, Copy)]
pub struct Ray<F: Float> {
    origin: Vector3D<F>,
    direction: Vector3D<F>,

    inside: bool,
}

impl<F: Float> Ray<F> {
    pub fn from_inside(origin: Vector3D<F>, direction: Vector3D<F>) -> Self {
        Self::from_inside_unchecked(origin, direction.norm())
    }

    pub fn from_inside_unchecked(origin: Vector3D<F>, direction: Vector3D<F>) -> Self {
        Self {
            origin,
            direction,
            inside: true,
        }
    }

    pub fn new(origin: Vector3D<F>, direction: Vector3D<F>) -> Self {
        Self::new_unchecked(origin, direction.norm())
    }

    pub fn new_unchecked(origin: Vector3D<F>, direction: Vector3D<F>) -> Self {
        Self {
            origin,
            direction,
            inside: false,
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

    pub fn inside(&self) -> bool {
        self.inside
    }
}
