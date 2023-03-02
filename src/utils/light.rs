use crate::utils::vec3::vec3;


#[derive(Debug, Copy, Clone)]
pub struct Light {
    pub pos: vec3,
    pub intensity: f32,
}

impl Light {
    pub const fn new(pos: vec3, intensity: f32) -> Light {
        Light {
            pos: pos,
            intensity: intensity,
        }
    }
}