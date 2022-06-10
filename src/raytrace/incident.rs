use crate::raytrace::Ray;
use crate::types::Float;
use crate::vector::Vector3D;

#[derive(Debug, Clone, Copy)]
pub struct Incident<F: Float> {
    coords: Vector3D<F>,
    normal: Vector3D<F>,
    distance: F,

    w_i: Vector3D<F>,
    from_inside: bool,
}

impl<F: Float> Incident<F> {
    pub fn new(coords: Vector3D<F>,
               normal: Vector3D<F>,
               distance: F,
               w_i: Vector3D<F>,
               from_inside: bool) -> Self {
        Self {
            coords,
            normal,
            distance,
            w_i,
            from_inside,
        }
    }
}

impl<F: Float> Incident<F> {
    pub fn coords(&self) -> Vector3D<F> {
        self.coords
    }

    pub fn normal(&self) -> Vector3D<F> {
        self.normal
    }

    pub fn invert_normal(&mut self) {
        self.normal = -self.normal;
    }

    pub fn distance(&self) -> F {
        self.distance
    }

    pub fn w_i(&self) -> Vector3D<F> {
        self.w_i
    }
    pub fn inside(&self) -> bool {
        self.from_inside
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BRDFIncident<F: Float> {
    pub f_r: Vector3D<F>,
    pub w_r: Vector3D<F>,
    pub pdf: F,

    pub multiplier: Vector3D<F>,
}

#[derive(Debug, Clone, Copy)]
pub struct EmitIncident<F: Float> {
    pub diff: Vector3D<F>,
}

#[derive(Debug, Clone, Copy)]
pub struct ProcessedIncident<F: Float> {
    inner: Incident<F>,

    brdf: BRDFIncident<F>,
    emit: EmitIncident<F>,
}

impl<F: Float> ProcessedIncident<F> {
    pub fn inner(&self) -> Incident<F> {
        self.inner
    }

    pub fn brdf_multiplier(&self) -> Vector3D<F> {
        self.brdf.multiplier
    }

    pub fn diff(&self) -> Vector3D<F> {
        self.emit.diff
    }

    pub fn next_ray(&self) -> Ray<F> {
        let epsilon = F::from(0.1).unwrap();

        Ray::new(
            self.inner.coords() + self.brdf.w_r * epsilon,
            self.brdf.w_r,
        )
    }
}

impl<F: Float> ProcessedIncident<F> {
    pub fn new(inner: Incident<F>,
               brdf: BRDFIncident<F>,
               emit: EmitIncident<F>) -> Self {
        Self {
            inner,
            brdf,
            emit,
        }
    }

    pub fn set_emit(&mut self, emit: EmitIncident<F>) {
        self.emit = emit;
    }
}
