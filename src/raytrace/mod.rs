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

pub mod objects;
pub mod materials;
pub mod tree;

pub trait LightInteractable<F: Float> {
    fn interact(
        &self,
        incident: Incident<F>,
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
}
