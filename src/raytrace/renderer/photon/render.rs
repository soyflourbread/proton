use crate::raytrace::{Incident, Ray, RayTraceable, Scene, SceneGenerator};
use crate::types::Float;
use crate::vector::Vector3D;
use crate::raytrace::tree::TheTree;

use std::sync::Arc;
use std::thread::JoinHandle;

use indicatif::ProgressBar;
use num::traits::real::Real;

pub fn par_render<F: Float>(
    width: u32, height: u32,
    scale: F,
    eye_pos: Vector3D<F>,
    scene_gen: Arc<dyn SceneGenerator<F>>,
    thread_count: u32,
    the_tree: TheTree<F>,
    k: u32
) -> Vec<(u8, u8, u8)> {
    let mut thread_handle_vec: Vec<JoinHandle<Vec<(u8, u8, u8)>>> = Vec::new();

    for t in 0..thread_count {
        let scene_gen = scene_gen.clone();
        let the_tree = the_tree.clone();

        let handle = std::thread::spawn(move || {
            let scene = scene_gen.gen_scene();

            render_thread(
                width, height,
                scale, eye_pos,
                scene,
                t,
                thread_count,
                the_tree,
                k,
            )
        });

        thread_handle_vec.push(handle);
    }

    let mut res_vec: Vec<(u8, u8, u8)> = Vec::with_capacity(
        (width * height) as usize
    );
    for thread in thread_handle_vec {
        let mut _res_vec = thread.join().expect("general error");
        res_vec.append(&mut _res_vec);
    }

    res_vec
}

struct RenderThread<F: Float> {
    pub width: u32,
    pub height: u32,

    pub scale: F,
    pub eye_pos: Vector3D<F>,

    pub objects: Vec<Arc<dyn RayTraceable<F>>>,

    pub the_tree: TheTree<F>,
}

fn thresh_rgb<F: Float>(mut pixel: Vector3D<F>, thresh: F) -> Vector3D<F> {
    if pixel.magnitude() > thresh {
        return pixel.norm() * thresh;
    }

    pixel
}

fn render_thread<F: Float>(
    width: u32, height: u32,
    scale: F,
    eye_pos: Vector3D<F>,
    scene: Scene<F>,
    t: u32,
    thread_count: u32,
    the_tree: TheTree<F>,
    k: u32,
) -> Vec<(u8, u8, u8)> {
    let render_thread = RenderThread {
        width,
        height,
        scale,
        eye_pos,
        objects: scene.objects,
        the_tree,
    };

    let thread_rows = width / thread_count;
    let row_start = t * thread_rows;
    let row_end = if t == thread_count - 1 {
        height
    } else {
        (t + 1) * thread_rows
    };

    let mut res_vec: Vec<(u8, u8, u8)> = Vec::with_capacity(
        ((row_end - row_start) * height) as usize
    );
    for w in row_start..row_end {
        for h in 0..height {
            let (r, g, b) = render_thread.render_one(w, h, k);
            res_vec.push((r, g, b));
        }
    }

    res_vec
}

impl<F: Float> RenderThread<F> {
    fn render_one(&self, w: u32, h: u32, k: u32) -> (u8, u8, u8) {
        let width = F::from(self.width as f64).unwrap();
        let height = F::from(self.height as f64).unwrap();

        let scale = self.scale;
        let eye_pos = self.eye_pos;
        let aspect_ratio = width / height;

        let _two = F::from(2).unwrap();

        let y = (F::one() - _two * (F::from(h).unwrap() + F::sample_rand() / _two) / height) * scale;
        let x = (_two * (F::from(w).unwrap() + F::sample_rand() / _two) / width - F::one()) * aspect_ratio * scale;

        let lookat_pos: Vector3D<F> = Vector3D::new(
            eye_pos.x - x,
            eye_pos.y + y,
            eye_pos.z + F::one());
        let dir = (lookat_pos - eye_pos).norm();

        let (hit, res) = self.cast_ray(
            &Ray::new(eye_pos, dir),
            k,
        );
        // let local_res = thresh_rgb(local_res, F::from(8.0).unwrap());
        if res.x < F::zero() || res.y < F::zero() || res.z < F::zero() {
            // println!("negative pixel");
        }

        let factor = F::from((1.0 / 2.2) as f64).unwrap();
        let r = res.x.powf(factor);
        let g = res.y.powf(factor);
        let b = res.z.powf(factor);

        let r_u8 = (r.to_f64().unwrap() * 255.0) as u8;
        let g_u8 = (g.to_f64().unwrap() * 255.0) as u8;
        let b_u8 = (b.to_f64().unwrap() * 255.0) as u8;

        (r_u8, g_u8, b_u8)
    }

    fn intersect(&self, ray: &Ray<F>) -> Option<(Arc<dyn RayTraceable<F>>, Incident<F>)> {
        let mut min_distance = F::max_value();
        let mut min_incident: Option<Incident<F>> = None;
        let mut min_object: Option<Arc<dyn RayTraceable<F>>> = None;
        for object in &self.objects {
            if object.partial_hit(ray) {
                if let Some(incident) = object.hit(ray) {
                    // println!("hit object: {}, dist={}", object.name(), incident.distance().to_f64().unwrap());
                    if incident.distance() < min_distance {
                        min_distance = incident.distance();
                        min_object = Some(object.clone());
                        min_incident = Some(incident);
                    }
                }
            }
        }

        let min_object = min_object?;
        let min_incident = min_incident?;

        Some((min_object, min_incident))
    }

    fn cast_ray(&self, ray: &Ray<F>, k: u32) -> (bool, Vector3D<F>) {
        let seed = F::sample_rand();

        if let Some((object, incident)) = self.intersect(ray) {
            if let Some(diff) = object.emit() {
                return (true, diff); // Definitely hit light source
            }

            let coords = incident.coords();
            // Do k-NN on the tree
            let (photons, r) = self.the_tree.knn(coords, k);
            // println!("r: {}", r.to_f64().unwrap());

            let mut l_x: Vector3D<F> = Vector3D::zero();

            for photon in photons {
                let diff = photon.diff();
                // println!("diff: ({}, {}, {})",
                //          diff.x.to_f64().unwrap(),
                //          diff.y.to_f64().unwrap(),
                //          diff.z.to_f64().unwrap(),
                // );

                let incident = Incident::new(
                    photon.coords(),
                    incident.normal(),
                    F::zero(),
                    incident.w_i(),
                    false,
                );
                let pdf = F::PI() * r * r;
                let processed = object.interact_predetermined(
                    incident,
                    photon.w_i(), // Outgoing
                    pdf,
                    seed);

                let f_r = processed.f_r();
                if f_r == Vector3D::zero() { // Somehow
                    continue; // Pass to next photon
                }
                // println!("f_r not zero: {}", f_r.x.to_f64().unwrap());

                // TODO: Can we use multiplier directly?
                let local_irr = f_r * diff / pdf;
                l_x = local_irr + l_x;
            }

            return (true, l_x);
        }

        (false, Vector3D::zero())
    }
}
