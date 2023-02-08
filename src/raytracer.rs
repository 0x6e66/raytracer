use crate::utils::{vec3, Light, Material, Sphere};
use chrono::{Datelike, Timelike};
use image::RgbImage;

pub struct Raytracer {
    width: u32,
    height: u32,
    background_color: vec3,
    floor_dimensions: (f32, f32),
    camera_pos: vec3,
    floor_color: vec3,
    floor_level: f32,
    max_depth: u32,
    offset_for_mitigating_occlusion: f32,
    anti_aliasing_offsets: Vec<(f32, f32)>,
    fov: f32,
    path_output: String,
    spheres: Vec<Sphere>,
    lights: Vec<Light>,
}

impl Raytracer {
    pub fn new(
        resolution: (u32, u32),
        floor_dimensions: (f32, f32),
        background_color: (u32, u32, u32),
        floor_color: (u32, u32, u32),
        camera_pos: (f32, f32, f32),
        floor_level: f32,
        max_depth: u32,
        offset_for_mitigating_occlusion: f32,
        anti_aliasing: u32,
        fov: f32,
        path_output: String,
        spheres: Vec<Sphere>,
        lights: Vec<Light>,
    ) -> Raytracer {
        let mut anti_aliasing_offsets: Vec<(f32, f32)> = vec![];
        for i in 1..anti_aliasing + 1 {
            for j in 1..anti_aliasing + 1 {
                anti_aliasing_offsets.push((
                    f32::round(100.0 * (j as f32 / (anti_aliasing + 1) as f32)) / 100.0,
                    f32::round(100.0 * (-(i as f32) / (anti_aliasing + 1) as f32)) / 100.0,
                ));
            }
        }
        return Raytracer {
            width: resolution.0,
            height: resolution.1,
            floor_dimensions: floor_dimensions,
            background_color: vec3 {
                x: background_color.0 as f32,
                y: background_color.1 as f32,
                z: background_color.2 as f32,
            } / 255.0,
            floor_color: vec3 {
                x: floor_color.0 as f32,
                y: floor_color.1 as f32,
                z: floor_color.2 as f32,
            } / 255.0,
            camera_pos: vec3 {
                x: camera_pos.0,
                y: camera_pos.1,
                z: camera_pos.2,
            },
            floor_level: floor_level,
            max_depth: max_depth,
            offset_for_mitigating_occlusion: offset_for_mitigating_occlusion,
            anti_aliasing_offsets: anti_aliasing_offsets,
            fov: fov,
            path_output: path_output,
            spheres: spheres,
            lights: lights,
        };
    }

    fn reflect(&mut self, vector: vec3, axis: vec3) -> vec3 {
        return vector - axis * (vector * axis) * 2.0;
    }

    fn refract(&mut self, mut vector: vec3, mut axis: vec3, eta_t: f32, eta_i: f32) -> vec3 {
        let cosi = -(vector * axis) / (vector.norm() * axis.norm());
        if cosi > 0.0 {
            let eta = eta_i / eta_t;
            let tmp = 1.0 - f32::powf(eta, 2.0) + f32::powf(eta * cosi, 2.0);
            return vector * eta + axis * eta * cosi - axis * f32::sqrt(tmp);
        }
        return self.refract(vector, -axis, eta_i, eta_t);
    }

    fn intersect_between_ray_and_sphere(
        &mut self,
        origin: vec3,
        direction: vec3,
        s: Sphere,
    ) -> (bool, f32) {
        let l = s.center - origin;
        let tca = l * direction;
        let d2 = l * l - f32::powf(tca, 2.0);
        if d2 < f32::powf(s.radius, 2.0) {
            let thc = f32::sqrt(f32::powf(s.radius, 2.0) - d2);
            let t0 = tca - thc;
            let t1 = tca + thc;
            if t0 > self.offset_for_mitigating_occlusion {
                return (true, t0);
            } else if t1 > self.offset_for_mitigating_occlusion {
                return (true, t1);
            }
        }
        return (false, 0.0);
    }

    fn scene_intersect(&mut self, origin: vec3, direction: vec3) -> (bool, vec3, vec3, Material) {
        let mut point = vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let mut normal = vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let mut material = Material::default();
        let mut nearest_dist = 1000.0;

        if direction.y.abs() > 0.0 {
            let distance = -(origin.y + (-self.floor_level)) / direction.y;
            let tmp_point = origin + direction * distance;

            let floor_hit = distance > self.offset_for_mitigating_occlusion
                && distance < nearest_dist
                && f32::abs(tmp_point.x) < self.floor_dimensions.0
                && (-10.0 - self.floor_dimensions.1) < tmp_point.z
                && tmp_point.z < -10.0;

            if floor_hit {
                nearest_dist = distance;
                point = tmp_point;
                normal = vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                };
                material.diffuse_color = self.floor_color;
            }
        }

