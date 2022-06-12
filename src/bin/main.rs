use proton::raytrace::objects::{Light, Mesh, Sphere};
use proton::raytrace::{Bounded, Ray, RayTraceable, Renderer, Scene, SceneGenerator};
use proton::vector::Vector3D;

use std::sync::Arc;
use proton::raytrace::materials::{Diffuse, Refract};

type RF = f64;
type Vector3f = Vector3D<RF>;

struct PracticalSceneGenerator {}

impl SceneGenerator<RF> for PracticalSceneGenerator {
    fn gen_scene(&self) -> Scene<RF> {
        let floor = Mesh::new(
            "cornellbox/floor.obj".to_string(),
            Box::new(Diffuse::new(
                Vector3f::new(0.725, 0.71, 0.68),
            )),
        );

        let short_box = Mesh::new(
            "cornellbox/shortbox.obj".to_string(),
            Box::new(Diffuse::new(
                Vector3f::new(0.725, 0.71, 0.68),
            ))
        );
        // let short_box = Mesh::new(
        //     "cornellbox/shortbox.obj".to_string(),
        //     Box::new(Refract::new(1.2)),
        // );
        let tall_box = Mesh::new(
            "cornellbox/tallbox.obj".to_string(),
            Box::new(Diffuse::new(
                Vector3f::new(0.725, 0.71, 0.68),
            )),
        );

        let left_wall = Mesh::new(
            "cornellbox/left.obj".to_string(),
            Box::new(Diffuse::new(
                Vector3f::new(0.63, 0.065, 0.05),
            )),
        );
        let right_wall = Mesh::new(
            "cornellbox/right.obj".to_string(),
            Box::new(Diffuse::new(
                Vector3f::new(0.14, 0.45, 0.091),
            )),
        );

        let light = Light::new(
            Box::new(Mesh::new(
                "cornellbox/light.obj".to_string(),
                Box::new(Diffuse::new(
                    Vector3f::new(0.14, 0.45, 0.091),
                )),
            )),
            Vector3f::new(0.747 + 0.058, 0.747 + 0.258, 0.747) * 8.0
                + Vector3f::new(0.740 + 0.287, 0.740 + 0.160, 0.740) * 15.6
                + Vector3f::new(0.737 + 0.642, 0.737 + 0.159, 0.737) * 18.4,
        );

        // let the_ball = Sphere::new(
        //     Vector3f::new(200.0, 220.0, 200.0),
        //     40.0,
        //     Box::new(Diffuse::new(
        //         Vector3f::new(0.725, 0.71, 0.68),
        //     )),
        // );
        let the_ball = Sphere::new(
            Vector3f::new(180.0, 220.0, 200.0),
            40.0,
            Box::new(Refract::new(1.2)),
        );

        Scene {
            objects: vec![
                Arc::new(floor),
                Arc::new(short_box), Arc::new(tall_box),
                Arc::new(left_wall), Arc::new(right_wall),
                Arc::new(the_ball),
                Arc::new(light),
            ]
        }
    }
}

fn main() {
    let scene_gen = Arc::new(PracticalSceneGenerator {});
    // let renderer: Renderer<f64> = Renderer::new(256, 256, 40, scene_gen);
    let renderer: Renderer<RF> = Renderer::new(1024, 1024, 40, scene_gen);

    let eye_pos = Vector3f::new(278.0, 273.0, -800.0);

    println!("Start rendering...");
    let start = std::time::Instant::now();
    // renderer.render(eye_pos);
    renderer.render_photon(eye_pos);
    let duration = start.elapsed();
    println!("Time elapsed in render() is: {:?}", duration);
}
