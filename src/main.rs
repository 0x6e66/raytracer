#[allow(non_camel_case_types)]
#[warn(non_snake_case)]
#[allow(non_upper_case_globals)]
mod raytracer;
mod setup;
mod utils;

use clap::{App, Arg, ArgMatches};

fn main() {
    let matches = App::new("Raytracer in rust")
        .version("0.1.0")
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .takes_value(true)
                .default_value("600")
                .validator(is_u32)
                .help("Width of the image"),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .long("height")
                .takes_value(true)
                .default_value("400")
                .validator(is_u32)
                .help("Height of the image"),
        )
        .arg(
            Arg::with_name("output_path")
                .short("o")
                .long("output")
                .takes_value(true)
                .default_value("out")
                .help("Path where to store results (creates folder if not found)"),
        )
        .arg(
            Arg::with_name("versionize")
                .long("versionize")
                .takes_value(false)
                .help("If set output images are being saved with the datetime as prefix"),
        )
        .arg(
            Arg::with_name("max_depth")
                .short("d")
                .long("max-depth")
                .takes_value(true)
                .default_value("4")
                .validator(is_u32)
                .help("Max. depth of reflected rays"),
        )
        .arg(
            Arg::with_name("anti_aliasing")
                .short("a")
                .long("anti-aliasing")
                .takes_value(true)
                .default_value("2")
                .validator(is_u32)
                .help("Set scale of anti aliasing (number of rays per pixel = <argument> ^ 2)"),
        )
        .arg(
            Arg::with_name("occlusion_offset")
                .long("occlusion-offset")
                .takes_value(true)
                .default_value("0.1")
                .validator(is_f32)
                .help("Offset for mitigation occlusion"),
        )
        .arg(
            Arg::with_name("fov")
                .long("fov")
                .takes_value(true)
                .default_value("1.0")
                .validator(is_f32)
                .help("Field of view"),
        )
        .arg(
            Arg::with_name("preset")
                .short("p")
                .long("preset")
                .takes_value(true)
                .default_value("1")
                .validator(is_valid_preset)
                .help("Select preset [1-3]"),
        )
        .get_matches();

    start_raytracer(matches);
}

fn start_raytracer(matches: ArgMatches) {
    let width = matches.value_of("width").unwrap().parse::<u32>().unwrap();
    let height = matches.value_of("height").unwrap().parse::<u32>().unwrap();
    let output_path = matches.value_of("output_path").unwrap().to_string();
    let versionize = matches.is_present("versionize");
    let max_depth = matches
        .value_of("max_depth")
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let anti_aliasing = matches
        .value_of("anti_aliasing")
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let occlusion_offset = matches
        .value_of("occlusion_offset")
        .unwrap()
        .parse::<f32>()
        .unwrap();
    let fov = matches.value_of("fov").unwrap().parse::<f32>().unwrap();
    let preset = matches.value_of("preset").unwrap().parse::<u32>().unwrap();

    let (spheres, lights) = match preset {
        1 => setup::get_spheres_lights_1(),
        2 => setup::get_spheres_lights_2(),
        3 => setup::get_spheres_lights_3(),
        _ => setup::get_spheres_lights_1(),
    };
    let mut tracer = raytracer::Raytracer::new(
        (width, height),
        (10.0, 40.0),
        (53, 108, 160),
        (230, 102, 30),
        -4.0,
        max_depth,
        occlusion_offset,
        anti_aliasing,
        fov,
        output_path,
        spheres,
        lights,
    );

    tracer.start(versionize);
}

fn is_u32(s: String) -> Result<(), String> {
    let parsed = s.parse::<u32>();
    return match parsed {
        Err(_) => Err(String::from("The value is not an unsigned integer")),
        Ok(_) => Ok(()),
    };
}

fn is_f32(s: String) -> Result<(), String> {
    let parsed = s.parse::<f32>();
    return match parsed {
        Err(_) => Err(String::from("The value is not a floating point number")),
        Ok(_) => Ok(()),
    };
}

fn is_valid_preset(s: String) -> Result<(), String> {
    let parsed = s.parse::<u32>();
    match parsed {
        Err(_) => return Err(String::from("The value is not an unsigned integer")),
        Ok(_) => ()
    };

    let value = parsed.unwrap();
    if 1 <= value && value <= 3 {
        return Ok(());
    }
    return Err(String::from("The value is not a valid preset number"));
}
