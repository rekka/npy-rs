#[macro_use]
extern crate npy_derive;
extern crate npy;

use std::io::Read;

#[derive(NpyData, Debug, PartialEq)]
struct Array {
    a: i32,
    b: f32,
}

fn main() {
    let mut arrays = vec![];
    for i in 0..360i32 {
        arrays.push(Array { a: i, b: (i as f32 * 3.14 / 180.0).sin() });
    }

    npy::to_file("examples/roundtrip.npy", arrays).unwrap();

    let mut buf = vec![];
    std::fs::File::open("examples/roundtrip.npy").unwrap()
        .read_to_end(&mut buf).unwrap();

    for (i, arr) in npy::from_bytes::<Array>(&buf).unwrap().enumerate() {
        assert_eq!(Array { a: i as i32, b: (i as f32 * 3.14 / 180.0).sin() }, arr);
    }
}