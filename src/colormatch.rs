use anyhow::anyhow;
use anyhow::Result;
use png::{ColorType, Decoder};
use std::convert::TryInto;
use std::fs::File;
use std::path;
use std::str::FromStr;

pub enum ColorKind {
    BackgroundColor((u8, u8, u8)),
    ForegroundColor((u8, u8, u8)),
}
/// Expected format: bR:G:B for background color or
/// R:G:B for foreground color
impl FromStr for ColorKind {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut str_to_load = s;
        if s.chars().next() == Some('b') {
            str_to_load = &s[1..];
        }
        let colon_idx = str_to_load.find(':');
        if let Some(colon_idx) = colon_idx {
            let red: u8 = str_to_load[0..colon_idx].parse()?;
            let second_colon_idx = str_to_load[colon_idx + 1..].find(':');
            if let Some(second_colon_idx) = second_colon_idx {
                let green: u8 = str_to_load[colon_idx + 1..second_colon_idx].parse()?;
                let blue: u8 = str_to_load[second_colon_idx + 1..].parse()?;
                return Ok(match s.chars().next() {
                    Some(x) if x == 'b' => ColorKind::BackgroundColor((red, green, blue)),
                    _ => ColorKind::ForegroundColor((red, green, blue)),
                });
            }
        }

        Err(anyhow!("Bad string format"))
    }
}
impl ColorKind {
    fn get_distance_to_color(&self, color_to_compare: (u8, u8, u8)) -> i16 {
        let self_color: (u8, u8, u8) = match *self {
            Self::BackgroundColor(c) => c,
            Self::ForegroundColor(c) => c,
        };
        let (r1, g1, b1): (i16, i16, i16) = (
            self_color.0.into(),
            self_color.1.into(),
            self_color.2.into(),
        );
        let (r2, g2, b2): (i16, i16, i16) = (
            color_to_compare.0.into(),
            color_to_compare.1.into(),
            color_to_compare.2.into(),
        );

        (r1 - r2).pow(2) + (g1 - g2).pow(2) + (b1 - b2).pow(2)
    }
}

pub struct ColorMatch<'a> {
    data: Vec<&'a ColorKind>,
}
impl<'a> ColorMatch<'a> {
    fn load_from_file(path: &path::PathBuf, color_collection: &'a Vec<ColorKind>) -> Result<Self> {
        if color_collection.len() == 0 {
            return Err(anyhow!("Color collection is empty."));
        }

        let loaded_image = load_image(path)?;

        // For each column, average the colors
        let average_colors_per_column = loaded_image
            .data
            .chunks(loaded_image.width.try_into()?)
            .map(|chunk| averageColors(chunk));

        // For each column, return corresponding matched color
        let average_colors_per_column_with_paired_distances_to_colors: Vec<Vec<i16>> =
            average_colors_per_column
                .map(|color_in_column: (u8, u8, u8)| {
                    color_collection
                        .iter()
                        .map(|color_in_collection| {
                            color_in_collection.get_distance_to_color(color_in_column)
                        })
                        .collect::<Vec<i16>>()
                })
                .collect();

        let chosen_index_for_each_column: Vec<usize> =
            average_colors_per_column_with_paired_distances_to_colors
                .iter()
                .map(|distances| distances.iter().enumerate())
                .map(|distances_with_indexes| {
                    distances_with_indexes
                        .min_by(|(idx, distance), (idx2, distance2)| distance.cmp(distance2))
                        .unwrap()
                        .0
                })
                .collect();

        let chosen_colors: Vec<&'a ColorKind> = chosen_index_for_each_column
            .iter()
            .map(|idx| color_collection.get(*idx).unwrap())
            .collect();

        Ok(ColorMatch {
            data: chosen_colors,
        })
    }
}

struct LoadedImageData {
    width: u32,
    height: u32,
    data: Vec<(u8, u8, u8)>,
}
impl LoadedImageData {}

fn load_image(path: &path::PathBuf) -> Result<LoadedImageData> {
    let decoder = Decoder::new(File::open(path)?);
    let (info, mut reader) = decoder.read_info()?;
    let mut img_data = vec![0; info.buffer_size()];
    reader.next_frame(&mut img_data)?;

    let data = match info.color_type {
        ColorType::RGB => img_data,
        ColorType::RGBA => img_data
            .iter()
            .enumerate()
            .filter(|(idx, item)| idx % 4 != 0 || *idx == 0)
            .map(|(idx, item)| *item)
            .collect(),
        ColorType::Grayscale => {
            let mut vec = Vec::with_capacity(img_data.len() * 3);
            for g in img_data {
                vec.extend([g, g, g].iter().cloned())
            }
            vec
        }
        ColorType::GrayscaleAlpha => {
            let mut vec = Vec::with_capacity(img_data.len() * 3);
            for ga in img_data.chunks(2) {
                let g = ga[0];
                let _a = ga[1];
                vec.extend([g, g, g].iter().cloned())
            }
            vec
        }
        _ => unreachable!("Encountered unknown color type"),
    };

    let data = data
        .chunks(3)
        .map(|chunk| (chunk[0], chunk[1], chunk[2]))
        .collect();

    Ok(LoadedImageData {
        width: info.width,
        height: info.height,
        data,
    })
}

fn averageColors(colors: &[(u8, u8, u8)]) -> (u8, u8, u8) {
    let mut r: u32 = 0;
    let mut g: u32 = 0;
    let mut b: u32 = 0;
    for color in colors {
        let r1: u32 = (*color).0.into();
        let g1: u32 = (*color).1.into();
        let b1: u32 = (*color).2.into();
        r += r1;
        g += g1;
        b += b1;
    }

    let div: u32 = colors.len().try_into().unwrap();
    (
        (r / div).try_into().unwrap(),
        (g / div).try_into().unwrap(),
        (b / div).try_into().unwrap(),
    )
}
