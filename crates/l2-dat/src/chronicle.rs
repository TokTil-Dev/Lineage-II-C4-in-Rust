use std::sync::OnceLock;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Chronicle {
    #[default]
    C4,
    Interlude,
    HighFive,
}

static OVERRIDE: OnceLock<Chronicle> = OnceLock::new();

impl Chronicle {
    pub fn parse(text: &str) -> Option<Self> {
        match text.trim().to_ascii_lowercase().as_str() {
            "c4" => Some(Self::C4),
            "interlude" | "c6" => Some(Self::Interlude),
            "h5" | "highfive" | "high-five" => Some(Self::HighFive),
            _ => None,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::C4 => "c4",
            Self::Interlude => "interlude",
            Self::HighFive => "h5",
        }
    }

    pub fn set_override(chronicle: Self) {
        let _ = OVERRIDE.set(chronicle);
    }

    pub fn forced() -> Option<Self> {
        OVERRIDE.get().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_command_line_names() {
        assert_eq!(Chronicle::parse("c4"), Some(Chronicle::C4));
        assert_eq!(Chronicle::parse(" Interlude "), Some(Chronicle::Interlude));
        assert_eq!(Chronicle::parse("h5"), Some(Chronicle::HighFive));
        assert_eq!(Chronicle::parse("High-Five"), Some(Chronicle::HighFive));
        assert_eq!(Chronicle::parse("highfive"), Some(Chronicle::HighFive));
        assert_eq!(Chronicle::parse("gracia"), None);
        assert_eq!(Chronicle::HighFive.name(), "h5");
        assert_eq!(Chronicle::default(), Chronicle::C4);
    }
}
