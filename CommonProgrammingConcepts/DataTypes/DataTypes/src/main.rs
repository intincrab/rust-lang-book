fn main() {
    let x=2.0; //f64 this has double precision

    let y:f32 = 3.0; //f32 this has single precision
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2; // index at 2 position

    let a: [i32; 5] = [1, 2, 3, 4, 5];
}
