use std::ops::Range;

use specs::{Component, VecStorage};

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct MeshRenderer {}

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct Mesh {
    pub vertices: Range<u32>,
    pub indices: Range<u32>,
}
