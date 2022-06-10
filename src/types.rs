pub trait Float: std::fmt::Debug + num::Float + Send + 'static + num::traits::FloatConst + CanRNG {}

pub trait CanRNG {
    fn sample_rand() -> Self;
}

use rand::Rng;

impl CanRNG for f32 {
    fn sample_rand() -> Self {
        let mut rng = rand::thread_rng();
        return rng.gen();
    }
}

impl CanRNG for f64 {
    fn sample_rand() -> Self {
        let mut rng = rand::thread_rng();
        return rng.gen();
    }
}

impl Float for f32 {}

impl Float for f64 {}
