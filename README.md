# colorconv

This crate provides some features to convert color code, RGB or color name(if exists) to struct `Color` which holds the color information.

What makes this crate (kind of) unique is that it supports the color name conversion. For example:
```rust
use colorconv::Color;

match Color::try_from("yale blue") {
    Ok(color) => assert_eq!(color.hex, "0f4d92".to_string()),
    Err(e) => eprintln!("{:?}", e),
}
```
This conversion is based on <https://github.com/jonathantneal/color-names>.

Also, you can convert a color code or RGB:
```rust
use colorconv::Color;

if let Ok(rusty_red) = Color::try_from("da2c43") {
    assert_eq!(Some("rusty red".to_string()), rusty_red.name);
}

let true_blue = Color::from([0, 115, 207]);
assert_eq!("0073cf".to_string(), true_blue.hex);