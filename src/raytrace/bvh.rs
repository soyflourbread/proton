use crate::raytrace::Ray;
use crate::types::Float;
use crate::vector::Vector3D;

mod backend {
    pub use bvh::{Point3, Vector3};
    pub use bvh::aabb::{Bounded, AABB};
    pub use bvh::bounding_hierarchy::BHShape;
    pub use bvh::bvh::BVH;
    pub use bvh::ray::Ray;

    use crate::types::Float;
    use crate::vector::Vector3D;

    pub fn vector_3d_from_vector3<F: Float>(vec3: Vector3) -> Vector3D<F> {
        Vector3D::new(
            F::from(vec3.x).unwrap(),
            F::from(vec3.y).unwrap(),
            F::from(vec3.z).unwrap(),
        )
    }

    pub fn vector3_from_vector_3d<F: Float>(vec3: Vector3D<F>) -> Vector3 {
        Vector3::new(
            vec3.x.to_f32().unwrap(),
            vec3.y.to_f32().unwrap(),
            vec3.z.to_f32().unwrap(),
        )
    }

    pub fn point3_from_vector_3d<F: Float>(vec3: Vector3D<F>) -> Point3 {
        Point3::new(
            vec3.x.to_f32().unwrap(),
            vec3.y.to_f32().unwrap(),
            vec3.z.to_f32().unwrap(),
        )
    }
}

#[derive(Clone)]
pub struct GenericBound<T: Clone, F: Float> {
    inner: T,

    min_pt: Vector3D<F>,
    max_pt: Vector3D<F>,

    node_index: usize,
}

impl<T: Clone, F: Float> GenericBound<T, F> {
    pub fn new(
        inner: T,
        min_pt: Vector3D<F>, max_pt: Vector3D<F>,
    ) -> Self {
        Self {
            inner,

            min_pt,
            max_pt,

            node_index: 0,
        }
    }

    pub fn get(&self) -> T {
        self.inner.clone()
    }
}

impl<T: Clone, F: Float> backend::Bounded for GenericBound<T, F> {
    fn aabb(&self) -> backend::AABB {
        backend::AABB::with_bounds(
            backend::vector3_from_vector_3d(self.min_pt),
            backend::vector3_from_vector_3d(self.max_pt),
        )
    }
}

impl<T: Clone, F: Float> backend::BHShape for GenericBound<T, F> {
    fn set_bh_node_index(&mut self, index: usize) {
        self.node_index = index;
    }

    fn bh_node_index(&self) -> usize {
        self.node_index
    }
}

#[derive(Clone)]
pub struct BVH<T: Clone, F: Float> {
    inner: backend::BVH,
    bound_vec: Vec<GenericBound<T, F>>,
}

impl<T: Clone, F: Float> BVH<T, F> {
    pub fn new(mut bound_vec: Vec<GenericBound<T, F>>) -> Self {
        let inner = backend::BVH::build(&mut bound_vec);

        Self {
            inner,
            bound_vec,
        }
    }

    pub fn inner(&self) -> &backend::BVH {
        &self.inner
    }

    pub fn bound_vec(&self) -> &Vec<GenericBound<T, F>> {
        &self.bound_vec
    }
}

impl<T: Clone, F: Float> BVH<T, F> {
    pub fn hit(&self, ray: &Ray<F>) -> Vec<&GenericBound<T, F>> {
        let _ray = backend::Ray::new(
            backend::point3_from_vector_3d(ray.origin()),
            backend::vector3_from_vector_3d(ray.direction()),
        );

        self.inner.traverse(&_ray, &self.bound_vec)
    }
}
