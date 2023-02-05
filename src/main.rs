#[allow(non_camel_case_types)]
#[warn(non_snake_case)]
#[allow(non_upper_case_globals)]
mod raytracer;
mod utils;

use crate::utils::{vec3, Light, Material, Sphere};
use std::collections::HashMap;

const R1080P: (u32, u32) = (1920, 1080);
const R400P: (u32, u32) = (600, 400);

fn main() {
    let (spheres, lights) = get_spheres_lights_4();

    let mut tracer = raytracer::Raytracer::new(
        R400P,
        (10.0, 40.0),
        vec3 {
            x: 53.0,
            y: 108.0,
            z: 160.0,
        },
        vec3 {
            x: 230.0,
            y: 102.0,
            z: 30.0,
        },
        -4.0,
        4,
        0.1,
        2,
        1.0,
        "out",
        spheres,
        lights,
    );

    tracer.start(false);
}

fn get_materials() -> HashMap<String, Material> {
    let mut materials: HashMap<String, Material> = HashMap::new();
    materials.insert(
        "mirror".to_string(),
        Material::new(
            1.0,
            0.0,
            16.0,
            0.8,
            0.0,
            vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            1425.0,
        ),
    );
    materials.insert(
        "glass".to_string(),
        Material::new(
            1.5,
            0.0,
            0.9,
            0.1,
            0.8,
            vec3 {
                x: 0.6,
                y: 0.7,
                z: 0.8,
            },
            vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            125.0,
        ),
    );
    materials.insert(
        "rubber".to_string(),
        Material::new(
            1.0,
            1.4,
            0.3,
            0.0,
            0.0,
            vec3 {
                x: 0.3,
                y: 0.1,
                z: 0.1,
            },
            vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            10.0,
        ),
    );
    materials.insert(
        "ivory".to_string(),
        Material::new(
            1.0,
            0.9,
            0.5,
            0.1,
            0.0,
            vec3 {
                x: 0.4,
                y: 0.4,
                z: 0.3,
            },
            vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            50.0,
        ),
    );
    materials.insert(
        "amber".to_string(),
        Material::new(
            1.5,
            0.9,
            1.5,
            0.1,
            0.0,
            vec3 {
                x: 1.0,
                y: 0.5,
                z: 0.0,
            },
            vec3 {
                x: 0.5,
                y: 0.5,
                z: 0.5,
            },
            50.0,
        ),
    );
    materials.insert(
        "sapphire".to_string(),
        Material::new(
            1.5,
            0.9,
            1.5,
            0.1,
            0.0,
            vec3 {
                x: 0.0,
                y: 0.5,
                z: 1.0,
            },
            vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            50.0,
        ),
    );
    return materials;
}

fn get_spheres_lights_1() -> (Vec<Sphere>, Vec<Light>) {
    let materials = get_materials();

    let mut spheres: Vec<Sphere> = Vec::new();
    spheres.push(Sphere::new(
        vec3 {
            x: -3.0,
            y: 0.0,
            z: -16.0,
        },
        2.0,
        materials["ivory"],
    ));
    spheres.push(Sphere::new(
        vec3 {
            x: -1.0,
            y: -1.5,
            z: -12.0,
        },
        2.0,
        materials["glass"],
    ));
    spheres.push(Sphere::new(
        vec3 {
            x: 1.5,
            y: -0.5,
            z: -18.0,
        },
        3.0,
        materials["amber"],
    ));
    spheres.push(Sphere::new(
        vec3 {
            x: 7.0,
            y: 5.0,
            z: -18.0,
        },
        4.0,
        materials["mirror"],
    ));
    let mut lights: Vec<Light> = Vec::new();
    lights.push(Light::new(
        vec3 {
            x: -20.0,
            y: 20.0,
            z: 20.0,
        },
        1.0,
    ));
    lights.push(Light::new(
        vec3 {
            x: 30.0,
            y: 50.0,
            z: -25.0,
        },
        1.0,
    ));
    lights.push(Light::new(
        vec3 {
            x: 30.0,
            y: 20.0,
            z: 30.0,
        },
        1.0,
    ));
    return (spheres, lights);
}

