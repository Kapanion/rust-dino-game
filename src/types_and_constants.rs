pub type Screen2 = (f32, f32);

pub const GROUND_Y_COORD    : f32 =   -500.0;
pub const JUMP_VELOCITY     : f32 =  1150.0;
pub const DINO_GRAVITY      : f32 = -3800.0;
pub const START_SCROLL_SPEED: f32 =   700.0;
pub const MAX_SCROLL_SPEED  : f32 =  2100.0;
pub const CACTUS_MIN_DELAY  : f32 =     0.7;
pub const PTERO_SPEED       : f32 =   100.0;

pub const NUM_OF_COLLIDERS  : usize = 2;
pub const SHOW_COLLIDERS    : bool = false;
pub const PAUSE_ENABLED     : bool = true;

pub const SCREEN: Screen2 = (1800.0, 1200.0);
pub const DESIRED_FPS: u32 = 60;

pub const RNG_DEFAULT_SEED: u64 = 69420;