# Documentation

## Author

- Name: Niklas Frondorf

## Sections

### (Fundamental) Rendering loop (5 points)

```rust
let img = image::RgbImage::new(WIDTH, HEIGHT);
for h in 0..HEIGHT {
    for w in 0..self.width {
        color = self.cast_ray(...);
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
