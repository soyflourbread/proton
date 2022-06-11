use crate::raytrace::{Incident, Ray};
use crate::types::Float;
use crate::vector::Vector3D;

#[derive(Clone)]
pub struct Triangle<F: Float> {
    v0: Vector3D<F>,
    v1: Vector3D<F>,
    v2: Vector3D<F>,

    e1: Vector3D<F>,
    e2: Vector3D<F>,

    area: F,

    normal: Vector3D<F>,
}

impl<F: Float> Triangle<F> {
    pub fn new(
        v0: Vector3D<F>,
        v1: Vector3D<F>,
        v2: Vector3D<F>,
    ) -> Self { // TODO: supposed to be counter-clockwise?
        let e1 = v1 - v0;
        let e2 = v2 - v0;

        let area = e1.cross(e2).magnitude();
        let normal = e1.cross(e2).norm();

        Self {
            v0,
            v1,
            v2,

            e1,
            e2,

            area,

            normal,
        }
    }

    pub fn vertices(&self) -> (Vector3D<F>, Vector3D<F>, Vector3D<F>) {
        (self.v0.clone(), self.v1.clone(), self.v2.clone())
    }

    pub fn area(&self) -> F {
        self.area
    }
}

impl<F: Float> Triangle<F> {
    fn hit_impl(&self, ray: &Ray<F>, inv: bool) -> Option<Incident<F>> {
        if ray.direction().dot(self.normal) > F::zero() { // Hit from inside
            return None;
        }

        let p_vec = ray.direction().cross(self.e2);
        let det = self.e1.dot(p_vec);
        let epsilon = F::from(0.01f32).unwrap();
        if det.abs() < epsilon {
            return None;
        }

        let det_inv: F = F::one() / det;
        let t_vec = ray.origin() - self.v0;
        let u = t_vec.dot(p_vec) * det_inv;
        if u < F::zero() || u > F::one() {
            return None;
        }

        let q_vec = t_vec.cross(self.e1);
        let v = ray.direction().dot(q_vec) * det_inv;
        if v < F::zero() || u + v > F::one() {
            return None;
        }

        let t_tmp = self.e2.dot(q_vec) * det_inv;
        if t_tmp < F::zero() { // Never hitting
            // println!("distance negative: {}", t_tmp.to_f64().unwrap());
            return None;
        }

        Some(
            Incident::new(
                ray.origin() + ray.direction() * t_tmp,
                self.normal,
                t_tmp,
                -ray.direction(),
                inv,
            )
        )
    }

    pub fn hit(&self, ray: &Ray<F>) -> Option<Incident<F>> {
        // TODO: Fix refraction
        if ray.direction().dot(self.normal) > F::zero() { // Hit from inside
            let inv_tri = Self::new(self.v0, self.v2, self.v1);
            return inv_tri.hit_impl(ray, true);
        }

        self.hit_impl(ray, false)
    }
}
