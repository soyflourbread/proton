use proton::raytrace::objects::{Light, Mesh, Sphere};
use proton::raytrace::{Renderer, Scene, SceneGenerator};
use proton::raytrace::objects::{Bounded, LightInteractable, LightSample, PartialBounded, RayTraceable};
use proton::vector::Vector3D;

use std::sync::Arc;
use std::thread::JoinHandle;
use image::DynamicImage;
use proton::raytrace::materials::{Diffuse, Refract};

type RF = f64;
type Vector3f = Vector3D<RF>;

struct MovieSceneGenerator {
    frame: u32,
    total_frame: u32,
}

impl MovieSceneGenerator {
    pub fn new(frame: u32, total_frame: u32) -> Self {
        Self {
            frame,
            total_frame,
        }
    }

    pub fn gen_background(&self) -> Vec<Arc<dyn RayTraceable<RF>>> {
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

        vec![
            Arc::new(floor),
            Arc::new(short_box), Arc::new(tall_box),
            Arc::new(left_wall), Arc::new(right_wall),
            Arc::new(the_ball),
            Arc::new(the_sun),
        ]
    }
}

impl MovieSceneGenerator {
    fn gen_bigger_ball_half(&self, cur_frame: u32) -> Sphere<RF> {
        let part = self.total_frame / 6;

        let radius: f64;
        let y: f64;
        if cur_frame < self.total_frame / 6 {
            let delta = cur_frame as f64 / part as f64;
            radius = 80.0 - delta * 60.0;
            y = 100.0;
        } else if cur_frame < self.total_frame / 3 {
            let delta = (cur_frame - part) as f64 / part as f64;
            radius = 20.0;
            y = 100.0 - delta * 80.0;
        } else {
            let delta = (cur_frame - part * 2) as f64 / part as f64;
            radius = 20.0 - 20.0 * delta;
            y = 20.0;
        }

        Sphere::new(
            Vector3f::new(
                400.0, y, 100.0,
            ),
            radius,
            Box::new(Refract::new(1.2)),
        )
    }

    fn gen_bigger_ball(&self) -> Sphere<RF> {
        let cur_frame: u32;
        if self.frame < self.total_frame / 2 { // before half
            cur_frame = self.frame;
        } else {
            cur_frame = self.total_frame - self.frame;
        }

        self.gen_bigger_ball_half(cur_frame)
    }
}

impl MovieSceneGenerator {
    fn gen_smaller_ball(&self) -> Sphere<RF> {
        let theta = std::f64::consts::PI * 2.0 * (self.frame as f64) / (self.total_frame as f64);

        let radius = 80.0;

        let x = radius * theta.cos() + 200.0;
        let y = 190.0;
        let z = radius * theta.sin() + 200.0;

        Sphere::new(
            Vector3f::new(x, y, z),
            20.0,
            Box::new(Refract::new(1.2)),
        )
    }
}

impl SceneGenerator<RF> for MovieSceneGenerator {
    fn gen_scene(&self) -> Scene<RF> {
        let mut objects = self.gen_background();
        let the_smaller_ball = self.gen_smaller_ball();
        let the_bigger_ball = self.gen_bigger_ball();
        objects.push(Arc::new(the_smaller_ball));
        objects.push(Arc::new(the_bigger_ball));

        Scene {
            objects
        }
    }
}

fn main() {
    let total_frame = 60;
    let renderer_thread_count = 8;
    let frame_per_thread = total_frame / renderer_thread_count;

    let mut thread_handle_vec: Vec<JoinHandle<()>> = Vec::new();

    for t in 0..renderer_thread_count {
        let frame_start = t * frame_per_thread;
        let frame_end = if t == renderer_thread_count - 1 {
            total_frame
        } else {
            (t + 1) * frame_per_thread
        };

        let handle = std::thread::spawn(move || {
            for frame in frame_start..frame_end {
                let scene_gen = Arc::new(
                    MovieSceneGenerator::new(
                        frame,
                        60,
                    )
                );
                // let renderer: Renderer<f64> = Renderer::new(256, 256, 40, scene_gen);
                let renderer: Renderer<RF> = Renderer::new(
                    2048, 2048, 40,
                    scene_gen,
                    4,
                );

                let eye_pos = Vector3f::new(278.0, 273.0, -800.0);

                let im = renderer.render(eye_pos);
                let path = format!("frames/{:02}.png", frame);
                println!("Saving frame to {}", path);
                im.save(path).unwrap();
            }
        });
        thread_handle_vec.push(handle);
    }

    for thread in thread_handle_vec {
        thread.join().expect("general error");
    }
}
