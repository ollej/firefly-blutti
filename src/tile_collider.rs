use serde::Deserialize;

#[derive(PartialEq, Clone, Copy, Debug, Deserialize)]
pub enum TileCollider {
    Full,
    Climbable,
    Star,
    ExtraLife,
    Deadly,
    Slippery,
    Conveyor,
    Exit,
    None,
}
