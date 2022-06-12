use crate::raytrace::{Incident, ProcessedIncident, Ray};
use crate::raytrace::objects::{Bounded, LightInteractable, LightSample, PartialBounded, RayTraceable};
use crate::types::Float;
use crate::vector::Vector3D;

pub struct Light<F: Float> {
    inner: Box<dyn RayTraceable<F>>,

    diff: Vector3D<F>,
}

impl<F: Float> Light<F> {
    pub fn new(
        inner: Box<dyn RayTraceable<F>>,
        diff: Vector3D<F>,
    ) -> Self {
        Self {
            inner,

            diff,
        }
    }
}

impl<F: Float> LightInteractable<F> for Light<F> {
    fn interact(&self, incident: Incident<F>, seed: F) -> ProcessedIncident<F> {
        self.inner.interact(incident, seed)
    }

    fn interact_predetermined(
        &self,
        incident: Incident<F>,
        w_r: Vector3D<F>,
        pdf: F,
        seed: F) -> ProcessedIncident<F> {
        self.inner.interact_predetermined(
            incident,
            w_r,
            pdf,
            seed,
        )
    }
}

impl<F: Float> Bounded<F> for Light<F> {
    fn hit(&self, ray: &Ray<F>) -> Option<Incident<F>> {
        self.inner.hit(ray)
    }
}

impl<F: Float> PartialBounded<F> for Light<F> {
    fn partial_hit(&self, ray: &Ray<F>) -> bool {
        self.inner.partial_hit(ray)
    }
}

impl<F: Float> RayTraceable<F> for Light<F> {
    fn name(&self) -> String {
        format!("light_{}", self.inner.name())
    }

    fn area(&self) -> F {
        self.inner.area()
    }
    fn emit(&self) -> Option<Vector3D<F>> {
        Some(self.diff)
    }

    fn focus(&self) -> bool {
        self.inner.focus()
    }

    fn sample_position(&self) -> (Vector3D<F>, Vector3D<F>, F) {
        self.inner.sample_position()
    }

    fn sample_direction(&self, coords: Vector3D<F>, normal: Vector3D<F>) -> (Vector3D<F>, F) {
        self.inner.sample_direction(coords, normal)
    }
}
