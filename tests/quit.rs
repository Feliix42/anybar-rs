extern crate anybar;

#[cfg(test)]
mod tests {
    use anybar::*;

    #[test]
    fn quit_anybar() {
        let bar = Anybar::new(1708).unwrap();
        bar.quit().unwrap();
    }
}
