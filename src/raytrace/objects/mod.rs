mod sphere;
mod mesh;
mod light;

pub use sphere::Sphere;
pub use mesh::Mesh;
pub use light::Light;

use crate::objects as base;

#[cfg(test)]
mod tests {
    use crate::raytrace::{Bounded, Ray};
    use crate::raytrace::materials::Diffuse;
    use crate::raytrace::objects::Sphere;
    use crate::vector::Vector3D;

    #[test]
    fn sphere_hit() {
        type RF = f64;
        type Vector3f = Vector3D<RF>;

        let sphere = Sphere::new(
            Vector3f::new(0.0, 0.0, 0.0),
            8.0,
            Box::new(Diffuse::new(
                Vector3f::new(0.725, 0.71, 0.68),
            )),
        );

        let ray = Ray::from_inside(
            Vector3f::new(-1.0, 1.0, 0.0),
            Vector3f::new(1.0, 0.0, 0.0),
        );
        if let Some(incident) = sphere.hit(&ray) {
            println!("hit: ({}, {}, {})",
                     incident.coords().x,
                     incident.coords().y,
                     incident.coords().z,
            )
        } else {
            println!("no hit");
        }
    }
}
