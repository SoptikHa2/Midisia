#[cfg(test)]
mod tests {
    use crate::colormatch::ColorKind;
    use crate::colormatch::ColorKind::{BackgroundColor, ForegroundColor};

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
}
