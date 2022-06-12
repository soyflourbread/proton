use num::traits::real::Real;
use crate::vector::Vector3D;
use crate::raytrace::{Incident, Ray, ProcessedIncident, to_world};
use crate::raytrace::objects::{Bounded, LightInteractable, LightSample, PartialBounded, RayTraceable};
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

impl<F: Float> BoundImpl<F> {
    pub fn new(inner: base::Sphere<F>) -> Self {
        Self {
            inner,

            radius_2: inner.radius() * inner.radius(),
        }
    }

    fn hit_impl(&self, ray: &Ray<F>, inv: bool) -> Option<Incident<F>> {
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

            let normal = (incident_coords - self.inner.center()).norm();

            return Some(
                Incident::new(incident_coords,
                              if inv { -normal } else { normal },
                              incident_dist,
                              -ray.direction(),
                              inv)
            );
        }

        None
    }
}

impl<F: Float> Bounded<F> for BoundImpl<F> {
    fn hit(&self, ray: &Ray<F>) -> Option<Incident<F>> {
        self.hit_impl(ray, ray.inside())
    }
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

    fn interact_predetermined(
        &self,
        incident: Incident<F>,
        w_r: Vector3D<F>,
        pdf: F,
        seed: F) -> ProcessedIncident<F> {
        self.material.interact_predetermined(
            incident,
            w_r,
            pdf,
            seed,
        )
    }
}

impl<F: Float> RayTraceable<F> for Sphere<F> {
    fn name(&self) -> String {
        "sphere".to_string()
    }

    fn area(&self) -> F {
        self.inner.area()
    }
    fn emit(&self) -> Option<Vector3D<F>> {
        None
    }

    fn focus(&self) -> bool {
        self.material.focus()
    }

    fn sample_position(&self) -> (Vector3D<F>, Vector3D<F>, F) {
        let _two = F::from(2u32).unwrap();

        let normal = {
            let theta = _two * F::PI() * F::sample_rand();
            let phi = F::PI() * F::sample_rand();

            Vector3D::new(
                phi.cos(),
                phi.sin() * theta.cos(),
                phi.sin() * theta.sin())
        };

        let coords = self.inner.center() + normal.clone() * self.inner.radius();
        let position_pdf = F::one() / self.area();

        (coords, normal, position_pdf)
    }

    fn sample_direction(&self, coords: Vector3D<F>, normal: Vector3D<F>) -> (Vector3D<F>, F) {
        let local_direction = {
            let x_1 = F::sample_rand();
            let x_2 = F::sample_rand();
            let z = F::one().abs_sub(x_1 * F::from(2u32).unwrap());
            let r = (F::one() - z * z).sqrt();
            let phi: F = F::from(2u32).unwrap() * F::PI() * x_2;

            Vector3D::new(r * phi.cos(), r * phi.sin(), z)
        };
        let direction = to_world(local_direction, normal.clone());
        let direction_pdf = F::from(0.5 as f64).unwrap() * F::FRAC_1_PI();

        (direction, direction_pdf)
    }
}

