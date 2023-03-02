use crate::utils::vec3::vec3;
use crate::utils::material::Material;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub center: vec3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub const fn new(center: vec3, radius: f32, material: Material) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
            material: material,
        }
    }
}