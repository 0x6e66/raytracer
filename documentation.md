# Documentation

## Author

- Name: Niklas Frondorf

## General

- There is no *Scene* or *Camera* Object
- The Camera is represented by a base vector in the struct *Raytracer* called `camera_pos` and a `direction` which is calculated at runtime 
  - Both of them are passed to the function `cast_ray`

## Sections

### (Fundamental) Rendering loop (5 points)
The main rendering loop(s) can be found in the function `start` of the struct `Raytracer`. The function will iterate over every pixel (height and width), calculate the color of the pixel and store that color. After the loops are done the image is stored to disk in a png format. Blow you can see a snippet of the main loop(s). Note, that the snippet is simplified for better understanding.
```rust
let img = image::RgbImage::new(WIDTH, HEIGHT);
for h in 0..HEIGHT {
    for w in 0..self.width {
        color = self.calc_color_at_pixel(w, h, ...);
        img.put_pixel(w, h, image::Rgb(color));
    }
} 
save_image(img, ...);
```

---
### (Fundamental) Camera (15 points)
In this project the camera is represented by a base vector `camera_pos` (specified in the struct `Raytracer`) and a direction vector `direction`, which is calculated at runtime. In the function `calc_color_at_pixel` of the struct `Raytracer` the color is calculated by casting a ray from the position of the camera (`camera_pos`) in a certain direction (`direction`). Blow you can see a snippet, which shows how the ray is casted. Note, that the snippet is simplified for better understanding.
```rust
color = cast_ray(camera_pos, direction.normalize(), ...);
```

---
### (Fundamental) Objects: shape (15 points)
The only objects in this project are spheres. Spheres are defined as follows:
```rust
pub struct Sphere {
    pub center: vec3,
    pub radius: f32,
    pub material: Material,
}
```
The intersection of rays and spheres is handled by the function `intersect_between_ray_and_sphere`. Blow you can see a snippet, which shows how the intersection is handled. Note, that the snippet is simplified for better understanding.
```rust
let distance_to_center = sphere.center - origin;
let tca = distance_to_center * direction;
let d2 = distance_to_center * distance_to_center - pow(tca, 2);
if d2 < pow(sphere.radius, 2) {
    let thc = sqrt(pow(sphere.radius, 2) - d2);
    let t0 = tca - thc;
    let t1 = tca + thc;
    if t0 > offset_for_mitigating_occlusion {
        return (true, t0);
    } else if t1 > offset_for_mitigating_occlusion {
        return (true, t1);
    }
}
return (false, 0.0);
```
The calculation of the color is done in the function `cast_ray`. More on that later.

---
### (Fundamental) Enhancing camera and rendering loop (10 points)
The anti-aliasing is implemented in the function `calc_color_at_pixel` as follows:
```rust
let color = vec3 {0, 0, 0};
for i in 0..anti_aliasing_offsets.len() {
    color += cast_ray(camera_pos, (direction + anti_aliasing_offsets[i]).normalize(), ...);
}
color = (color / self.anti_aliasing_offsets.len() as f32);
```
You can specify the amount of rays per pixel with the anti_aliasing argument in the function `new` of the struct `Raytracer`. Note that the number of rays per pixel ($n$) is equal to the anti_aliasing argument ($a$) squared $\Rightarrow$ $n = a^2$. This is, because the offsets for each pixel are calculated as follows:
```rust
let anti_aliasing_offsets: Vec<(f32, f32)> = vec![];
for i in 1..anti_aliasing + 1 {
    for j in 1..anti_aliasing + 1 {
        anti_aliasing_offsets.push(
            (j / (anti_aliasing + 1), -i / (anti_aliasing + 1))
        );
    }
}
```
This results in uniformly distributed points inside each pixel. Here are two examples of offset distributions with 2 and 3 as input respectively.

<img src="img/anti_aliasing_2.png" width=45%>
<img src="img/anti_aliasing_3.png" width=45%>

---
### (Fundamental) Object material: diffuse (15 points)
Material in this project are defined as follows:
```rust
pub struct Material {
    pub refractive_index: f32,
    pub diffuse_multiplier: f32,
    pub specular_multiplier: f32,
    pub reflection_multiplier: f32,
    pub refraction_multiplier: f32,
    pub color: vec3,
    pub specular_exponent: f32,
}
```
The diffuse color is calculated in the function `cast_ray` of the struct `Raytracer` as follows:
```rust
let diffuse_color = material.color * diffuse_light_intensity * material.diffuse_multiplier;
```
More on the calculation of `diffuse_light_intensity` [here](#optional-lights-30-points)

---
### (Fundamental) Object material: specular (20 points)
For definition of materials see [here](#fundamental-object-material-diffuse-15-points)
The specular color is calculated in the function `cast_ray` of the struct `Raytracer` as follows:
```rust
let specular_color = material.color * specular_light_intensity * material.specular_multiplier;
```
More on the calculation of `specular_light_intensity` [here](#optional-lights-30-points)

---
### (Optional) Object material: specular transmission (30 points)
---
### (Optional) Lights (30 points)
A Light consists of a position vector (`pos`) and an intensity value (`intensity`) and is defined as follows:
```rust
pub struct Light {
    pub pos: vec3,
    pub intensity: f32,
}
```

The intensity of colors in regards to the light is calculated as follows:
```rust
let diffuse_light_intensity = 0;
let specular_light_intensity = 0;
for light in lights {
    let light_dir = (light.pos - point).normalize();
    let (hit, shadow_pt, _, _) = self.scene_interact(point, light_dir);

    if !(hit && (shadow_pt - point).norm() < (light.pos - point).norm()) {
        diffuse_light_intensity += max(0, light_dir * normal) * light.intensity;

        let tmp_base = max(0, -self.reflect(-light_dir, normal) * direction);
        specular_light_intensity += pow(tmp_base, material.specular_exponent) * light.intensity;
    }
}
```
Note, that the snippet is simplified for better understanding.

---
### (Optional) Positioning and orienting camera (30 points)
- look_at
---
### (Optional) Animation (30 points)
