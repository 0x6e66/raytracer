#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]

pub struct vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl std::ops::Add<vec3> for vec3 {
    type Output = vec3;

    fn add(self, other: vec3) -> vec3 {
        vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::AddAssign<vec3> for vec3 {
    fn add_assign(&mut self, other: vec3) {
        *self = vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub<vec3> for vec3 {
    type Output = vec3;

    fn sub(self, other: vec3) -> vec3 {
        vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Neg for vec3 {
    type Output = vec3;

    fn neg(self) -> vec3 {
        vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl std::ops::Mul<vec3> for vec3 {
    type Output = f32;

    fn mul(self, other: vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl std::ops::Mul<f32> for vec3 {
    type Output = vec3;

    fn mul(self, other: f32) -> vec3 {
        vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl std::ops::Div<f32> for vec3 {
    type Output = vec3;

    fn div(self, other: f32) -> vec3 {
        vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl vec3 {
    pub fn norm(&mut self) -> f32 {
        return f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
    }

    pub fn cross(self, vec: vec3) -> vec3 {
        return vec3 {
            x: self.y * vec.z - self.z * vec.y,
            y: self.z * vec.x - self.x * vec.z,
            z: self.x * vec.y - self.y * vec.x,
        };
    }

    pub fn normalize(&mut self) -> vec3 {
        let tmp: f32 = self.norm();
        return vec3 {
            x: self.x / tmp,
            y: self.y / tmp,
            z: self.z / tmp,
        };
    }

    pub fn look_at(&mut self, to: vec3) -> vec3 {
        let forward = (*self - to).normalize();
        let right = vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
        .cross(forward);
        let up = forward.cross(right);

        let current_pos = ndarray::arr1(&[self.x, self.y, self.z, 0.0]);

        let transformation_matrix = ndarray::arr2(&[
            [right.x, right.y, right.z, 0.0],
            [up.x, up.y, up.z, 0.0],
            [forward.x, forward.y, forward.z, 0.0],
            [self.x, self.y, self.z, 1.0],
        ]);

        println!("{:?}\n{:?}\n", transformation_matrix, current_pos);

        let new_cords = transformation_matrix.dot(&current_pos);

        println!("{:?}\n", new_cords);
        let res = vec3 {
            x: new_cords[0] / new_cords[3],
            y: new_cords[1] / new_cords[3],
            z: new_cords[2] / new_cords[3],
        };
        return res;
    }

    pub fn rot_x(&mut self, theta: f32) -> vec3 {
        return vec3 {
            x: self.x,
            y: self.y * f32::cos(theta) - self.z * f32::sin(theta),
            z: self.y * f32::sin(theta) + self.z * f32::cos(theta),
        };
    }

    pub fn rot_y(&mut self, theta: f32) -> vec3 {
        return vec3 {
            x: self.x * f32::cos(theta) + self.z * f32::sin(theta),
            y: self.y,
            z: -self.x * f32::sin(theta) + self.z * f32::cos(theta),
        };
    }

    pub fn rot_z(&mut self, theta: f32) -> vec3 {
        return vec3 {
            x: self.x * f32::cos(theta) - self.y * f32::sin(theta),
            y: self.x * f32::sin(theta) + self.y * f32::cos(theta),
            z: self.z,
        };
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub refractive_index: f32,
    pub diffuse_multiplier: f32,
    pub specular_multiplier: f32,
    pub reflection_multiplier: f32,
    pub refraction_multiplier: f32,
    pub diffuse_color: vec3,
    pub specular_color: vec3,
    pub specular_exponent: f32,
}

impl Material {
    pub const fn new(
        refractive_index: f32,
        diffuse_multiplier: f32,
        specular_multiplier: f32,
        reflection_multiplier: f32,
        refraction_multiplier: f32,
        diffuse_color: vec3,
        specular_color: vec3,
        specular_exponent: f32,
    ) -> Material {
        Material {
            refractive_index: refractive_index,
            diffuse_multiplier: diffuse_multiplier,
            specular_multiplier: specular_multiplier,
            reflection_multiplier: reflection_multiplier,
            refraction_multiplier: refraction_multiplier,
            diffuse_color: diffuse_color,
            specular_color: specular_color,
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
            diffuse_color: vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            specular_color: vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            specular_exponent: 0.0,
        }
    }
}

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
