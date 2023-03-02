use crate::utils::vec3::vec3;

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub refractive_index: f32,
    pub diffuse_multiplier: f32,
    pub specular_multiplier: f32,
    pub reflection_multiplier: f32,
    pub refraction_multiplier: f32,
    pub color: vec3,
    pub specular_exponent: f32,
}

impl Material {
    pub const fn new(
        refractive_index: f32,
        diffuse_multiplier: f32,
        specular_multiplier: f32,
        reflection_multiplier: f32,
        refraction_multiplier: f32,
        color: vec3,
        specular_exponent: f32,
    ) -> Material {
        Material {
            refractive_index: refractive_index,
            diffuse_multiplier: diffuse_multiplier,
            specular_multiplier: specular_multiplier,
            reflection_multiplier: reflection_multiplier,
            refraction_multiplier: refraction_multiplier,
            color: color,
            specular_exponent: specular_exponent,
        }
    }

    pub fn default() -> Material {
        Material {
            refractive_index: 1.0,
            diffuse_multiplier: 2.0,
            specular_multiplier: 0.0,
            reflection_multiplier: 0.0,
            refraction_multiplier: 0.0,
            color: vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            specular_exponent: 0.0,
        }
    }
}