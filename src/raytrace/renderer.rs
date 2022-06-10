use crate::raytrace::{Incident, Ray, RayTraceable, Scene, SceneGenerator};
use crate::types::Float;
use crate::vector::Vector3D;

use std::sync::Arc;
use std::thread::JoinHandle;

use image::GenericImage;
use indicatif::ProgressBar;
use num::traits::real::Real;

#[derive(Debug, Clone, Copy)]
struct Dimensions {
    pub width: u32,
    pub height: u32,
}

pub struct Renderer<F: Float> {
    dims: Dimensions,

    fov: u32,
    spp: u32,

    rr: F,

    scene_gen: Arc<dyn SceneGenerator<F>>,

    thread_count: u32,

    progress_bar: ProgressBar,
}

impl<F: Float> Renderer<F> {
    pub fn new(
        width: u32, height: u32, fov: u32,
        scene_gen: Arc<dyn SceneGenerator<F>>) -> Self {
        Self {
            dims: Dimensions {
                width,
                height,
            },
            fov,
            spp: 1024,
            rr: F::from(0.8 as f64).unwrap(),
            scene_gen,
            thread_count: 24,
            progress_bar: ProgressBar::new((width * height) as u64),
        }
    }
}

impl<F: Float> Renderer<F> {
    pub fn render(&self, eye_pos: Vector3D<F>) {
        let fov = F::from(self.fov).unwrap();
        let scale: F = (fov * F::from(0.5 as f64).unwrap()).to_radians().tan();

        let mut im = image::DynamicImage::new_rgb8(self.dims.width, self.dims.height);

        let res_vec = par_render(
            self.dims.width, self.dims.height,
            self.rr,
            scale,
            eye_pos,
            self.scene_gen.clone(),
            self.spp,
            self.thread_count,
            self.progress_bar.clone(),
        );

        for w in 0..self.dims.width {
            for h in 0..self.dims.height {
                let (r, g, b) = res_vec[(w * self.dims.height + h) as usize];
                im.put_pixel(w, h, image::Rgba::from([r, g, b, 255]));
            }
        }

        self.progress_bar.finish();
        im.save("binary.png").unwrap();
    }
}

struct RenderThread<F: Float> {
    pub width: u32,
    pub height: u32,

    pub rr: F,

    pub scale: F,
    pub eye_pos: Vector3D<F>,

    pub objects: Vec<Arc<dyn RayTraceable<F>>>,
}

fn thresh_rgb<F: Float>(mut pixel: Vector3D<F>, thresh: F) -> Vector3D<F> {
    if pixel.magnitude() > thresh {
        return pixel.norm() * thresh;
    }

    pixel
}

fn render_thread<F: Float>(
    width: u32, height: u32,
    rr: F,
    scale: F,
    eye_pos: Vector3D<F>,
    scene: Scene<F>,
    spp: u32,
    t: u32,
    thread_count: u32,
    progress_bar: ProgressBar,
) -> Vec<(u8, u8, u8)> {
    let render_thread = RenderThread {
        width,
        height,
        rr,
        scale,
        eye_pos,
        objects: scene.objects,
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
            let (r, g, b) = render_thread.render_one(w, h, spp);
            res_vec.push((r, g, b));
            progress_bar.inc(1);
        }
    }

    res_vec
}

fn par_render<F: Float>(
    width: u32, height: u32,
    rr: F,
    scale: F,
    eye_pos: Vector3D<F>,
    scene_gen: Arc<dyn SceneGenerator<F>>,
    spp: u32,
    thread_count: u32,
    progress_bar: ProgressBar,
) -> Vec<(u8, u8, u8)> {
    let mut thread_handle_vec: Vec<JoinHandle<Vec<(u8, u8, u8)>>> = Vec::new();

    for t in 0..thread_count {
        let progress_bar = progress_bar.clone();
        let scene_gen = scene_gen.clone();

        let handle = std::thread::spawn(move || {
            let scene = scene_gen.gen_scene();

            render_thread(
                width, height,
                rr, scale, eye_pos,
                scene,
                spp,
                t,
                thread_count,
                progress_bar,
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

impl<F: Float> RenderThread<F> {
    fn render_one(&self, w: u32, h: u32, spp: u32) -> (u8, u8, u8) {
        let width = F::from(self.width as f64).unwrap();
        let height = F::from(self.height as f64).unwrap();

        let scale = self.scale;
        let eye_pos = self.eye_pos;
        let aspect_ratio = width / height;

        let _two = F::from(2).unwrap();

        let mut res: Vector3D<F> = Vector3D::zero();
        let _1_spp = F::one() / F::from(spp).unwrap();

        for _ in 0..spp {
            let y = (F::one() - _two * (F::from(h).unwrap() + F::sample_rand() / _two) / height) * scale;
            let x = (_two * (F::from(w).unwrap() + F::sample_rand() / _two) / width - F::one()) * aspect_ratio * scale;

            let lookat_pos: Vector3D<F> = Vector3D::new(
                eye_pos.x - x,
                eye_pos.y + y,
                eye_pos.z + F::one());
            let dir = (lookat_pos - eye_pos).norm();

            let (hit, local_res) = self.cast_ray(
                &Ray::new(eye_pos, dir)
            );
            // let local_res = thresh_rgb(local_res, F::from(8.0).unwrap());
            if local_res.x < F::zero() || local_res.y < F::zero() || local_res.z < F::zero() {
                // println!("negative pixel");
            } else {
                res += local_res * _1_spp;
            }
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

    fn cast_ray(&self, ray: &Ray<F>) -> (bool, Vector3D<F>) {
        if let Some((object, incident)) = self.intersect(ray) {
            let processed = object.interact(incident);
            if processed.diff() != Vector3D::zero() {
                return (true, processed.diff()); // Definitely hit light source
            }

            let mut l_x: Vector3D<F> = Vector3D::zero();

            if F::sample_rand() < self.rr {
                let next_ray = processed.next_ray();
                let (next_hit, next_incident) = self.cast_ray(&next_ray);
                if next_hit { // TODO: Fix direct lighting
                    l_x += processed.multiplier() * next_incident / self.rr;
                }
            }

            return (true, l_x);
        }

        (false, Vector3D::zero())
    }
}
