use crate::raytrace::{Bounded, BVH, Incident, LightInteractable, PartialBounded, ProcessedIncident, Ray, RayTraceable};
use crate::raytrace::bvh::GenericBound;
use crate::raytrace::materials::Material;

use crate::types::Float;
use crate::vector::Vector3D;

use super::base;

pub struct Mesh<F: Float> {
    name: String,

    bound: BoundImpl<F>,
    partial_bound: PartialBoundImpl<F>,

    material: Box<dyn Material<F>>,
}

impl<F: Float> Mesh<F> {
    pub fn new(source: String, material: Box<dyn Material<F>>) -> Self {
        let name = source.clone();

        let inner = base::Mesh::new(source);

        let partial_bound = PartialBoundImpl::new(&inner);
        let bound = BoundImpl::new(inner);

        Self {
            name,

            bound,
            partial_bound,

            material,
        }
    }
}

#[derive(Clone)]
struct BoundImpl<F: Float> {
    inner: base::Mesh<F>,

    bvh: BVH<usize, F>,
}

impl<F: Float> BoundImpl<F> {
    pub fn new(inner: base::Mesh<F>) -> Self {
        let mut bound_vec = Vec::new();
        for i in 0..inner.triangles().len() {
            let (v0, v1, v2) = inner.triangles()[i].vertices();

            let epsilon = F::from(0.1).unwrap();
            let min_pt = Vector3D::new(
                v0.x.min(v1.x.min(v2.x)),
                v0.y.min(v1.y.min(v2.y)),
                v0.z.min(v1.z.min(v2.z)),
            ) - epsilon;
            let max_pt = Vector3D::new(
                v0.x.max(v1.x.max(v2.x)),
                v0.y.max(v1.y.max(v2.y)),
                v0.z.max(v1.z.max(v2.z)),
            ) + epsilon;

            let bound = GenericBound::new(
                i,
                min_pt, max_pt,
            );

            bound_vec.push(bound);
        }

        let bvh = BVH::new(bound_vec);

        Self {
            inner,
            bvh,
        }
    }
}

impl<F: Float> BoundImpl<F> {
    pub fn hit(&self, ray: &Ray<F>) -> Option<Incident<F>> {
        let hit_bound_vec = self.bvh.hit(ray);
        if hit_bound_vec.is_empty() {
            return None;
        }

        // println!("percentage: {}/{}", hit_bound_vec.len(), self.bvh.bound_vec().len());

        let mut min_incident: Option<Incident<F>> = None;
        let mut min_distance = F::max_value();
        for bound in hit_bound_vec {
            let id = bound.get();
            if let Some(incident) = self.inner.triangles()[id].hit(ray) {
                if incident.distance() < min_distance {
                    min_distance = incident.distance();
                    min_incident = Some(incident);
                }
            }
        }

        min_incident
    }
}

#[derive(Debug, Clone, Copy)]
struct PartialBoundImpl<F: Float> {
    min_pt: Vector3D<F>,
    max_pt: Vector3D<F>,
}

impl<F: Float> PartialBoundImpl<F> {
    pub fn new(inner: &base::Mesh<F>) -> Self {
        let (min_pt, max_pt) = inner.extreme_pts();

        Self {
            min_pt,
            max_pt,
        }
    }
}

impl<F: Float> PartialBoundImpl<F> {
    pub fn partial_hit(&self, ray: &Ray<F>) -> bool {
        let origin = ray.origin();
        let w_i = ray.direction();
        let inv_dir = Vector3D::new(
            F::one() / w_i.x,
            F::one() / w_i.y,
            F::one() / w_i.z,
        );

        let (tx_min, tx_max) = if w_i.x >= F::zero() {
            (
                (self.min_pt.x - origin.x) * inv_dir.x,
                (self.max_pt.x - origin.x) * inv_dir.x,
            )
        } else {
            (
                (self.max_pt.x - origin.x) * inv_dir.x,
                (self.min_pt.x - origin.x) * inv_dir.x,
            )
        };
        let (ty_min, ty_max) = if w_i.y >= F::zero() {
            (
                (self.min_pt.y - origin.y) * inv_dir.y,
                (self.max_pt.y - origin.y) * inv_dir.y,
            )
        } else {
            (
                (self.max_pt.y - origin.y) * inv_dir.y,
                (self.min_pt.y - origin.y) * inv_dir.y,
            )
        };
        let (tz_min, tz_max) = if w_i.z >= F::zero() {
            (
                (self.min_pt.z - origin.z) * inv_dir.z,
                (self.max_pt.z - origin.z) * inv_dir.z,
            )
        } else {
            (
                (self.max_pt.z - origin.z) * inv_dir.z,
                (self.min_pt.z - origin.z) * inv_dir.z,
            )
        };

        let t_enter = tx_min.max(ty_min.max(tz_min));
        let t_exit = tx_max.min(ty_max.min(tz_max));

        let epsilon = F::from(1e-4f32).unwrap();
        t_enter < t_exit + epsilon && t_exit > F::zero()
    }
}

impl<F: Float> Bounded<F> for Mesh<F> {
    fn hit(&self, ray: &Ray<F>) -> Option<Incident<F>> {
        self.bound.hit(ray)
    }
}

impl<F: Float> PartialBounded<F> for Mesh<F> {
    fn partial_hit(&self, ray: &Ray<F>) -> bool {
        self.partial_bound.partial_hit(ray)
    }
}

impl<F: Float> LightInteractable<F> for Mesh<F> {
    fn interact(
        &self,
        incident: Incident<F>,
        seed: F,
    ) -> ProcessedIncident<F> {
        self.material.interact(incident, seed)
    }
}

impl<F: Float> RayTraceable<F> for Mesh<F> {
    fn name(&self) -> String {
        self.name.clone()
    }
}

