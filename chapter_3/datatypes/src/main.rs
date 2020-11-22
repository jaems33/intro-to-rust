fn main() {
    // Integer Types
    let a1 = 29; // i32
    let a2: i64 = 29993929;
    
    // Floating Point Types
    let b1 = 2.0; // f64
    let b2: f32 = 3.0; //f32

    // Bool (obviously)
    let c1 = true;
    let c2: bool = false;

    // Chars
    let d1 = 'z';
    let d2: char = 'z';

    // Compound types
    let tuple1:(i32, i32, i32) = (10, 20, 30);
    let (x, y, z) = tuple1;
    let firstPos = tuple1.0;

    // Array
    let arr1 = [1, 2, 3, 4];
    let arr2: [i128; 5] = [100, 200, 300, 400, 500];
    let arr3 = [0; 10]; // Generate 10 elements set to 0
}
