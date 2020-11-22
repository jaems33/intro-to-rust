// Function with no return value
fn hello_world() {
    println!("Hello, world!");
}

// Function with argument and return value
// Functions is synonymous with the final expression of a block without a semi-colon
fn adder(x: i32) -> i32 {
    x+1
}

// Functions can container new scopes
fn add_ten(x: i32) -> i32 {
    let y = {
        let z = x + 10;
        z
    };
    y
}

fn main() {
    hello_world();
    let output = add_ten(10);
    println!("Output is: {}", output);
    println!("Output is: {}", adder(output));
}
