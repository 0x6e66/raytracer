use crate::utils::{light::Light};
use crate::utils::vec3::vec3;
use crate::utils::sphere::Sphere;
use crate::utils::material::Material;
use std::collections::HashMap;

pub fn get_materials() -> HashMap<String, Material> {
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
            50.0,
        ),
    );
    return materials;
}

#[allow(dead_code)]
pub fn get_spheres_lights_1() -> (Vec<Sphere>, Vec<Light>) {
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

#[allow(dead_code)]
pub fn get_spheres_lights_2() -> (Vec<Sphere>, Vec<Light>) {
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

#[allow(dead_code)]
pub fn get_spheres_lights_3() -> (Vec<Sphere>, Vec<Light>) {
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

#[allow(dead_code)]
pub fn get_spheres_lights_4() -> (Vec<Sphere>, Vec<Light>) {
    let materials = get_materials();

    let mut spheres: Vec<Sphere> = Vec::new();
    spheres.push(Sphere::new(
        vec3 {
            x: -10.0,
            y: 0.0,
            z: -20.0,
        },
        4.0,
        materials["mirror"],
    ));
    spheres.push(Sphere::new(
        vec3 {
            x: -3.0,
            y: 0.0,
            z: -20.0,
        },
        3.0,
        materials["amber"],
    ));
    spheres.push(Sphere::new(
        vec3 {
            x: 4.0,
            y: 0.0,
            z: -20.0,
        },
        4.0,
        materials["rubber"],
    ));
    spheres.push(Sphere::new(
        vec3 {
            x: -6.0,
            y: 2.0,
            z: -30.0,
        },
        6.0,
        materials["sapphire"],
    ));
    spheres.push(Sphere::new(
        vec3 {
            x: 6.0,
            y: 2.0,
            z: -30.0,
        },
        6.0,
        materials["glass"],
    ));

    let mut lights: Vec<Light> = Vec::new();
    lights.push(Light::new(
        vec3 {
            x: 4.0,
            y: 30.0,
            z: -20.0,
        },
        1.0,
    ));
    // lights.push(Light::new(
    //     vec3 {
    //         x: -4.0,
    //         y: 30.0,
    //         z: -20.0,
    //     },
    //     1.0,
    // ));
    lights.push(Light::new(
        vec3 {
            x: 0.0,
            y: 30.0,
            z: -16.0,
        },
        1.0,
    ));

    return (spheres, lights);
}