use crate::raytrace::{Incident, ProcessedIncident};
use crate::raytrace::incident::{BRDFIncident, RefractIncident, EmitIncident};
use crate::types::Float;
use crate::vector::Vector3D;

mod diffuse;
mod light;
mod refract;

pub use diffuse::Diffuse;
pub use light::Light;
pub use refract::Refract;

pub trait Material<F: Float> {
    fn interact(
        &self,
        incident: Incident<F>,
        seed: F,
    ) -> ProcessedIncident<F>;
}

pub trait Emitter<F: Float> {
    fn emit(&self, incident: &Incident<F>) -> EmitIncident<F>;
}

pub trait BRDFReflector<F: Float> {
    fn f_r(
        &self,
        coords: Vector3D<F>,
        w_i: Vector3D<F>, w_r: Vector3D<F>,
        normal: Vector3D<F>,
    ) -> Vector3D<F>;
    fn sample_reflected(
        &self,
        coords: Vector3D<F>,
        w_i: Vector3D<F>,
        normal: Vector3D<F>,
    ) -> (Vector3D<F>, F);
    fn reflect(&self, incident: &Incident<F>) -> BRDFIncident<F> {
        let coords = incident.coords();
        let w_i = incident.w_i();
        let normal = incident.normal();

        let (w_r, pdf) = self.sample_reflected(coords, w_i, normal);

        let f_r = self.f_r(coords, w_i, w_r, normal);

        let multiplier = if pdf == F::zero() {
            Vector3D::new(F::one(), F::one(), F::one())
        } else {
            f_r * w_r.dot(normal) / pdf
        };
        if multiplier.x < F::zero() {
            println!("negative multiplier");
        }

        BRDFIncident {
            f_r,
            w_r,
            pdf,

            multiplier,
        }
    }
}

pub trait Refractor<F: Float> {
    fn sample_refracted(
        &self,
        coords: Vector3D<F>,
        w_i: Vector3D<F>, normal: Vector3D<F>,
        inside: bool,
    ) -> Vector3D<F>;
    fn refract(&self, incident: &Incident<F>) -> RefractIncident<F> {
        let coords = incident.coords();
        let w_i = incident.w_i();
        let normal = incident.normal();
        let inside = incident.inside();

        let w_r = self.sample_refracted(coords, w_i, normal, inside);

        RefractIncident {
            w_r,
        }
    }
}
