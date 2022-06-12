use crate::raytrace::{Incident, ProcessedIncident, to_world};
use crate::raytrace::materials::{BRDFReflector, Material};
use crate::types::Float;
use crate::vector::Vector3D;

#[derive(Debug, Clone, Copy)]
pub struct Diffuse<F: Float> {
    kd: Vector3D<F>,
}

impl<F: Float> Diffuse<F> {
    pub fn new(kd: Vector3D<F>) -> Self {
        Self {
            kd
        }
    }
}

impl<F: Float> BRDFReflector<F> for Diffuse<F> {
    fn f_r(
        &self,
        coords: Vector3D<F>,
        w_i: Vector3D<F>,
        w_r: Vector3D<F>,
        normal: Vector3D<F>,
        seed: F,
    ) -> Vector3D<F> {
        let cos_alpha = normal.dot(w_r);
        if cos_alpha > F::zero() {
            return self.kd * F::FRAC_1_PI();
        }
        Vector3D::zero()
    }

    fn sample_reflected(
        &self,
        coords: Vector3D<F>,
        w_i: Vector3D<F>,
        normal: Vector3D<F>,
        seed: F,
    ) -> (Vector3D<F>, F) {
        let w_r = self.sample(normal);
        let pdf = self.pdf(w_r, normal);

        (w_r, pdf)
    }
}

impl<F: Float> Diffuse<F> {
    fn sample(&self, normal: Vector3D<F>) -> Vector3D<F> {
        // let x_1 = F::sample_rand() * F::from(0.2).unwrap() + F::from(0.05).unwrap();
        // let x_2 = F::sample_rand() * F::from(0.2).unwrap() + F::from(0.05).unwrap();
        let x_1 = F::sample_rand();
        let x_2 = F::sample_rand();
        let z = F::one().abs_sub(x_1 * F::from(2).unwrap());
        let r = (F::one() - z * z).sqrt();
        let phi: F = F::from(2).unwrap() * F::PI() * x_2;

        let local_w_r = Vector3D::new(r * phi.cos(), r * phi.sin(), z);

        let w_r = to_world::<F>(local_w_r, normal);
        if w_r.dot(normal) < F::zero() {
            // println!("w_r is faulty, local=({},{},{}),\n w_r=({},{},{}),\n norm=({},{},{})",
            //          local_w_r.x.to_f64().unwrap(),
            //          local_w_r.y.to_f64().unwrap(),
            //          local_w_r.z.to_f64().unwrap(),
            //          w_r.x.to_f64().unwrap(),
            //          w_r.y.to_f64().unwrap(),
            //          w_r.z.to_f64().unwrap(),
            //          normal.x.to_f64().unwrap(),
            //          normal.y.to_f64().unwrap(),
            //          normal.z.to_f64().unwrap(),
            // );
        }

        w_r
    }

    fn pdf(&self, w_r: Vector3D<F>, normal: Vector3D<F>) -> F {
        if w_r.dot(normal) > F::zero() {
            return F::from(0.5 as f64).unwrap() * F::FRAC_1_PI();
        }
        return F::zero();
    }
}

impl<F: Float> Material<F> for Diffuse<F> {
    fn interact(
        &self,
        incident: Incident<F>,
        seed: F,
    ) -> ProcessedIncident<F> {
        let coords = incident.coords();
        let w_i = incident.w_i();
        let normal = incident.normal();

        let (w_r, pdf) = self.sample_reflected(coords, w_i, normal, seed);

        self.interact_predetermined(incident, w_r, pdf, seed)
    }

    fn interact_predetermined(
        &self,
        incident: Incident<F>,
        w_r: Vector3D<F>,
        pdf: F,
        seed: F) -> ProcessedIncident<F> {
        let brdf = self.reflect_predetermined(&incident, w_r, pdf, seed);

        ProcessedIncident::from_brdf(
            incident,
            brdf,
        )
    }

    fn focus(&self) -> bool {
        false
    }
}
