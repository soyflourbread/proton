mod sphere;
mod mesh;
mod light;

pub use sphere::Sphere;
pub use mesh::Mesh;
pub use light::Light;

use crate::objects as base;

use crate::raytrace::{Incident, ProcessedIncident, Ray, to_world};
use crate::types::Float;
use crate::vector::Vector3D;

pub trait LightInteractable<F: Float> {
    fn interact(
        &self,
        incident: Incident<F>,
        seed: F
    ) -> ProcessedIncident<F>;

    fn interact_predetermined(
        &self,
        incident: Incident<F>,
        w_r: Vector3D<F>,
        pdf: F,
        seed: F
    ) -> ProcessedIncident<F>;
}

pub trait Bounded<F: Float> {
    fn hit(&self, ray: &Ray<F>) -> Option<Incident<F>>;
}

pub trait PartialBounded<F: Float> {
    fn partial_hit(&self, ray: &Ray<F>) -> bool;
}

pub trait RayTraceable<F: Float>
: LightInteractable<F> + Bounded<F> + PartialBounded<F> {
    fn name(&self) -> String;

    fn area(&self) -> F;
    fn emit(&self) -> Option<Vector3D<F>>;

    fn focus(&self) -> bool;

    fn sample_position(&self) -> (Vector3D<F>, Vector3D<F>, F);
    fn sample_direction(&self, coords: Vector3D<F>, normal: Vector3D<F>) -> (Vector3D<F>, F);

    fn sample_light(&self) -> LightSample<F> {
        let (coords, normal, position_pdf) = self.sample_position();
        let (direction, direction_pdf) = self.sample_direction(coords, normal);

        let ray = Ray::new(coords, direction);

        LightSample {
            ray,
            normal,
            position_pdf,
            direction_pdf,
        }
    }
}

pub struct LightSample<F: Float> {
    pub ray: Ray<F>,
    pub normal: Vector3D<F>,

    pub position_pdf: F,
    pub direction_pdf: F,
}

