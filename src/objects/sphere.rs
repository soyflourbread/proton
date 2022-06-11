use crate::types::Float;
use crate::vector::Vector3D;

#[derive(Debug, Clone, Copy)]
pub struct Sphere<F: Float> {
    center: Vector3D<F>,
    radius: F,
}

impl<F: Float> Sphere<F> {
    pub fn new(center: Vector3D<F>, radius: F) -> Self {
        Self {
            center,
            radius,
        }
    }
}

impl<F: Float> Sphere<F> {
    pub fn center(&self) -> Vector3D<F> {
        self.center
    }

    pub fn radius(&self) -> F {
        self.radius
    }

    pub fn area(&self) -> F {
        let _four = F::from(4u32).unwrap();
        _four * F::PI() * self.radius * self.radius
    }
}
