//! This is a small crate for the [AnyBar](https://github.com/tonsky/AnyBar).
//!
//! # Examples
//! The functioning of this crate is pretty simple. These are the most use cases:
//!
//! ## Using the default port
//! ```
//! # use anybar::*;
//! // create a new AnyBar instance connected to the default port
//! let mut bar = Anybar::default();
//!
//! // set the color
//! bar.set_color(Color::Red);
//! ```
//!
//! ## Using a separate port
//! ```
//! # use anybar::*;
//! // Anybar::new() takes the Anybar port as parameter
//! let mut custom_bar = Anybar::new(1708);
//! custom_bar.set_color(Color::Exclamation);
//! ```
//!
//! ## Additional information
//! ```
//! # use anybar::*;
//! let mut bar = Anybar::default();
//!
//! // after instantiation, the last color is None
//! assert!(bar.color.is_none());
//!
//! bar.set_color(Color::Red);
//! // the last color now contains a value
//! // assert_eq!(bar.color.unwrap(), Color::Red);
//! ```
//!
//! Note that the value of `bar.color` does not necessarily represent the real color
//! displayed at the moment, depending on whether you have something or someone else messing
//! with your AnyBar simultaneously.
//!
//! # Note
//! The AnyBar itself does not provide any information on whether the sent command was executed
//! successfully, so the lib will only panic if it was not able to bind a UDP socket.
//!

use std::net;

/// The Anybar handle.
pub struct Anybar {
    /// The UDP Port the Anybar is connected to
    pub port: u16,
    /// The last color that has been set.
    ///
    /// When no color has been set yet, color is `None`.
    pub color: Option<Color>,
}

/// The different colors supported by AnyBar.
pub enum Color {
    /// White dot
    White,
    /// Red dot
    Red,
    /// Orange dot
    Orange,
    /// Yellow dot
    Yellow,
    /// Green dot
    Green,
    /// Cyan dot
    Cyan,
    /// Blue dot
    Blue,
    /// Purple dot
    Purple,
    /// Black dot; Has a white frame in _dark_ mode
    Black,
    /// Question mark
    Question,
    /// White exclamation mark on red ground
    Exclamation,
}

impl Anybar {
    /// Create a new Anybar instance, connected to the given UDP port.
    ///
    /// `port` may be any port between 0 and 6553.
    ///
    /// # Panics
    /// Panics if the port has a non-valid value outside of the range between 0 and 6553.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anybar::*;
    /// let custom_bar = Anybar::new(1708);
    ///
    /// assert_eq!(custom_bar.port, 1708);
    /// ```
    pub fn new(port: u16) -> Anybar {
        if port > 6553 {
            panic!("The port {} is not between 0 and 6553!", port);
        }
        Anybar{port:port, color:None}
    }

    fn parse_color(color: &Color) -> Vec<u8> {
        use Color::*;
        let col = match *color {
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

    fn socket(ip: &str, port: u16) -> net::UdpSocket {
        match net::UdpSocket::bind((ip, port)) {
            Ok(sock) => sock,
            Err(err) => panic!("Could not bind: {}", err),
        }
    }

    /// Set a new color.
    ///
    /// # Panics
    /// Panics if the UDP socket can not be bound.
    pub fn set_color(&mut self, color: Color) {
        let message = Anybar::parse_color(&color);

        let socket = Anybar::socket("127.0.0.1", 0);
        let _ = socket.send_to(&message, ("127.0.0.1", self.port));
        drop(socket);

        self.color = Some(color);
    }

    /// Sends the quit signal to the Anybar and takes ownership of the object.
    ///
    /// _This is an experimental feature of AnyBar._ Since the AnyBar will quit during the
    /// execution of this function, it takes the ownership of `self`,
    /// which will be dropped when this function returns.
    ///
    /// # Panics
    /// Panics if the UDP socket can not be bound.
    ///
    /// # Example
    /// ```ignore
    /// # use anybar::*;
    /// let mut bar = Anybar::default();
    ///
    /// // do stuff...
    ///
    /// bar.quit();
    ///
    /// bar.set_color(Color::White);  // this won't work, bar has been moved
    /// ```
    pub fn quit(self) {
        let mut message: Vec<u8> = Vec::new();
        message.extend("quit".as_bytes()
                             .iter());

        let socket = Anybar::socket("127.0.0.1", 0);
        let _ = socket.send_to(&message, ("127.0.0.1", self.port));
        drop(socket);
    }
}

/// Instanciates the default AnyBar, connected to the Port `1738`.
///
/// ```
/// # use anybar::*;
/// let bar = Anybar::default();
/// assert_eq!(bar.port, 1738);
/// ```
impl Default for Anybar {
    fn default() -> Anybar {
        Anybar{port:1738, color:None}
    }
}
