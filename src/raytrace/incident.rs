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

    emit: Vector3D<F>,
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
            emit: Vector3D::zero(),
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

// Retract might be full reflection
#[derive(Debug, Clone, Copy)]
pub struct RefractIncident<F: Float> {
    pub w_r: Vector3D<F>,
}

#[derive(Debug, Clone, Copy)]
pub enum InteractIncident<F: Float> {
    Reflect(BRDFIncident<F>),
    Refract(RefractIncident<F>),
}

#[derive(Debug, Clone, Copy)]
pub struct ProcessedIncident<F: Float> {
    inner: Incident<F>,

    interact: InteractIncident<F>,
}

impl<F: Float> ProcessedIncident<F> {
    pub fn inner(&self) -> Incident<F> {
        self.inner
    }

    pub fn multiplier(&self) -> Vector3D<F> {
        match self.interact {
            InteractIncident::Reflect(brdf) => {
                brdf.multiplier
            }
            InteractIncident::Refract(_) => {
                Vector3D::one() // TODO: does russian roulette ensure this?
            }
        }
    }

    pub fn next_ray(&self) -> Ray<F> {
        let epsilon = F::from(0.1).unwrap();

        match self.interact {
            InteractIncident::Reflect(brdf) => {
                Ray::new(
                    self.inner.coords() + brdf.w_r * epsilon,
                    brdf.w_r,
                )
            }
            InteractIncident::Refract(retract) => {
                Ray::new(
                    self.inner.coords() + retract.w_r * epsilon,
                    retract.w_r,
                )
            }
        }
    }
}

impl<F: Float> ProcessedIncident<F> {
    pub fn from_brdf(
        inner: Incident<F>,
        brdf: BRDFIncident<F>,
    ) -> Self {
        let interact = InteractIncident::Reflect(brdf);

        Self {
            inner,
            interact,
        }
    }

    pub fn from_refract(
        inner: Incident<F>,
        refract: RefractIncident<F>,
    ) -> Self {
        let interact = InteractIncident::Refract(refract);

        Self {
            inner,
            interact,
        }
    }
}
