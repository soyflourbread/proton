use crate::raytrace::{Incident, ProcessedIncident, Ray, Scene, SceneGenerator};
use crate::raytrace::objects::RayTraceable;

use crate::types::Float;
use crate::vector::Vector3D;
use crate::raytrace::renderer::Dimensions;

use std::sync::Arc;
use std::thread::JoinHandle;

use indicatif::ProgressBar;
use num::traits::real::Real;
use crate::raytrace::tree::{Photon, TheTree};

pub struct SimpleRenderer<F: Float> {
    dims: Dimensions,

    fov: u32,
    spp: u32,

    rr: F,

    scene_gen: Arc<dyn SceneGenerator<F>>,

    thread_count: u32,

    the_tree: TheTree<F>,
    k: u32,
    max_radius: F,

    progress_bar: ProgressBar,
}

impl<F: Float> SimpleRenderer<F> {
    pub fn new(
        dims: Dimensions,
        fov: u32,
        spp: u32,
        rr: F,
        scene_gen: Arc<dyn SceneGenerator<F>>,
        thread_count: u32,
        the_tree: TheTree<F>,
        k: u32,
        max_radius: F,
        progress_bar: ProgressBar,
    ) -> Self {
        Self {
            dims,
            fov,
            spp,
            rr,
            scene_gen,
            thread_count,
            the_tree,
            k,
            max_radius,
            progress_bar,
        }
    }

    pub fn render(&self, eye_pos: Vector3D<F>) -> Vec<(u8, u8, u8)> {
        let fov = F::from(self.fov).unwrap();
        let scale: F = (fov * F::from(0.5 as f64).unwrap()).to_radians().tan();

        par_render(
            self.dims.width, self.dims.height,
            self.rr,
            scale,
            eye_pos,
            self.scene_gen.clone(),
            self.spp,
            self.thread_count,
            self.the_tree.clone(),
            self.k,
            self.max_radius,
            self.progress_bar.clone(),
        )
    }
}

