use crate::raytrace::objects::RayTraceable;
use crate::types::Float;

use std::sync::Arc;

pub struct Scene<F: Float> {
    pub objects: Vec<Arc<dyn RayTraceable<F>>>
}

pub trait SceneGenerator<F: Float>: Send + Sync {
    fn gen_scene(&self) -> Scene<F>;
}
