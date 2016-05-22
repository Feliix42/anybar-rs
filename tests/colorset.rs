extern crate anybar;

#[cfg(test)]
mod colorset {
    use anybar::*;

    // Integration test to test functionality
    #[test]
    fn set_color() {
        let anyb = Anybar::default();

        anyb.set_color(Color::White);
    }
}
