use clap::{App, AppSettings, Arg, SubCommand};
use std::path::Path;

fn main() {
    let commands = vec![
        ("blur", "Blur an image"),
        ("brighten", "Brighten an image"),
        ("crop", "Crop an image"),
        ("rotate", "Rotate an image"),
        ("invert", "Invert an image"),
        ("grayscale", "Grayscale an image"),
        ("generate", "Generante a random image"),
        ("fractal", "Create a fractal image"),
    ];

    let matches = App::new("imgp")
        .about("A image processing tool")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommands(
            commands
                .iter()
                .map(|(command_name, command_description)| {
                    SubCommand::with_name(command_name)
                        .about(command_description.clone())
                        .arg(
                            Arg::with_name("path")
                                .short("p")
                                .help("file path")
                                .takes_value(true)
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("ouput")
                                .short("o")
                                .help("output file path")
                                .takes_value(true),
                        )
                })
                .collect::<Vec<_>>()
        )
        .get_matches();

    match matches.subcommand() {
        ("blur", Some(sub)) => {
            let path = sub.value_of("path").unwrap().to_string();
            let output = if sub.occurrences_of("output") > 0 {
                sub.value_of("output").unwrap().to_string()
            } else {
                let path = Path::new(&path);
                let extension = path.extension().unwrap().to_str().unwrap();
                let filename = path.file_stem().unwrap().to_str().unwrap();
                format!("temp/{}_blurred.{}", filename, extension)
            };
            blur(path, output);
        }
        ("brighten", Some(sub)) => {
            let path = sub.value_of("path").unwrap().to_string();
            let output = if sub.occurrences_of("output") > 0 {
                sub.value_of("output").unwrap().to_string()
            } else {
                let path = Path::new(&path);
                let extension = path.extension().unwrap().to_str().unwrap();
                let filename = path.file_stem().unwrap().to_str().unwrap();
                format!("temp/{}_brightened.{}", filename, extension)
            };
            brighten(path, output);
        }
        ("crop", Some(sub)) => {
            let path = sub.value_of("path").unwrap().to_string();
            let output = if sub.occurrences_of("output") > 0 {
                sub.value_of("output").unwrap().to_string()
            } else {
                let path = Path::new(&path);
                let extension = path.extension().unwrap().to_str().unwrap();
                let filename = path.file_stem().unwrap().to_str().unwrap();
                format!("temp/{}_cropped.{}", filename, extension)
            };
            crop(path, output);
        }
        ("rotate", Some(sub)) => {
            let path = sub.value_of("path").unwrap().to_string();
            let output = if sub.occurrences_of("output") > 0 {
                sub.value_of("output").unwrap().to_string()
            } else {
                let path = Path::new(&path);
                let extension = path.extension().unwrap().to_str().unwrap();
                let filename = path.file_stem().unwrap().to_str().unwrap();
                format!("temp/{}_rotated.{}", filename, extension)
            };
            rotate(path, output);
        }
        ("invert", Some(sub)) => {
            let path = sub.value_of("path").unwrap().to_string();
            let output = if sub.occurrences_of("output") > 0 {
                sub.value_of("output").unwrap().to_string()
            } else {
                let path = Path::new(&path);
                let extension = path.extension().unwrap().to_str().unwrap();
                let filename = path.file_stem().unwrap().to_str().unwrap();
                format!("temp/{}_inverted.{}", filename, extension)
            };
            invert(path, output);
        }
        ("grayscale", Some(sub)) => {
            let path = sub.value_of("path").unwrap().to_string();
            let output = if sub.occurrences_of("output") > 0 {
                sub.value_of("output").unwrap().to_string()
            } else {
                let path = Path::new(&path);
                let extension = path.extension().unwrap().to_str().unwrap();
                let filename = path.file_stem().unwrap().to_str().unwrap();
                format!("temp/{}_grayscaled.{}", filename, extension)
            };
            grayscale(path, output);
        }
        ("generate", Some(sub)) => {
            let output = if sub.occurrences_of("output") > 0 {
                sub.value_of("output").unwrap().to_string()
            } else {
                "temp/image_generated.png".to_string()
            };
            generate(output);
        }
        ("fractal", Some(sub)) => {
            let output = if sub.occurrences_of("output") > 0 {
                sub.value_of("output").unwrap().to_string()
            } else {
                "temp/image_fractal.png".to_string()
            };
            fractal(output);
        }
        _ => panic!("Invalid command")
    }
}

fn blur(infile: String, outfile: String) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.blur(2.0);
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn brighten(infile: String, outfile: String) {
    let img = image::open(infile).expect("Failed to open INFILE");
    let brightened_img = img.brighten(50);
    brightened_img
        .save(outfile)
        .expect("Failed writing OUTFILE.");
}

fn crop(infile: String, outfile: String) {
    let img = image::open(infile).expect("Failed to open INFILE");
    let cropped_img = img.crop_imm(0, 1, 10, 10);
    cropped_img.save(outfile).expect("Failed writing OUTFILE");
}

fn rotate(infile: String, outfile: String) {
    let img = image::open(infile).expect("Failed to open INFILE");
    let rotated_img = img.rotate90();
    rotated_img.save(outfile).expect("Failed writing OUTFILE");
}

fn invert(infile: String, outfile: String) {
    let mut img = image::open(infile).expect("Failed to open INFILE");
    img.invert();
    img.save(outfile).expect("Failed writing OUTFILE");
}

fn grayscale(infile: String, outfile: String) {
    let img = image::open(infile).expect("Failed to open INFILE");
    let grayscaled_img = img.grayscale();
    grayscaled_img
        .save(outfile)
        .expect("Failed writing OUTFILE");
}

fn generate(outfile: String) {
    let width = 800;
    let height = 800;

    let mut img_buff = image::ImageBuffer::new(width, height);

    for (_x, _y, pixel) in img_buff.enumerate_pixels_mut() {
        let red = rand::random::<u8>();
        let blue = rand::random::<u8>();
        let green = rand::random::<u8>();
        *pixel = image::Rgb([red, blue, green]);
    }

    img_buff.save(outfile).unwrap();
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut img_buff = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in img_buff.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    img_buff.save(outfile).unwrap();
}