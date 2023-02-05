#[allow(non_camel_case_types)]
#[warn(non_snake_case)]
#[allow(non_upper_case_globals)]
mod raytracer;
mod utils;
mod setup;

use clap::{App, Arg, ArgMatches };

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
                .help("Width of the image (defalut: 600)"),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .long("height")
                .takes_value(true)
                .default_value("400")
                .validator(is_u32)
                .help("Height of the image (defalut: 400)"),
        )
        .arg(
            Arg::with_name("output_path")
                .short("o")
                .long("output")
                .takes_value(true)
                .default_value("out")
                .help("Path where to store results (defalut: 'out'; creates folder if not found)"),
        )
        .arg(
            Arg::with_name("versionize")
                .long("versionize")
                .takes_value(false)
                .help("If set output images are being saved with the datetime as prefix"),
        )
        .get_matches();

    start_raytracer(matches);
}

fn start_raytracer(matches: ArgMatches) {
    let width = matches.value_of("width").unwrap().parse::<u32>().unwrap();
    let height = matches.value_of("height").unwrap().parse::<u32>().unwrap();
    let output_path = matches.value_of("output_path").unwrap().to_string();
    let versionize = matches.is_present("versionize");

    let (spheres, lights) = setup::get_spheres_lights_4();
    let mut tracer = raytracer::Raytracer::new(
        (width, height),
        (10.0, 40.0),
        (53, 108, 160),
        (230, 102, 30),
        -4.0,
        4,
        0.1,
        2,
        1.0,
        output_path,
        spheres,
        lights,
    );

    tracer.start(versionize);
}

fn is_u32(s: String) -> Result<(), String> {
    let parsed = s.parse::<u32>();
    return match parsed {
        Err(_) => Err(String::from(
            "The value is not an unsigned integer",
        )),
        Ok(_) => Ok(())
    };
}