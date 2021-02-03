mod colormatch;
mod midi;
mod tests;

use crate::colormatch::{ColorKind, ColorMatch};
use crate::midi::note_name_to_midi_id;
use std::env;
use std::path::PathBuf;

/// This consists of two parts.
/// One that reads all the cropped png files
/// and generates color matches.
/// And second one that consumes generated
/// color matches and produces MIDI from it.
///
/// Arguments:
/// - Colors separated by commas (b0:0:0,255:0:0)
/// - Leftmost key (A0 .. C8, C4 is the "default" C)
/// - Rightmost key
/// - Filenames of images to process (repeatable)
///
/// Midi numbers:
/// C2          | 36
/// C3          | 48
/// C4 (main C) | 60
/// D4          | 62
/// C5          | 72
/// C6          | 84
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
    // Load midi
    let leftmost_midi = note_name_to_midi_id(&args.next().expect("Expected leftmost midi key ID"))
        .expect("Failed to parse leftmost key");
    let rightmost_midi =
        note_name_to_midi_id(&args.next().expect("Expected rightmost midi key ID"))
            .expect("Failed to parse rightmost key");
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

    debug_print_color_recognition_to_netpbm(&color_matches);
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

fn debug_print_color_recognition_to_netpbm(color_matches: &Vec<ColorMatch>) {
    println!("P1");
    let height = color_matches.len();
    let width = color_matches.first().unwrap().data.len();
    println!("{} {}", width, height);
    for line in color_matches {
        let line = &line.data;
        for record in line {
            print!(
                "{}",
                match **record {
                    ColorKind::ForegroundColor(_) => 1,
                    ColorKind::BackgroundColor(_) => 0,
                }
            );
        }
        println!();
    }
}
