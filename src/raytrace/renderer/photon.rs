use std::sync::Arc;
use indicatif::ProgressBar;
use crate::raytrace::renderer::Dimensions;
use crate::raytrace::SceneGenerator;
use crate::raytrace::tree::TheTree;
use crate::types::Float;
use crate::vector::Vector3D;

pub struct PhotonRenderer<F: Float> {
    dims: Dimensions,

    fov: u32,
    spp: u32,

    rr: F,

    scene_gen: Arc<dyn SceneGenerator<F>>,

    thread_count: u32,

    progress_bar: ProgressBar,
}

impl<F: Float> PhotonRenderer<F> {
    pub fn new(
        dims: Dimensions,
        fov: u32,
        spp: u32,
        rr: F,
        scene_gen: Arc<dyn SceneGenerator<F>>,
        thread_count: u32,
        progress_bar: ProgressBar,
    ) -> Self {
        Self {
            dims,
            fov,
            spp,
            rr,
            scene_gen,
            thread_count,
            progress_bar,
        }
    }

    pub fn render(&self, eye_pos: Vector3D<F>) {
        gen_photon_map(
            self.rr,
            1000usize,
            self.scene_gen.clone(),
        );
    }
}

fn gen_photon_map<F: Float>(
    rr: F,
    photon_count: usize,
    scene_gen: Arc<dyn SceneGenerator<F>>,
) -> TheTree<F> {
    let scene = scene_gen.gen_scene();

    let mut lightsource_vec = Vec::new();
    for object in scene.objects.clone() {
        if let Some(emit) = object.emit() { // Is light source
            lightsource_vec.push(object);
        }
    }

    let mut total_illumination_area = F::zero();
    for lightsource in lightsource_vec {
        if let Some(emit) = lightsource.emit() { // Is light source
            total_illumination_area = total_illumination_area + lightsource.area();
        }
    }
    println!("Total illum area: {}", total_illumination_area.to_f64().unwrap());

    todo!()
}

