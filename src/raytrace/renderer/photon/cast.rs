use std::sync::Arc;
use std::thread::JoinHandle;

use crate::raytrace::{Incident, Ray, RayTraceable, Scene, SceneGenerator};
use crate::raytrace::tree::{Photon, TheTree};
use crate::types::Float;
use crate::vector::Vector3D;

fn sample_lightsource<F: Float>(
    lightsource_vec: Vec<Arc<dyn RayTraceable<F>>>,
    total_illumination_area: F,
    seed: F,
) -> Arc<dyn RayTraceable<F>> {
    let mut partial_illum_area = total_illumination_area * seed;
    for lightsource in lightsource_vec.clone() {
        if let Some(emit) = lightsource.emit() { // Is light source
            partial_illum_area = partial_illum_area - lightsource.area();
            if partial_illum_area <= F::zero() { // The chosen one
                return lightsource;
            }
        }
    }

    panic!("faulty seed or illumination area")
}

pub fn gen_photon_map<F: Float>(
    rr: F,
    photon_count: u32,
    scene_gen: Arc<dyn SceneGenerator<F>>,
    thread_count: u32,
) -> TheTree<F> {
    let scene = scene_gen.gen_scene();

    let mut lightsource_vec = Vec::new();
    for object in scene.objects.clone() {
        if let Some(emit) = object.emit() { // Is light source
            lightsource_vec.push(object);
        }
    }

    let mut total_illumination_area = F::zero();
    for lightsource in lightsource_vec.clone() {
        if let Some(emit) = lightsource.emit() { // Is light source
            total_illumination_area = total_illumination_area + lightsource.area();
        }
    }
    println!("Total illum area: {}", total_illumination_area.to_f64().unwrap());

    let photon_per_thread = photon_count / thread_count;
    let mut thread_handle_vec: Vec<JoinHandle<Vec<Photon<F>>>> = Vec::new();

    for t in 0..thread_count {
        let scene_gen = scene_gen.clone();

        let handle = std::thread::spawn(move || {
            let scene = scene_gen.gen_scene();

            cast_thread(
                rr,
                scene,
                total_illumination_area,
                photon_count,
                photon_per_thread,
                t,
                thread_count,
            )
        });

        thread_handle_vec.push(handle);
    }

    let mut photons: Vec<Photon<F>> = Vec::with_capacity(
        photon_count as usize
    );
    for thread in thread_handle_vec {
        let mut _photons = thread.join().expect("general error");
        photons.append(&mut _photons);
    }

    println!("{} photons registered", photons.len());
    println!("({}, {}, {})",
             photons[0].coords().x.to_f64().unwrap(),
             photons[0].coords().y.to_f64().unwrap(),
             photons[0].coords().z.to_f64().unwrap(),
    );

    TheTree::new(photons)
}

fn cast_thread<F: Float>(
    rr: F,
    scene: Scene<F>,
    total_illumination_area: F,
    photon_count: u32,
    photon_per_thread: u32,
    t: u32,
    thread_count: u32,
) -> Vec<Photon<F>> {
    let mut lightsource_vec = Vec::new();
    for object in scene.objects.clone() {
        if let Some(emit) = object.emit() { // Is light source
            lightsource_vec.push(object);
        }
    }

    let cast_thread = CastThread {
        rr,
        objects: scene.objects,
    };

    let mut photons: Vec<Photon<F>> = Vec::with_capacity(
        photon_per_thread as usize
    );
    for _ in 0..photon_per_thread {
        let seed = F::sample_rand();

        let lightsource = sample_lightsource(
            lightsource_vec.clone(),
            total_illumination_area,
            seed);
        let (ray, pdf) = lightsource.sample_light();
        let diff = lightsource.emit().unwrap();

        let _two = F::from(2u32).unwrap();
        let diff = diff / pdf;
        let diff = diff / F::from(photon_count).unwrap();
        let diff = diff * _two *  F::PI();

        cast_thread.cast_ray(&ray, diff, &mut photons);
    }

    photons
}

struct CastThread<F: Float> {
    pub rr: F,

    pub objects: Vec<Arc<dyn RayTraceable<F>>>,
}

impl<F: Float> CastThread<F> {
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

    fn cast_ray(
        &self,
        ray: &Ray<F>,
        diff: Vector3D<F>,
        photons: &mut Vec<Photon<F>>,
    ) {
        let seed = F::sample_rand();

        if let Some((object, incident)) = self.intersect(ray) {
            let photon = Photon::new(
                incident.coords(),
                incident.w_i(), // Inverse of incoming direction
                diff, // Do not multiply f_r
            );
            photons.push(photon);

            if seed < self.rr { // Might just stop
                let processed = object.interact(incident, seed / self.rr);

                let next_ray = processed.next_ray();
                let multiplier = processed.f_r()
                    * F::PI()
                    * incident.w_i().dot(incident.normal());
                self.cast_ray(&next_ray, diff, photons); // TODO: diff faulty?
            }
        }
    }
}
