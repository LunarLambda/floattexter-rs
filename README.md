# FloatTexter-RS

A Rust version of [FloatTexter](https://github.com/LunarLambda/FloatTexter).

Build and install with `cargo install --path .`  
Not sure why you would though. ╮(´･ᴗ･` )╭

## Examples

Running `floattexter-rs "Hello, World"` produces:

```rust
use std::str::from_utf8_unchecked as make_str;
use std::slice::from_raw_parts as make_slice;

fn main() {
    let text = [
        1.143139122e27f32,
        1.761127013e14f32,
        1.744670964e22f32,
    ];

    let text = unsafe {
        make_str(make_slice(text.as_ptr() as _, 12))
    };

    println!("{}", text);
}
```

Which you can execute with:

```sh
$ floattexter-rs "Hello, World!" | rustc -o out - && ./out
Hello, World
```

## License

[MIT](LICENSE)
