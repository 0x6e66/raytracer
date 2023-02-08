#[allow(non_camel_case_types)]
#[warn(non_snake_case)]
#[allow(non_upper_case_globals)]
mod raytracer;
mod setup;
mod utils;

use clap::{value_parser, Arg, ArgMatches, Command};

fn main() {
    let matches: ArgMatches = get_matches();
    start_raytracer(matches);
}

fn get_matches() -> ArgMatches {
    let matches = Command::new("trace")
        .version("0.1.0")
        .arg(Arg::new("width")
                .short('w')
                .long("width")
                .default_value("600")
                .value_parser(value_parser!(u32))
                .help("Width of the image")
        )
        .arg(Arg::new("height")
                // .short('h')
                .long("height")
                .default_value("400")
                .value_parser(value_parser!(u32))
                .help("Height of the image")
        )
        .arg(Arg::new("output_path")
                .short('o')
                .long("output")
                .default_value("out")
                .help("Path where to store results (creates folder if not found)")
        )
        .arg(Arg::new("versionize")
                .long("versionize")
                .action(clap::ArgAction::SetTrue)
                .help("If set output images are being saved with the datetime as prefix")
        )
        .arg(Arg::new("max_depth")
                .short('d')
                .long("max-depth")
                .default_value("4")
                .value_parser(value_parser!(u32))
                .help("Max. depth of reflected rays")
        )
        .arg(Arg::new("anti_aliasing")
                .short('a')
                .long("anti-aliasing")
                .default_value("2")
                .value_parser(value_parser!(u32))
                .help("Set scale of anti aliasing (number of rays per pixel = <argument> ^ 2)")
        )
        .arg(Arg::new("occlusion_offset")
                .long("occlusion-offset")
                .default_value("0.1")
                .value_parser(value_parser!(f32))
                .help("Offset for mitigation occlusion")
        )
        .arg(Arg::new("fov")
                .long("fov")
                .default_value("1.0")
                .value_parser(value_parser!(f32))
                .help("Field of view")
        )
        .arg(Arg::new("preset")
                .short('p')
                .long("preset")
                .default_value("1")
                .value_parser(value_parser!(u32).range(1..4))
                .help("Select preset [1-3]")
        )
        .arg(Arg::new("look_at")
                .long("look-at")
                .action(clap::ArgAction::SetTrue)
                .help("Select weather or not you want to enable the look-at transformation to the camera (requires option 'camera_pos' and 'look_at_pos')"),
        )
        .arg(Arg::new("camera_pos")
                .long("camera-pos")
                .default_value("0,0,0")
                .value_delimiter(',')
                .value_parser(value_parser!(i32))
                .allow_hyphen_values(true)
                .help("Set position of camera")
        )
        .arg(Arg::new("look_at_pos")
                .long("look-at-pos")
                .default_value("0,-4,-20")
                .value_delimiter(',')
                .value_parser(value_parser!(i32))
                .allow_hyphen_values(true)
                .help("Set position of point to look at")
        )
        .get_matches();
    return matches;
}

fn start_raytracer(matches: ArgMatches) {
    let width = *matches.get_one::<u32>("width").unwrap();
    let height = *matches.get_one::<u32>("height").unwrap();
    let output_path = matches.get_one::<String>("output_path").unwrap();
    let versionize = matches.get_flag("versionize");
    let max_depth = *matches.get_one::<u32>("max_depth").unwrap();
    let anti_aliasing = *matches.get_one::<u32>("anti_aliasing").unwrap();
    let occlusion_offset = *matches.get_one::<f32>("occlusion_offset").unwrap();
    let fov = *matches.get_one::<f32>("fov").unwrap();
    let preset = *matches.get_one::<u32>("preset").unwrap();
    let look_at = matches.get_flag("look_at");

    let camera_pos_tmp: Vec<Vec<&i32>> = matches
        .get_occurrences("camera_pos")
        .unwrap()
        .map(Iterator::collect)
        .collect();
    let camera_pos: (f32, f32, f32) = (
        *camera_pos_tmp[0][0] as f32,
        *camera_pos_tmp[0][1] as f32,
        *camera_pos_tmp[0][2] as f32,
    );

    let look_at_pos_tmp: Vec<Vec<&i32>> = matches
        .get_occurrences("look_at_pos")
        .unwrap()
        .map(Iterator::collect)
        .collect();
    let look_at_point: (f32, f32, f32) = (
        *look_at_pos_tmp[0][0] as f32,
        *look_at_pos_tmp[0][1] as f32,
        *look_at_pos_tmp[0][2] as f32,
    );

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
        camera_pos,
        look_at_point,
        look_at,
        -4.0,
        max_depth,
        occlusion_offset,
        anti_aliasing,
        fov,
        output_path.clone(),
        spheres,
        lights,
    );

    tracer.start(versionize);
}