        for s in self.spheres.clone() {
            let (intersection, distance) =
                self.intersect_between_ray_and_sphere(origin, direction, s);
            if intersection && distance <= nearest_dist {
                nearest_dist = distance;
                point = origin + direction * nearest_dist;
                normal = (point - s.center).normalize();
                material = s.material;
            }
        }
        return (nearest_dist < 1000.0, point, normal, material);
    }

    fn map_range(&mut self, from_range: (f32, f32), to_range: (f32, f32), s: f32) -> f32 {
        to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
    }

    fn cast_ray(&mut self, origin: vec3, direction: vec3, depth: u32) -> vec3 {
        let (hit, point, normal, material) = self.scene_intersect(origin, direction);
        if depth == self.max_depth || !hit {
            let mut bc = self.map_range((-1.0, 1.0), (0.0, 0.8), direction.y);
            bc = ((100.0 * bc) as u32) as f32 / 100.0;
            return vec3 {
                x: bc * self.background_color.x,
                y: bc * self.background_color.y,
                z: bc * self.background_color.z,
            };
        }

        let direction_of_reflection = self.reflect(direction, normal).normalize();
        let direction_of_refraction = self
            .refract(direction, normal, material.refractive_index, 1.0)
            .normalize();
        let color_of_reflection = self.cast_ray(point, direction_of_reflection, depth + 1);
        let color_of_refraction = self.cast_ray(point, direction_of_refraction, depth + 1);

        let mut diffuse_light_intensity = 0.0;
        let mut specular_light_intensity = 0.0;
        for light in self.lights.clone() {
            let light_dir = (light.pos - point).normalize();
            let (hit, shadow_pt, _, _) = self.scene_intersect(point, light_dir);

            if !(hit && (shadow_pt - point).norm() < (light.pos - point).norm()) {
                diffuse_light_intensity += f32::max(0.0, light_dir * normal) * light.intensity;
                specular_light_intensity += f32::powf(
                    f32::max(0.0, -self.reflect(-light_dir, normal) * direction),
                    material.specular_exponent,
                ) * light.intensity;
            }
        }
        let diffuse_color =
            material.diffuse_color * diffuse_light_intensity * material.diffuse_multiplier;
        let specular_color =
            material.specular_color * specular_light_intensity * material.specular_multiplier;
        let reflection_color = color_of_reflection * material.reflection_multiplier;
        let refraction_color = color_of_refraction * material.refraction_multiplier;

        return diffuse_color + specular_color + reflection_color + refraction_color;
    }

    fn save_image(&mut self, img: RgbImage, versionize: bool) {
        let mut path_buf = std::path::PathBuf::new();
        if !std::path::Path::new(self.path_output.as_str()).exists() {
            std::fs::create_dir(self.path_output.as_str()).unwrap();
        }
        path_buf.push(self.path_output.as_str());
        if versionize {
            let year = chrono::Local::now().year();
            let month = chrono::Local::now().month();
            let day = chrono::Local::now().day();
            let hour = chrono::Local::now().hour();
            let minute = chrono::Local::now().minute();
            let second = chrono::Local::now().second();
            let img_name = format!(
                "{}-{:0>2}-{:0>2}_{:0>2}{:0>2}{:0>2}_out.png",
                year, month, day, hour, minute, second
            );
            path_buf.push(img_name);
        } else {
            path_buf.push("out.png");
        }

        match img.save(path_buf.to_str().unwrap()) {
            Err(e) => println!("{:?}", e),
            _ => (),
        }
    }

    pub fn start(&mut self, versionize: bool) {
        let mut img = image::RgbImage::new(self.width, self.height);

        let x_rot = - 0.2;
        let y_rot = 0.0;
        let z_rot = 0.0;
        let dir_z = - (self.height as f32) / (2.0 * f32::tan(self.fov / 2.0));
        for h in tqdm::tqdm(0..self.height) {
            for w in 0..self.width {
                let mut color = vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                };
                for i in 0..self.anti_aliasing_offsets.len() {
                    let dir_x = w as f32 - (self.width as f32 / 2.0).floor()
                        + self.anti_aliasing_offsets[i].0;
                    let dir_y = (self.height as f32 / 2.0).floor()
                        - h as f32
                        - self.anti_aliasing_offsets[i].1;
                    color += self.cast_ray(
                        self.camera_pos,
                        vec3 {
                            x: dir_x,
                            y: dir_y,
                            z: dir_z,
                        }
                        .rot_x(x_rot)
                        .rot_y(y_rot)
                        .rot_z(z_rot)
                        .normalize(),
                        0,
                    );
                }
                color = (color / self.anti_aliasing_offsets.len() as f32) * 255.0;

                img.put_pixel(
                    w,
                    h,
                    image::Rgb([
                        color.x.floor() as u8,
                        color.y.floor() as u8,
                        color.z.floor() as u8,
                    ]),
                );
            }
        }
        self.save_image(img, versionize);
    }
}
