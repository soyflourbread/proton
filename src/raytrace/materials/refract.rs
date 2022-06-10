use crate::raytrace::{Incident, ProcessedIncident};
use crate::raytrace::incident::{BRDFIncident, EmitIncident};
use crate::raytrace::materials::{Refractor, Material};
use crate::types::Float;
use crate::vector::Vector3D;

pub struct Refract<F: Float> {
    index_of_coin: F,
}

impl<F: Float> Refract<F> {
    pub fn new(index_of_coin: F) -> Self {
        Self {
            index_of_coin
        }
    }
}

fn reflect<F: Float>(v: Vector3D<F>, n: Vector3D<F>) -> Vector3D<F> {
    let _two = F::from(2u32).unwrap();

    v - n * (_two * v.dot(n))
}

fn refract<F: Float>(uv: Vector3D<F>, n: Vector3D<F>, etai_over_etat: F) -> Vector3D<F> {
    let cos_theta = ((-uv).dot(n)).min(F::one());
    let r_out_perp = (uv + n * cos_theta) * etai_over_etat;
    let r_out_parallel = n * (-F::one() * (F::one() - r_out_perp.dot(r_out_perp)).abs().sqrt());
    r_out_perp + r_out_parallel
}

impl<F: Float> Refractor<F> for Refract<F> {
    fn sample_refracted(&self, coords: Vector3D<F>, w_i: Vector3D<F>, mut normal: Vector3D<F>, inside: bool) -> (Vector3D<F>, F) {
        let refraction_ratio = if inside { // Hit from inside
            self.index_of_coin
        } else { // Hit from outside
            F::one() / self.index_of_coin
        };

        let cos_theta = w_i.dot(normal).min(F::one());
        let sin_theta = (F::one() - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > F::one();
        if cannot_refract {
            let reflected = reflect(-w_i, normal); // Is it correct?
            (reflected, F::one())
        } else {
            let refracted = refract(-w_i, normal, refraction_ratio);
            (refracted, F::one())
        }
    }
}

impl<F: Float> Material<F> for Refract<F> {
    fn interact(&self, incident: Incident<F>) -> ProcessedIncident<F> {
        let brdf = self.refract(&incident);

        ProcessedIncident::new(
            incident,
            brdf,
            EmitIncident {
                diff: Vector3D::zero(),
            },
        )
    }
}