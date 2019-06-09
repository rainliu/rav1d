use crate::plane::*;
use crate::util::*;

#[derive(Debug, Clone)]
pub struct Frame<T: Pixel> {
    pub planes: [Plane<T>; 3],
}
