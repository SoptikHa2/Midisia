#[cfg(test)]
mod tests {
    use crate::colormatch::ColorKind;
    use crate::colormatch::ColorKind::{BackgroundColor, ForegroundColor};
    use crate::midi::note_name_to_midi_id;

    #[test]
    fn parse_colors() {
        assert_eq!(
            "b0:0:0".parse::<ColorKind>().unwrap(),
            BackgroundColor((0, 0, 0))
        );
        assert_eq!(
            "0:0:0".parse::<ColorKind>().unwrap(),
            ForegroundColor((0, 0, 0))
        );
        assert_eq!(
            "b255:14:253".parse::<ColorKind>().unwrap(),
            BackgroundColor((255, 14, 253))
        );
        assert_eq!(
            "b000255:000000000000014:000253"
                .parse::<ColorKind>()
                .unwrap(),
            BackgroundColor((255, 14, 253))
        );
        assert_eq!(
            "252:14:255".parse::<ColorKind>().unwrap(),
            ForegroundColor((252, 14, 255))
        );
        assert!("256:0:0".parse::<ColorKind>().is_err());
        assert!("b256:0:0".parse::<ColorKind>().is_err());
        assert!("a0:0:0".parse::<ColorKind>().is_err());
        assert!("-4:0:0".parse::<ColorKind>().is_err());
        assert!("4:18:-24".parse::<ColorKind>().is_err());
    }

    #[test]
    fn note_to_midi() {
        assert_eq!(note_name_to_midi_id("G0"), None);
        assert_eq!(note_name_to_midi_id("A0"), Some(21));
        assert_eq!(note_name_to_midi_id("B0"), Some(23));
        assert_eq!(note_name_to_midi_id("H0"), Some(23));
        assert_eq!(note_name_to_midi_id("C2"), Some(36));
        assert_eq!(note_name_to_midi_id("D2"), Some(38));
        assert_eq!(note_name_to_midi_id("H2"), Some(47));
        assert_eq!(note_name_to_midi_id("C8"), Some(108));
        assert_eq!(note_name_to_midi_id("G6"), Some(91));
        assert_eq!(note_name_to_midi_id("E8"), None);
        assert_eq!(note_name_to_midi_id("A9"), None);
    }
}
