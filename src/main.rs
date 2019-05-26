use std::{env, mem};
use std::process::exit;

fn main() {
    let mut bytes = match env::args().nth(1) {
        Some(s) => s.into_bytes(),
        None    => { 
            eprintln!("Usage: {} <text>", env::args().nth(0).unwrap());
            exit(1);
        }
    };

    let len = bytes.len();

    while bytes.len() % 4 != 0 {
        bytes.push(0);
    }

    println!(
"use std::str::from_utf8_unchecked as make_str;
use std::slice::from_raw_parts as make_slice;

fn main() {{
    let text = [");

    for b in bytes.chunks_exact(4) {
        let f: f32 = unsafe { mem::transmute([b[0], b[1], b[2], b[3]]) };
        println!("        {:.9e}f32,", f);
    }

    println!(
"    ];

    let text = unsafe {{
        make_str(make_slice(text.as_ptr() as _, {}))
    }};

    println!(\"{{}}\", text);
}}", len);
}