fn get_spheres_lights_2() -> (Vec<Sphere>, Vec<Light>) {
    let materials = get_materials();

    let mut spheres: Vec<Sphere> = Vec::new();
    spheres.push(Sphere::new(
        vec3 {
            x: 0.0,
            y: -1.5,
            z: -20.0,
        },
        2.0,
        materials["mirror"],
    ));
    spheres.push(Sphere::new(
        vec3 {
            x: 4.0,
            y: -1.5,
            z: -16.0,
        },
        2.0,
        materials["sapphire"],
    ));
    spheres.push(Sphere::new(
        vec3 {
            x: -4.0,
            y: -1.5,
            z: -16.0,
        },
        2.0,
        materials["amber"],
    ));

    let mut lights: Vec<Light> = Vec::new();
    // lights.push(Light::new(
    //     vec3 {
    //         x: 0.0,
    //         y: 0.0,
    //         z: -16.0,
    //     },
    //     2.0,
    // ));
    lights.push(Light::new(
        vec3 {
            x: 4.0,
            y: -1.5,
            z: -20.0,
        },
        1.0,
    ));
    lights.push(Light::new(
        vec3 {
            x: -4.0,
            y: -1.5,
            z: -20.0,
        },
        1.0,
    ));
    lights.push(Light::new(
        vec3 {
            x: 0.0,
            y: 30.0,
            z: -16.0,
        },
        0.4,
    ));

    return (spheres, lights);
}

fn get_spheres_lights_3() -> (Vec<Sphere>, Vec<Light>) {
    let materials = get_materials();

    let mut spheres: Vec<Sphere> = Vec::new();
    spheres.push(Sphere::new(
        vec3 {
            x: -4.0,
            y: 6.0,
            z: -22.0,
        },
        8.0,
        materials["sapphire"],
    ));
    spheres.push(Sphere::new(
        vec3 {
            x: 4.0,
            y: -1.0,
            z: -16.0,
        },
        2.0,
        materials["mirror"],
    ));
    spheres.push(Sphere::new(
        vec3 {
            x: 1.0,
            y: -1.0,
            z: -18.0,
        },
        2.0,
        materials["rubber"],
    ));

    let mut lights: Vec<Light> = Vec::new();
    lights.push(Light::new(
        vec3 {
            x: 0.0,
            y: 50.0,
            z: -20.0,
        },
        1.0,
    ));
    lights.push(Light::new(
        vec3 {
            x: -2.0,
            y: -1.0,
            z: -10.0,
        },
        0.4,
    ));

    return (spheres, lights);
}

fn get_spheres_lights_4() -> (Vec<Sphere>, Vec<Light>) {
    let materials = get_materials();

    let mut spheres: Vec<Sphere> = Vec::new();
    spheres.push(Sphere::new(
        vec3 {
            x: 0.0,
            y: 0.0,
            z: -20.0,
        },
        4.0,
        materials["mirror"],
    ));
    spheres.push(Sphere::new(
        vec3 {
            x: 8.0,
            y: 0.0,
            z: -16.0,
        },
        4.0,
        materials["mirror"],
    ));
    spheres.push(Sphere::new(
        vec3 {
            x: -8.0,
            y: 0.0,
            z: -16.0,
        },
        4.0,
        materials["mirror"],
    ));
    spheres.push(Sphere::new(
        vec3 {
            x: 0.0,
            y: -1.5,
            z: -16.0,
        },
        2.0,
        materials["sapphire"],
    ));

    let mut lights: Vec<Light> = Vec::new();
    lights.push(Light::new(
        vec3 {
            x: 0.0,
            y: 30.0,
            z: 26.0,
        },
        0.33,
    ));
    lights.push(Light::new(
        vec3 {
            x: 8.0,
            y: 30.0,
            z: 12.0,
        },
        0.33,
    ));
    lights.push(Light::new(
        vec3 {
            x: -8.0,
            y: 30.0,
            z: 12.0,
        },
        0.33,
    ));

    return (spheres, lights);
}
