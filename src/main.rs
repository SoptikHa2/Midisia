mod colormatch;
use crate::colormatch::{ColorKind, ColorMatch};
use anyhow::anyhow;
use std::env;
use std::path::PathBuf;

/// This consists of two parts.
/// One that reads all the cropped png files
/// and generates color matches.
/// And second one that consumes generated
/// color matches and produces MIDI from it.
fn main() {
    let mut args = env::args();
    // Skip 0th argument - binary name
    args.next();
    // Load colors as one long string separated by commas
    let colors = args.next().expect("Missing color argument. Specify list of colors separated by ',' in format R:G:B. Prefix background colors with letter 'b'.");
    let predefined_colors: Vec<ColorKind> = colors
        .split(',')
        .map(|color_part| color_part.parse())
        // TODO: Skipping unrecognized colors
        .filter(|result| result.is_ok())
        .map(|result| result.unwrap())
        .collect();
    if predefined_colors.len() == 0 {
        eprintln!("Failed to load any colors. Bailing out.");
        return;
    }
    // Load image files
    let mut image_paths: Vec<PathBuf> = Vec::new();
    for image_path in args {
        let path: PathBuf = image_path.parse().expect(&format!(
            "Image path {} could not be turned into valid system UTF-8 path.",
            image_path
        ));
        image_paths.push(path);
    }

    let color_matches = recognize_colors_in_files(&predefined_colors, &image_paths);
}

fn recognize_colors_in_files<'a>(
    predefined_colors: &'a Vec<ColorKind>,
    image_paths: &'a Vec<PathBuf>,
) -> Vec<ColorMatch<'a>> {
    image_paths
        .iter()
        .map(|path| ColorMatch::load_from_file(path, predefined_colors))
        .filter(|res| res.is_ok())
        .map(|res| res.unwrap())
        .collect()
}
