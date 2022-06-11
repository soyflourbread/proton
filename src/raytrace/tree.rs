use kd_tree::{KdPoint, KdTree};
use crate::types::Float;
use crate::vector::Vector3D;

pub struct Photon<F: Float> {
    origin: Vector3D<F>,

    w_i: Vector3D<F>,

    id: usize,
}

impl<F: Float> KdPoint for Photon<F> {
    type Scalar = f64;
    type Dim = typenum::U3;
    // 2 dimensional tree.
    fn at(&self, k: usize) -> f64 {
        let res = match k {
            0 => self.origin.x,
            1 => self.origin.x,
            2 => self.origin.x,
            _ => panic!("wrong dims"),
        };

        res.to_f64().unwrap()
    }
}


pub struct TheTree<F: Float> {
    inner: KdTree<Photon<F>>,
}

impl<F: Float> TheTree<F> {
    pub fn new(points: Vec<Photon<F>>) -> Self {
        let inner = KdTree::build_by_ordered_float(points);

        Self {
            inner,
        }
    }
}