mod ray;
mod scene;
mod incident;
mod renderer;
mod bvh;

pub use ray::Ray;
pub use scene::{Scene, SceneGenerator};
pub use incident::{Incident, ProcessedIncident};
pub use renderer::Renderer;
pub use self::bvh::BVH;

use crate::types::Float;
use crate::vector::Vector3D;

pub mod objects;
pub mod materials;
pub mod tree;

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

    fn sample_light(&self) -> (Ray<F>, F);
}

pub fn to_world<F: Float>(w: Vector3D<F>, normal: Vector3D<F>) -> Vector3D<F> {
    if normal.x.abs() > normal.y.abs() {
        let inv_len = F::one() / (normal.x * normal.x + normal.z * normal.z).sqrt();
        let c = Vector3D::new(normal.z * inv_len, F::zero(), -normal.x * inv_len);
        let b = c.cross(normal);
        b * w.x + c * w.y + normal * w.z
    } else {
        let inv_len = F::one() / (normal.y * normal.y + normal.z * normal.z).sqrt();
        let c = Vector3D::new(F::zero(), normal.z * inv_len, -normal.y * inv_len);
        let b = c.cross(normal);
        b * w.x + c * w.y + normal * w.z
    }
}
