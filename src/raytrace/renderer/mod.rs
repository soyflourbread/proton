mod simple;
mod cast;

use crate::raytrace::SceneGenerator;
use crate::types::Float;
use crate::vector::Vector3D;

use std::sync::Arc;

use image::GenericImage;
use indicatif::ProgressBar;
use num::traits::real::Real;

#[derive(Debug, Clone, Copy)]
pub struct Dimensions {
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
        scene_gen: Arc<dyn SceneGenerator<F>>,
        thread_count: u32) -> Self {
        Self {
            dims: Dimensions {
                width,
                height,
            },
            fov,
            spp: 64,
            rr: F::from(0.8 as f64).unwrap(),
            scene_gen,
            thread_count,
            progress_bar: ProgressBar::new((width * height) as u64),
        }
    }
}

impl<F: Float> Renderer<F> {
    pub fn render(&self, eye_pos: Vector3D<F>) -> image::DynamicImage {
        let start = std::time::Instant::now();
        let the_tree = cast::gen_photon_map(
            self.rr,
            10000000, // 10m photon, a portion wasted
            self.scene_gen.clone(),
            self.thread_count,
        );
        let duration = start.elapsed();
        println!("Time elapsed in gen_photon_map() is: {:?}", duration);

        let mut im = image::DynamicImage::new_rgb8(self.dims.width, self.dims.height);

        let start = std::time::Instant::now();
        let simple_renderer = simple::SimpleRenderer::new(
            self.dims,
            self.fov,
            self.spp,
            self.rr,
            self.scene_gen.clone(),
            self.thread_count,
            the_tree,
            8,
            F::from(10u32).unwrap(),
            self.progress_bar.clone(),
        );

        let res_vec = simple_renderer.render(eye_pos);

        for w in 0..self.dims.width {
            for h in 0..self.dims.height {
                let (r, g, b) = res_vec[(w * self.dims.height + h) as usize];
                im.put_pixel(w, h, image::Rgba::from([r, g, b, 255]));
            }
        }

        self.progress_bar.finish();

        let duration = start.elapsed();
        println!("Time elapsed in render() is: {:?}", duration);

        im
    }
}
