use crate::objects::Triangle;
use crate::types::Float;
use crate::vector::Vector3D;

mod backend {
    pub use tri_mesh::mesh::Mesh;
    pub use tri_mesh::mesh_builder::MeshBuilder;
    pub use tri_mesh::prelude::Vec3;
    pub use tri_mesh::prelude::Intersection;
    pub use tri_mesh::prelude::Primitive;

    use crate::types::Float;
    use crate::vector::Vector3D;

    pub fn vector_3d_from_vec3<F: Float>(vec3: Vec3) -> Vector3D<F> {
        Vector3D::new(
            F::from(vec3.x).unwrap(),
            F::from(vec3.y).unwrap(),
            F::from(vec3.z).unwrap(),
        )
    }
}

#[derive(Clone)]
pub struct Mesh<F: Float> {
    triangles: Vec<Triangle<F>>,

    area: F,

    min_vert: Vector3D<F>,
    max_vert: Vector3D<F>,
}

impl<F: Float> Mesh<F> {
    pub fn new(source: String) -> Self {
        let obj_source = std::fs::read_to_string(source)
            .expect("Something went wrong reading the file");

        let builder = backend::MeshBuilder::new();
        let mesh = builder.with_obj(obj_source).build().unwrap();

        let mut min_vert = Vector3D::max_value();
        let mut max_vert = Vector3D::min_value();

        let mut area = F::zero();

        let mut triangles = Vec::new();
        for face_id in mesh.face_iter() {
            let (v0, v1, v2) = mesh.face_positions(face_id);
            let v0 = backend::vector_3d_from_vec3(v0);
            let v1 = backend::vector_3d_from_vec3(v1);
            let v2 = backend::vector_3d_from_vec3(v2);

            min_vert = min_vert.min(v0.min(v1.min(v2)));
            max_vert = max_vert.max(v0.max(v1.max(v2)));

            let triangle = Triangle::new(
                v0, v1, v2,
            );
            area = area + triangle.area();
            triangles.push(triangle);
        }

        Self {
            triangles,

            area,

            min_vert,
            max_vert,
        }
    }

    pub fn triangles(&self) -> &Vec<Triangle<F>> {
        &self.triangles
    }

    pub fn area(&self) -> F {
        self.area
    }
}

impl<F: Float> Mesh<F> {
    pub fn extreme_pts(&self) -> (Vector3D<F>, Vector3D<F>) {
        (
            self.min_vert,
            self.max_vert,
        )
    }
}
