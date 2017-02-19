extern crate anybar;

#[cfg(test)]
mod tests {
    use anybar::*;

    #[test]
    #[should_panic]
    fn wrong_port() {
        let _ = Anybar::new(6554).unwrap();
    }

    #[test]
    fn valid_port() {
        let bar = Anybar::new(1708).unwrap();
        assert_eq!(bar.port, 1708);
    }
}
