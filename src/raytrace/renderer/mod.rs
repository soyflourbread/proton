mod simple;
mod photon;

use crate::raytrace::SceneGenerator;
use crate::types::Float;
use crate::vector::Vector3D;

use std::sync::Arc;

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

        let res_vec = simple::par_render(
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
