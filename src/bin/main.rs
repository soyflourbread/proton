use proton::raytrace::objects::{Light, Mesh, Sphere};
use proton::raytrace::{Renderer, Scene, SceneGenerator};
use proton::raytrace::objects::{Bounded, LightInteractable, LightSample, PartialBounded, RayTraceable};
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
            )),
        );
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

        let the_ball = Sphere::new(
            Vector3f::new(200.0, 240.0, 200.0),
            60.0,
            Box::new(Refract::new(1.2)),
        );
        let the_smaller_ball = Sphere::new(
            Vector3f::new(120.0, 190.0, 200.0),
            20.0,
            Box::new(Refract::new(1.2)),
        );
        let the_bigger_ball = Sphere::new(
            Vector3f::new(400.0, 100.0, 100.0),
            80.0,
            Box::new(Refract::new(1.2)),
        );

        let the_sun = Light::new(
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
        // let the_sum_diff = Vector3f::new(0.747 + 0.058, 0.747 + 0.258, 0.747) * 8.0
        //     + Vector3f::new(0.740 + 0.287, 0.740 + 0.160, 0.740) * 15.6
        //     + Vector3f::new(0.737 + 0.642, 0.737 + 0.159, 0.737) * 18.4;
        // let the_sun = Light::new(
        //     Box::new(Sphere::new(
        //         Vector3f::new(278.0, 480.0, 279.5),
        //         40.0,
        //         Box::new(Diffuse::new(
        //             Vector3f::new(0.14, 0.45, 0.091),
        //         ))
        //     )),
        //     the_sum_diff * 2.0,
        // );

        Scene {
            objects: vec![
                Arc::new(floor),
                Arc::new(short_box), Arc::new(tall_box),
                Arc::new(left_wall), Arc::new(right_wall),
                Arc::new(the_ball), Arc::new(the_smaller_ball), Arc::new(the_bigger_ball),
                Arc::new(the_sun),
            ]
        }
    }
}

fn main() {
    let scene_gen = Arc::new(PracticalSceneGenerator {});
    // let renderer: Renderer<f64> = Renderer::new(256, 256, 40, scene_gen);
    let renderer: Renderer<RF> = Renderer::new(2048, 2048, 40, scene_gen);

    let eye_pos = Vector3f::new(278.0, 273.0, -800.0);

    let im = renderer.render(eye_pos);

    im.save("binary.png");
}
