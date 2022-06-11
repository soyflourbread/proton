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
}

fn gen_photon_map<F: Float>(
    rr: F,
    photon_count: usize,
    scene_gen: Arc<dyn SceneGenerator<F>>,
) -> TheTree<F> {
    let scene = scene_gen.gen_scene();

    for _ in 0..photon_count {
        for object in scene.objects.clone() {
        }
    }

    todo!()
}

