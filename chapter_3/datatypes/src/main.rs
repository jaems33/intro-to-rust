fn main() {
    // Integer Types
    let _a1 = 29; // i32
    let _a2: i64 = 29993929;
    
    // Floating Point Types
    let _b1 = 2.0; // f64
    let _b2: f32 = 3.0; //f32

    // Bool (obviously)
    let _c1 = true;
    let _c2: bool = false;

    // Chars
    let _d1 = 'z';
    let _d2: char = 'z';

    // Compound types
    let tuple1:(i32, i32, i32) = (10, 20, 30);
    let (_x, _y, _z) = tuple1;
    let _first_pos = tuple1.0;

    // Array
    let _arr1 = [1, 2, 3, 4];
    let _arr2: [i128; 5] = [100, 200, 300, 400, 500];
    let _arr3 = [0; 10]; // Generate 10 elements set to 0
}
