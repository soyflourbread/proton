use crate::types::Float;
use crate::vector::Vector3D;

struct Photon<F: Float> {
    origin: Vector3D<F>,

    w_i: Vector3D<F>,

    id: usize,
}

pub struct TheTree {

}