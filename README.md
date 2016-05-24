# anybar-rs

This is a small Rust Crate for [AnyBar](https://github.com/tonsky/AnyBar).

[![Build Status](https://travis-ci.org/Feliix42/anybar-rs.svg?branch=master)](https://travis-ci.org/Feliix42/anybar-rs)

## Examples

### Connect using the default port
```rust
use anybar::*;

// create a new AnyBar instance connected to the default port
let mut bar = Anybar::default();

// set the color
bar.set_color(Color::Red);
```

### Connect using a separate port
```rust
use anybar::*;

// Anybar::new() takes the AnyBar port as parameter
let mut custom_bar = Anybar::new(1708);
custom_bar.set_color(Color::Exclamation);
```

## Usage & Documentation
Please check the [documentation](https://feliix42.github.io/anybar-rs/) for more examples and details on certain functions.

## License
This work is licensed under the _MIT_ license. See `LICENSE` for more information.
