#[derive(PartialEq, Debug)]
pub enum PlayerState {
    Idle,
    Jumping,
    JumpingStop,
    Dashing,
    Running,
    RunningStop,
    Climbing,
    ClimbingStop,
    ClimbingSideways,
    ClimbingSidewaysStop,
    ClimbingIdle,
    Falling,
}
