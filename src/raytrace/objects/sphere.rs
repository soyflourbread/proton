use crate::vector::Vector3D;
use crate::raytrace::{Incident, Ray, RayTraceable, ProcessedIncident, PartialBounded, Bounded, LightInteractable};
use crate::raytrace::materials::Material;
use crate::types::Float;

use super::base;

pub struct Sphere<F: Float> {
    inner: base::Sphere<F>,

    bound: BoundImpl<F>,

    material: Box<dyn Material<F>>,
}

impl<F: Float> Sphere<F> {
    pub fn new(center: Vector3D<F>, radius: F,
               material: Box<dyn Material<F>>) -> Self {
        let inner = base::Sphere::new(center, radius);

        let bound = BoundImpl::new(inner);

        Self { inner, bound, material }
    }
}

#[derive(Debug, Clone, Copy)]
struct BoundImpl<F: Float> {
    inner: base::Sphere<F>,

    radius_2: F,
}

impl<F: Float> BoundImpl<F> {
    pub fn new(inner: base::Sphere<F>) -> Self {
        Self {
            inner,

            radius_2: inner.radius() * inner.radius(),
        }
    }
}

impl<F: Float> Bounded<F> for BoundImpl<F> {
    fn hit(&self, ray: &Ray<F>) -> Option<Incident<F>> {
        let distance = ray.origin() - self.inner.center();
        let direction = ray.direction();

        let a = direction.dot(direction);
        let b = direction.dot(distance) * F::from(2 as u32).unwrap();
        let c = distance.dot(distance) - self.radius_2;

        if let Some((t0, t1)) = quadratic(a, b, c) {
            if t1 < F::zero() {
                return None;
            }

            let incident_dist = if t0 < F::zero() { t1 } else { t0 };
            let incident_coords = ray.origin() + ray.direction() * incident_dist;

            return Some(
                Incident::new(incident_coords,
                              (incident_coords - self.inner.center()).norm(),
                              incident_dist,
                              -ray.direction(),
                              false)
            );
        }

        None
    }
}

fn quadratic<F: Float>(a: F, b: F, c: F) -> Option<(F, F)> {
    let _neg_half = F::from(-0.5).unwrap();

    let discr: F = b * b - F::from(4).unwrap() * a * c;
    if discr < F::zero() {
        return None;
    }
    if discr == F::zero() {
        let x = _neg_half * b / a;
        return Some((x, x));
    }

    let q = if b > F::zero() {
        _neg_half * (b + discr.sqrt())
    } else {
        _neg_half * (b - discr.sqrt())
    };

    let x0 = q / a;
    let x1 = c / q;

    Some((x0.min(x1), x0.max(x1)))
}

impl<F: Float> Bounded<F> for Sphere<F> {
    fn hit(&self, ray: &Ray<F>) -> Option<Incident<F>> {
        self.bound.hit(ray)
    }
}

impl<F: Float> PartialBounded<F> for Sphere<F> {
    fn partial_hit(&self, ray: &Ray<F>) -> bool {
        self.hit(ray).is_some()
    }
}

impl<F: Float> LightInteractable<F> for Sphere<F> {
    fn interact(
        &self,
        incident: Incident<F>,
        seed: F,
    ) -> ProcessedIncident<F> {
        self.material.interact(incident, seed)
    }
}

impl<F: Float> RayTraceable<F> for Sphere<F> {
    fn name(&self) -> String {
        "sphere".to_string()
    }
}

