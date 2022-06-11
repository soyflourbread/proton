mod cast;
mod render;

use std::sync::Arc;

use crate::raytrace::renderer::Dimensions;
use crate::raytrace::{Incident, Ray, RayTraceable, Scene, SceneGenerator};
use crate::raytrace::tree::{Photon, TheTree};
use crate::types::Float;
use crate::vector::Vector3D;

use indicatif::ProgressBar;
use num::traits::real::Real;

pub struct PhotonRenderer<F: Float> {
    dims: Dimensions,

    fov: u32,

    rr: F,

    scene_gen: Arc<dyn SceneGenerator<F>>,

    thread_count: u32,

    progress_bar: ProgressBar,
}

impl<F: Float> PhotonRenderer<F> {
    pub fn new(
        dims: Dimensions,
        fov: u32,
        rr: F,
        scene_gen: Arc<dyn SceneGenerator<F>>,
        thread_count: u32,
        progress_bar: ProgressBar,
    ) -> Self {
        Self {
            dims,
            fov,
            rr,
            scene_gen,
            thread_count,
            progress_bar,
        }
    }

    pub fn render(&self, eye_pos: Vector3D<F>) -> Vec<(u8, u8, u8)> {
        let fov = F::from(self.fov).unwrap();
        let scale: F = (fov * F::from(0.5 as f64).unwrap()).to_radians().tan();

        let the_tree = cast::gen_photon_map(
            self.rr,
            800000,
            self.scene_gen.clone(),
            self.thread_count,
        );

        render::par_render(
            self.dims.width, self.dims.height,
            scale,
            eye_pos,
            self.scene_gen.clone(),
            self.thread_count,
            the_tree,
            8
        )
    }
}
