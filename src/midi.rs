use std::convert::TryInto;

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
