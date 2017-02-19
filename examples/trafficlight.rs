extern crate anybar;
use anybar::{Anybar,Color};
use std::{thread, time};

fn main() {
    // create a new AnyBar instance connected to the default port
    let mut bar = Anybar::default();

    // set the color
    bar.set_color(Color::Red).unwrap();
    thread::sleep(time::Duration::from_millis(700));
    bar.set_color(Color::Orange).unwrap();
    thread::sleep(time::Duration::from_millis(700));
    bar.set_color(Color::Green).unwrap();
    thread::sleep(time::Duration::from_millis(700));
    bar.quit().unwrap();
}
