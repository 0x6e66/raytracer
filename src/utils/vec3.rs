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

    pub fn look_at(from: vec3, to: vec3, vec: vec3) -> vec3 {
        let forward = (from - to).normalize();
        let right = vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
        .cross(forward);
        let up = forward.cross(right);

        let translation_x = -from * right;
        let translation_y = -from * up;
        let translation_z = -from * forward;

        let transformation_matrix = ndarray::arr2(&[
            [right.x, up.x, forward.x, 0.0],
            [right.y, up.y, forward.y, 0.0],
            [right.z, up.z, forward.z, 0.0],
            [translation_x, translation_y, translation_z, 1.0],
        ]);

        let vec_ = ndarray::arr1(&[vec.x, vec.y, vec.z, 0.0]);
        let new_vec = transformation_matrix.dot(&vec_);

        return vec3 {
            x: new_vec[0],
            y: new_vec[1],
            z: new_vec[2],
        };
    }

    #[allow(dead_code)]
    pub fn rot_x(&mut self, theta: f32) -> vec3 {
        return vec3 {
            x: self.x,
            y: self.y * f32::cos(theta) - self.z * f32::sin(theta),
            z: self.y * f32::sin(theta) + self.z * f32::cos(theta),
        };
    }

    #[allow(dead_code)]
    pub fn rot_y(&mut self, theta: f32) -> vec3 {
        return vec3 {
            x: self.x * f32::cos(theta) + self.z * f32::sin(theta),
            y: self.y,
            z: -self.x * f32::sin(theta) + self.z * f32::cos(theta),
        };
    }

    #[allow(dead_code)]
    pub fn rot_z(&mut self, theta: f32) -> vec3 {
        return vec3 {
            x: self.x * f32::cos(theta) - self.y * f32::sin(theta),
            y: self.x * f32::sin(theta) + self.y * f32::cos(theta),
            z: self.z,
        };
    }
}