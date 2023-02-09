# Documentation

## Author

- Name: Niklas Frondorf

## General

- There is no *Scene* or *Camera* Object
- The Camera is represented by a base vector in the struct *Raytracer* called `camera_pos` and a `direction` which is calculated at runtime 
  - Both of them are passed to the function `cast_ray`

## Sections

### (Fundamental) Rendering loop (5 points)
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

### (Fundamental) Camera (15 points)
### (Fundamental) Objects: shape (15 points)
### (Fundamental) Enhancing camera and rendering loop (10 points)
### (Fundamental) Object material: diffuse (15 points)
### (Fundamental) Object material: specular (20 points)
### (Optional) Object material: specular transmission (30 points)
### (Optional) Lights (30 points)
### (Optional) Positioning and orienting camera (30 points)
### (Optional) Animation (30 points)
