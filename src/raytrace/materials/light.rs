use crate::raytrace::{Incident, ProcessedIncident};
use crate::raytrace::incident::{BRDFIncident, EmitIncident};
use crate::raytrace::materials::{Emitter, Material};
use crate::types::Float;
use crate::vector::Vector3D;

#[derive(Debug, Clone, Copy)]
pub struct Light<F: Float> {
    diff: Vector3D<F>,
}

impl<F: Float> Light<F> {
    pub fn new(diff: Vector3D<F>) -> Self {
        Self {
            diff
        }
    }
}

impl<F: Float> Emitter<F> for Light<F> {
    fn emit(&self, incident: &Incident<F>) -> EmitIncident<F> {
        EmitIncident {
            diff: self.diff,
        }
    }
}

impl<F: Float> Material<F> for Light<F> {
    fn interact(&self, incident: Incident<F>) -> ProcessedIncident<F> {
        ProcessedIncident::from_brdf(
            incident,
            BRDFIncident {
                f_r: Vector3D::one(),
                w_r: Vector3D::one(),
                pdf: F::one(),
                multiplier: Vector3D::one(),
            },
            EmitIncident {
                diff: self.diff,
            },
        )
    }
}
