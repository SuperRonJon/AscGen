use std::{fs::{self}, path::PathBuf};

use clap::Parser;
use image::{imageops::FilterType::Nearest, DynamicImage, GenericImageView, ImageError, ImageReader, Pixel, Rgba};

/// Ascii Art Generator
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the image file
    #[arg()]
    filename: String,
    /// Invert the image color brightnesses
    #[arg(short, long, default_value_t = false)]
    invert: bool,
    /// Width Scaling Factor
    #[arg(short='w', long="width", default_value_t = -1.0)]
    width_scaling: f64,
    /// Height Scaling Factor
    #[arg(short='t', long="height", default_value_t = -1.0)]
    height_scaling: f64,
    /// Even scaling factor for both height & width
    #[arg(short, long, default_value_t = 1.0)]
    scaling: f64,
    /// Output file name. If supplied, outputs to file rather than to console.
    #[arg(short='f', long="file", default_value_t = String::new())]
    out_file: String
}

fn main() {
    let args = Args::parse();
    let filename = args.filename;
    let invert = args.invert;
    let width_scaling = args.width_scaling;
    let height_scaling = args.height_scaling;
    let scaling = args.scaling;
    let out_file = args.out_file;

    let width_given = width_scaling > 0.0;
    let height_given = height_scaling > 0.0;
    let even_scaling = !width_given && !height_given;

    let ascii_characters = ['@', '%', '#', '*', '+', '=', '-', ':', '.', ' '];

    let img_result = get_image(&filename);
    let img = match img_result {
        Ok(file) => file,
        Err(error) => {
            println!("Error opening image file \"{}\": {}", filename, error.to_string());
            return;
        }
    };

    if !even_scaling && (width_scaling <= 0.0 || height_scaling <= 0.0) {
        println!("Invalid scaling parameters.\nIf not using equivalent scaling for height and width (-s) both height and width must be supplied and greater than 0.");
        return;
    }

    let mut w_scale = width_scaling;
    let mut h_scale = height_scaling;
    if even_scaling {
        w_scale = scaling;
        h_scale = scaling;
    }
    
    let scaled_image = scale_image(img, w_scale, h_scale);
    let str = image_to_string(&scaled_image, &ascii_characters, invert);
    if out_file.is_empty() {
        print!("{}", str);
    } else {
        write_to_file(&out_file, &str);
        println!("Ascii art written to {}", out_file);
    }
}

fn image_to_string(img: &DynamicImage, character_set: &[char; 10], invert: bool) -> String {
    let char_count = ((img.width() * img.height()) + img.height()) as usize;
    let mut result = String::with_capacity(char_count);
    let mut characters = character_set.clone();
    if invert {
        characters.reverse();
    }

    for pixel in img.pixels() {
        let brightness = get_brightness_value(&pixel.2);
        let char_index = (brightness / (255.1 / characters.len() as f64)).floor() as usize;

        let brightness_char = characters[char_index];
        result.push(brightness_char);
        if pixel.0 == img.width() - 1 {
            result.push('\n');
        }
    }
    return result;
}

fn get_image(filename: &String) -> Result<DynamicImage, ImageError> {
    return ImageReader::open(filename)?.decode();
}

fn get_brightness_value(p: &Rgba<u8>) -> f64 {
    let pr = 0.299;
    let pg = 0.587;
    let pb = 0.114;

    let channels = p.channels();
    let red = channels[0] as i32;
    let green = channels[1] as i32;
    let blue = channels[2] as i32;

    return ((pr * (red.pow(2)) as f64) + (pg * (green.pow(2)) as f64) + (pb * (blue.pow(2)) as f64)).sqrt();
}

fn scale_image(img: DynamicImage, width_factor: f64, height_factor: f64) -> DynamicImage {
    let new_width = (img.width() as f64 * width_factor) as u32;
    let new_height = (img.height() as f64 * height_factor) as u32;

    return img.resize_exact(new_width, new_height, Nearest);
}

fn write_to_file(output_file: &String, art_string: &String) {
    let filepath = PathBuf::from(output_file);
    let out_dir = filepath.parent().unwrap();
    if ! fs::exists(out_dir).expect("Error parsing output filepath...") {
        let create_result = fs::create_dir_all(out_dir);
        match create_result {
            Ok(()) => {},
            Err(error) => {
                println!("Error creating output directory: {}", error);
                return;
            }
        }
    }
    fs::write(filepath.as_path(), art_string).expect("Error writing to file...");
}