use crate::colormatch::{ColorKind, ColorMatch};
use apres::MIDIEvent::{NoteOff, NoteOn};
use apres::MIDI;
use std::convert::TryInto;
use std::path::PathBuf;

pub fn create_midi_file(
    output_filename: PathBuf,
    leftmost_key: usize,
    rightmost_key: usize,
    pressed_keys_pixelized: &Vec<ColorMatch>,
) {
    let mut midi = MIDI::new();

    println!("{}", pressed_keys_pixelized.len());
    let mut pressed_keys: Vec<bool> = (leftmost_key..(rightmost_key + 1)).map(|_| false).collect();
    let mut tick_midi: usize = 0;
    for tick in pressed_keys_pixelized {
        let pixels_this_tick = &(*tick).data;

        let determined_pressed_midi_ids = get_corresponding_notes(
            pixels_this_tick
                .iter()
                .map(|x| match **x {
                    ColorKind::ForegroundColor(_) => true,
                    ColorKind::BackgroundColor(_) => false,
                })
                .collect(),
            leftmost_key,
            rightmost_key,
        );

        // Todo: speed up lookup by always having all keys at it's desired index at determined_pressed_midis_ids
        let pressed_keys_this_tick: Vec<bool> = (leftmost_key..(rightmost_key + 1))
            .map(|idx| determined_pressed_midi_ids.contains(&(idx + leftmost_key)))
            .collect();
        let debug_filter: Vec<(usize, &bool)> = pressed_keys_this_tick
            .iter()
            .enumerate()
            .filter(|(idx, x)| **x)
            .collect();

        for (index, key) in pressed_keys.iter().enumerate() {
            let key_this_tick = pressed_keys_this_tick[index];
            if key_this_tick != *key {
                match key_this_tick {
                    false => {
                        // Key was released
                        midi.insert_event(
                            0,
                            tick_midi,
                            NoteOff(0, (index + leftmost_key) as u8, 100),
                        );
                        println!("{}\t\t-\t{}", tick_midi, (index + leftmost_key));
                    }
                    true => {
                        // Key was pressed
                        midi.insert_event(
                            0,
                            tick_midi,
                            NoteOn(0, (index + leftmost_key) as u8, 100),
                        );
                        println!("{}\t\t+\t{}", tick_midi, (index + leftmost_key));
                    }
                }
            }
        }

        tick_midi += 120;
    }

    // Save it to a file
    midi.save(output_filename.to_str().unwrap());
}

pub fn note_name_to_midi_id(n: &str) -> Option<usize> {
    // Expected format: Letter (C,D,E,F,G,A,(H/B))
    // Number (0-8)
    // eg F2 (bF2, #F2)
    // Range: A0, H0, C1 .. C8
    let mut midi_id = 21; // A0
    let mut name = n;

    if n.chars().next()? == '#' {
        midi_id += 1;
        name = &n[1..];
    }
    if n.chars().next()? == 'b' {
        midi_id -= 1;
        name = &n[1..];
    }

    let name = name.trim();
    if name.len() != 2 {
        return None;
    }
    let mut letter = name.chars().next()?;
    let number: usize = name[1..].parse().ok()?;

    // Perform sanity check
    if letter == 'H' {
        letter = 'B';
    }

    if letter < 'A' || letter > 'G' {
        return None;
    }

    if number > 8 || (letter != 'C' && number > 7) || (letter >= 'C' && number == 0) {
        return None;
    }

    midi_id += midi_step_up_from_A(letter);
    midi_id += number as isize * 12;

    Some(midi_id.try_into().ok()?)
}

fn midi_step_up_from_A(letter: char) -> isize {
    match letter {
        'A' => 0,
        'B' => 2,
        'C' => 3 - 12,
        'D' => 5 - 12,
        'E' => 7 - 12,
        'F' => 8 - 12,
        'G' => 10 - 12,
        _ => unimplemented!(),
    }
}

fn get_corresponding_notes(is_pressed: Vec<bool>, left: usize, right: usize) -> Vec<usize> {
    const WHOLE_TONE_LEN: usize = 2;
    const HALF_TONE_LEN: usize = 1;
    /// Offset from both sides to the looking-into color interval (in pixels)
    const IGNORE_OFFSET: usize = 0;

    // Which keys in allowed interval and full and halftones
    let key_sizes = (left..(right + 1)).map(|key| is_key_full_tone(key));

    // Lengths of keys in interval, as specified by constants above
    let total_distances = key_sizes.map(|key_type| match key_type {
        true => WHOLE_TONE_LEN,
        false => HALF_TONE_LEN,
    });

    let pixels = is_pressed.len();
    let total_distance: usize = total_distances.sum();

    let pixels_per_halftone: usize = pixels / total_distance;

    let mut result: Vec<usize> = Vec::new();
    let mut current_pixel_id: usize = 0;
    for note in left..(right + 1) {
        if is_pressed
            .iter()
            .skip(current_pixel_id + IGNORE_OFFSET)
            .take(
                match is_key_full_tone(note) {
                    true => WHOLE_TONE_LEN * pixels_per_halftone,
                    false => HALF_TONE_LEN * pixels_per_halftone,
                } - IGNORE_OFFSET,
            )
            .all(|x| *x)
        {
            result.push(note);
        }
        current_pixel_id += 1;
    }

    result
}

fn is_key_full_tone(midi_id: usize) -> bool {
    if midi_id < 21 {
        // 12 is one octave in midi IDs
        return is_key_full_tone(midi_id + 12);
    }
    if midi_id > 32 {
        return is_key_full_tone(midi_id - 12);
    }
    match midi_id {
        21 | 23 | 24 | 26 | 28 | 29 | 31 => true,
        22 | 25 | 27 | 30 | 32 => false,
        _ => unreachable!(),
    }
}
