use crate::Vec2;

pub struct Particle {
    pub pos: Vec2,
    pub direction: Vec2,
    pub opacity: f32,
    pub time_to_live: i64,
}
