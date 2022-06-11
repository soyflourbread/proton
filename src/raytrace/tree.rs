use kd_tree::{KdPoint, KdTree};
use crate::types::Float;
use crate::vector::Vector3D;

#[derive(Clone)]
pub struct Photon<F: Float> {
    coords: Vector3D<F>,

    w_i: Vector3D<F>,

    diff: Vector3D<F>,
}

impl<F: Float> Photon<F> {
    pub fn new(
        coords: Vector3D<F>,
        w_i: Vector3D<F>,
        diff: Vector3D<F>,
    ) -> Self {
        Self {
            coords,
            w_i,
            diff,
        }
    }

    pub fn coords(&self) -> Vector3D<F> {
        self.coords
    }

    pub fn w_i(&self) -> Vector3D<F> {
        self.w_i
    }

    pub fn diff(&self) -> Vector3D<F> {
        self.diff
    }
}

impl<F: Float> KdPoint for Photon<F> {
    type Scalar = f64;
    type Dim = typenum::U3;
    // 2 dimensional tree.
    fn at(&self, k: usize) -> f64 {
        let res = match k {
            0 => self.coords.x,
            1 => self.coords.y,
            2 => self.coords.z,
            _ => panic!("wrong dims"),
        };

        res.to_f64().unwrap()
    }
}

#[derive(Clone)]
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

    pub fn knn(&self, coords: Vector3D<F>, k: u32) -> (Vec<Photon<F>>, F) {
        let found = self.inner.nearests(
            &[
                coords.x.to_f64().unwrap(),
                coords.y.to_f64().unwrap(),
                coords.z.to_f64().unwrap(),
            ],
            k as usize,
        );

        let photons = found.iter()
            .map(|f| f.item.clone())
            .collect::<Vec<Photon<F>>>();

        let mut radius = F::zero();
        for photon in &photons {
            radius = radius.max((photon.coords - coords).magnitude());
        }

        (photons, radius)
    }
}