fn par_render<F: Float>(
    width: u32, height: u32,
    rr: F,
    scale: F,
    eye_pos: Vector3D<F>,
    scene_gen: Arc<dyn SceneGenerator<F>>,
    spp: u32,
    thread_count: u32,
    the_tree: TheTree<F>,
    k: u32,
    max_radius: F,
    progress_bar: ProgressBar,
) -> Vec<(u8, u8, u8)> {
    let mut thread_handle_vec: Vec<JoinHandle<Vec<(u8, u8, u8)>>> = Vec::new();

    for t in 0..thread_count {
        let the_tree = the_tree.clone();
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
                the_tree,
                k,
                max_radius,
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

struct RenderThread<F: Float> {
    pub width: u32,
    pub height: u32,

    pub rr: F,

    pub scale: F,
    pub eye_pos: Vector3D<F>,

    pub objects: Vec<Arc<dyn RayTraceable<F>>>,
    pub lightsources: Vec<Arc<dyn RayTraceable<F>>>,

    pub total_illumination_area: F,

    the_tree: TheTree<F>,
    k: u32,
    max_radius: F,
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
    the_tree: TheTree<F>,
    k: u32,
    max_radius: F,
    progress_bar: ProgressBar,
) -> Vec<(u8, u8, u8)> {
    let mut lightsources = Vec::new();
    for object in scene.objects.clone() {
        if let Some(_) = object.emit() { // Is light source
            lightsources.push(object);
        }
    }

    let mut total_illumination_area = F::zero();
    for lightsource in lightsources.clone() {
        total_illumination_area = total_illumination_area + lightsource.area();
    }

    let render_thread = RenderThread {
        width,
        height,
        rr,
        scale,
        eye_pos,
        objects: scene.objects,
        lightsources,
        total_illumination_area,
        the_tree,
        k,
        max_radius,
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

fn thresh_rgb<F: Float>(mut pixel: Vector3D<F>, thresh: F) -> Vector3D<F> {
    if pixel.magnitude() > thresh {
        return pixel.norm() * thresh;
    }

    pixel
}

impl<F: Float> RenderThread<F> {
    fn sample_lightsource(
        &self,
        seed: F,
    ) -> Arc<dyn RayTraceable<F>> {
        let mut partial_illum_area = self.total_illumination_area * seed;
        for lightsource in self.lightsources.clone() {
            if let Some(emit) = lightsource.emit() { // Is light source
                partial_illum_area = partial_illum_area - lightsource.area();
                if partial_illum_area <= F::zero() { // The chosen one
                    return lightsource;
                }
            }
        }

        panic!("faulty seed or illumination area")
    }

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

            let local_res = self.cast_ray(
                &Ray::new(eye_pos, dir)
            );
            let local_res = thresh_rgb(local_res, F::from(1.2).unwrap());
            if local_res.x < F::zero() || local_res.y < F::zero() || local_res.z < F::zero() {
                println!("negative pixel");
            }

            res += local_res * _1_spp;
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

    fn calc_direct_brdf(
        &self,
        processed: &ProcessedIncident<F>,
        next_object: Arc<dyn RayTraceable<F>>,
    ) -> Vector3D<F> {
        if let Some(emit) = next_object.emit() {
            return emit * processed.multiplier();
        }

        Vector3D::zero()
    }

    fn calc_direct_light(
        &self,
        object: Arc<dyn RayTraceable<F>>,
        incident: &Incident<F>,
        seed: F,
    ) -> Vector3D<F> {
        if object.focus() { // Skip direct light on transparent object for now
            return Vector3D::zero();
        }

        let lightsource = self.sample_lightsource(seed);
        let emit = lightsource.emit().expect("the sun!no!!!!!");
        let (coords, normal, light_pdf_area) = lightsource.sample_position();

        let w_r = (coords - incident.coords()).norm();
        if w_r.dot(incident.normal()) < F::zero() {
            // println!("attempt to sample from inside");
            return Vector3D::zero();
        }

        let light_ray = Ray::new(
            incident.coords(),
            w_r,
        );

        if let Some((
                        next_object,
                        next_incident,
                    )) = self.intersect(&light_ray) {
            let epsilon = F::from(0.1f32).unwrap();
            if (next_incident.coords() - coords).magnitude() < epsilon {
                let x_diff = incident.coords() - coords;
                let _cos = x_diff.norm().dot(next_incident.normal());
                let light_pdf = light_pdf_area * x_diff.dot(x_diff) / _cos;

                let processed = object.interact_predetermined(
                    incident.clone(),
                    w_r, // Outgoing
                    light_pdf,
                    seed);

                return emit * processed.multiplier();
            }
        }

        Vector3D::zero()
    }

    fn calc_indirect(
        &self,
        processed: &ProcessedIncident<F>,
        next_object: Arc<dyn RayTraceable<F>>,
        next_incident: Incident<F>,
    ) -> Vector3D<F> {
        processed.multiplier() * self.calc(next_object, next_incident)
    }

    fn calc_caustics(
        &self,
        object: Arc<dyn RayTraceable<F>>,
        incident: &Incident<F>,
        seed: F,
    ) -> Vector3D<F> {
        let coords = incident.coords();
        // Do k-NN on the tree
        if !self.the_tree.within_radius(coords, self.max_radius) {
            return Vector3D::zero();
        }

        let (photons, r) = self.the_tree.knn(coords, self.k);
        // println!("r: {}", r.to_f64().unwrap());
        if r > self.max_radius {
            return Vector3D::zero();
        }

        let mut l_x: Vector3D<F> = Vector3D::zero();

        for photon in photons {
            let diff = photon.diff();
            if diff.magnitude() < F::from(0.1f32).unwrap() {
                println!("diff incredibly small {}", diff.magnitude().to_f64().unwrap());
            }

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
            let local_irr = processed.multiplier() * diff;
            l_x = local_irr + l_x;
        }

        l_x
    }

    fn calc(
        &self,
        object: Arc<dyn RayTraceable<F>>,
        incident: Incident<F>,
    ) -> Vector3D<F> {
        if let Some(diff) = object.emit() {
            return diff;
        }

        let seed = F::sample_rand();

        let w_0 = F::from(0.7f32).unwrap();
        let w_1 = F::from(0.3f32).unwrap();
        let w_2 = F::from(0.5f32).unwrap();

        let processed = object.interact(incident, seed);
        let mut l_x: Vector3D<F> = Vector3D::zero();

        l_x = l_x + self.calc_direct_light(
            object.clone(),
            &incident,
            seed,
        ) * w_0;

        l_x = l_x + self.calc_caustics(
            object.clone(),
            &incident,
            seed,
        ) * w_2;

        if let Some((
                        next_object,
                        next_incident,
                    )) = self.intersect(&processed.next_ray()) {
            l_x = l_x + self.calc_direct_brdf(
                &processed,
                next_object.clone(),
            ) * w_1;

            let _thresh = F::from(1.2f32).unwrap();
            if l_x.x > _thresh || l_x.y > _thresh || l_x.z > _thresh {
                return l_x;
            }

            if next_object.emit().is_none() && seed < self.rr {
                let indirect = self.calc_indirect(&processed, next_object, next_incident);
                l_x = l_x + (indirect / self.rr);
            }
        }

        l_x
    }

    fn cast_ray(&self, ray: &Ray<F>) -> Vector3D<F> {
        if let Some((object, incident)) = self.intersect(ray) {
            return self.calc(object, incident);
        }

        Vector3D::zero()
    }
}
