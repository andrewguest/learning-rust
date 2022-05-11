fn main() {
    // boolean
    let x = true;  // set 'x' to true
    let y: bool = false;  // type hint that 'y' is going to be a boolean type and then set it to false

    // char
    let a = 'a';
    let b: char = '😎';

    // signed integers (the default integer type in Rust is i32)
    // i8, i16, i32, i64, i128
    let c = 10;
    let d: i8 = -128;

    // unsigned integers
    // u8, u16, u32, u64, u128
    let e: u8 = 255;

    // isize, usize
    // Pointer sized signed and unsigned integer types

    // floating point numbers
    // f32, f64
    // Should avoid using f32, unless you need to reduce memory consumption badly or if you are doing low-level optimization, when targeted hardware does not support for double-precision or when single-precision is faster than double-precision on it.
    let f = 1.5;
    let g: f64 = 2.0;

    // array (list)
    // Fixed size list of elements of same data type.
    // Arrays are immutable by default and even with mut, its element count cannot be changed.
    // If you are looking for a dynamic/ growable array, you can use vectors.
    let h = [1, 2, 3];
    let i: [i32; 3] = [1, 2, 3];  // [TYPE, # of elements]
    let mut j: [i32, 3] = [4, 5, 6];
    j[0] = 10;  // update '4' to be '10'

    let k = [0; 5]; // [0, 0, 0, 0, 0]

    // tuple
    // Fixed size ordered list of elements of different (or same) data types
    // Tuples are also immutable by default and even with mut, its element count cannot be changed.
    let l = (1, 1.5, true, 'a');
    let m: (i32, f64, bool, char) = (10, 12.5, false, 'x');

    // slice
    // Dynamically-sized reference to another data structure
    let parent_arry = [1, 2, 3, 4];
    let n = &parent_arry;  // slicing the whole array
    let o = &parent_arry[0..4];  // from 0th position to 4th(excluding)
    let p = &parent_arry[1..3];  // [2, 3]  (does not include the item in the 3rd place (4)

    // str
    // Unsized UTF-8 sequence of unicode string slices
    let q = "Hello, world!";  // a: &'static str
    let r: &str = "こんにちは, 世界!";  // &str is used to borrow and assign the whole array to the given variable binding.
}
