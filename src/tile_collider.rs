use serde::Deserialize;

#[derive(PartialEq, Clone, Copy, Debug, Deserialize)]
pub enum TileCollider {
    Climbable,
    Collectable(i32),
    Conveyor,
    Deadly,
    Exit,
    ExtraLife,
    Full,
    None,
    Slippery,
    Star,
}
