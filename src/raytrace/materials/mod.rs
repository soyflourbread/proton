use crate::raytrace::{Incident, ProcessedIncident};
use crate::raytrace::incident::{BRDFIncident, RefractIncident};
use crate::types::Float;
use crate::vector::Vector3D;

mod diffuse;
mod refract;

pub use diffuse::Diffuse;
pub use refract::Refract;

pub trait Material<F: Float> {
    fn interact(
        &self,
        incident: Incident<F>,
        seed: F,
    ) -> ProcessedIncident<F>;
    fn interact_predetermined(
        &self,
        incident: Incident<F>,
        w_r: Vector3D<F>,
        pdf: F,
        seed: F,
    ) -> ProcessedIncident<F>;

    fn focus(&self) -> bool;
}

pub trait BRDFReflector<F: Float> {
    fn f_r(
        &self,
        coords: Vector3D<F>,
        w_i: Vector3D<F>, w_r: Vector3D<F>,
        normal: Vector3D<F>,
        seed: F,
    ) -> Vector3D<F>;
    fn sample_reflected(
        &self,
        coords: Vector3D<F>,
        w_i: Vector3D<F>,
        normal: Vector3D<F>,
        seed: F,
    ) -> (Vector3D<F>, F);
    fn reflect_predetermined(
        &self,
        incident: &Incident<F>,
        w_r: Vector3D<F>,
        pdf: F,
        seed: F) -> BRDFIncident<F> {
        let coords = incident.coords();
        let w_i = incident.w_i();
        let normal = incident.normal();

        let f_r = self.f_r(coords, w_i, w_r, normal, seed);
        let mut multiplier = if pdf == F::zero() {
            Vector3D::new(F::one(), F::one(), F::one())
        } else {
            f_r * w_r.dot(normal) / pdf
        };
        let rev_f_r = self.f_r(coords, w_r, w_i, normal, seed);
        let mut rev_multiplier = if pdf == F::zero() {
            Vector3D::new(F::one(), F::one(), F::one())
        } else {
            rev_f_r * w_i.dot(normal) / pdf
        };

        BRDFIncident {
            f_r,
            w_r,
            pdf,

            multiplier,
            rev_multiplier,
        }
    }
}

pub trait Refractor<F: Float> {
    fn sample_refracted(
        &self,
        coords: Vector3D<F>,
        w_i: Vector3D<F>, normal: Vector3D<F>,
        inside: bool,
        seed: F,
    ) -> (bool, Vector3D<F>);
    fn refract(&self, incident: &Incident<F>, seed: F) -> RefractIncident<F> {
        let coords = incident.coords();
        let w_i = incident.w_i();
        let normal = incident.normal();
        let inside = incident.inside();

        let (flip, w_r) = self.sample_refracted(coords, w_i, normal, inside, seed);

        RefractIncident {
            w_r,
            flip,
        }
    }
}
