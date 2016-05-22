use std::net;

pub struct Anybar {
    /// The UDP Port the Anybar is connected to
    pub port: u16,
    /// The last color that has been set
    pub color: Option<Color>,
}

pub enum Color {
    White,
    Red,
    Orange,
    Yellow,
    Green,
    Cyan,
    Blue,
    Purple,
    Black,
    Question,
    Exclamation,
}

impl Anybar {
    pub fn new(port: u16) -> Anybar {
        //TODO check if port is valid
        Anybar{port:port, color:None}
    }

    fn parse_color(color: Color) -> Vec<u8> {
        use Color::*;
        let col = match color {
            White       => "white",
            Red         => "red",
            Orange      => "orange",
            Yellow      => "yellow",
            Green       => "green",
            Cyan        => "cyan",
            Blue        => "blue",
            Purple      => "purple",
            Black       => "black",
            Question    => "question",
            Exclamation => "exclamation",
        };

        let mut parsed: Vec<u8> = Vec::new();
        parsed.extend(col.as_bytes()
                         .iter());
        parsed
    }

    pub fn set_color(&self, color: Color) {
        let message = Anybar::parse_color(color);
        // TODO
    }
}

impl Default for Anybar {
    fn default() -> Anybar {
        Anybar{port:1738, color:None}
    }
}
