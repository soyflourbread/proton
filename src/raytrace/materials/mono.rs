use crate::raytrace::{Incident, ProcessedIncident};
use crate::raytrace::incident::{BRDFIncident, EmitIncident};
use crate::raytrace::materials::{BRDFReflector, Refractor, Emitter, Material};
use crate::types::Float;
use crate::vector::Vector3D;

pub struct Mono<F: Float> {
    // An object must be able to reflect
    reflector: Box<dyn BRDFReflector<F>>,
    refractor: Option<Box<dyn Refractor<F>>>,

    refract_prob: F,
}

impl<F: Float> Mono<F> {
    pub fn from_both(
        reflector: Box<dyn BRDFReflector<F>>,
        refractor: Box<dyn Refractor<F>>,
        refract_prob: F,
    ) -> Self {
        Self {
            reflector,
            refractor: Some(refractor),

            refract_prob,
        }
    }

    pub fn from_reflector(
        reflector: Box<dyn BRDFReflector<F>>
    ) -> Self {
        Self {
            reflector,
            refractor: None,

            refract_prob: F::zero(),
        }
    }
}

impl<F: Float> Material<F> for Mono<F> {
    fn interact(&self, incident: Incident<F>) -> ProcessedIncident<F> {
        // let brdf = if let Some(refractor) = &self.refractor {
        //     // TODO: Only reflect when light hit from outer surface
        //     if incident.w_i().dot(incident.normal()) < F::zero() {
        //         refractor.refract(&incident)
        //     } else if F::sample_rand() < self.refract_prob {
        //         refractor.refract(&incident)
        //     } else {
        //         self.reflector.reflect(&incident)
        //     }
        // } else {
        //     self.reflector.reflect(&incident)
        // };
        let brdf = self.reflector.reflect(&incident);

        ProcessedIncident::new(
            incident,
            brdf,
            EmitIncident {
                diff: Vector3D::zero(),
            },
        )
    }
}
