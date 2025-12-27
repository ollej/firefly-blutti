use firefly_rust::Point;

use crate::tile_collider::*;

pub struct Collision {
    pub tile_collider: TileCollider,
    pub position: Point,
}
