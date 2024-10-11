fn main() {
    let a;
    a = 127;
    println!("Value of a is {a}");

    // integers -> signed (i<size>) & unsigned (u<size>)
    // i8, u32, i64, u128, isize
    let b = -23;
    println!("Value of b is {b}");

    // u<n> (0 to 2^(n-1))
    // i<n> (-2^(n) to 2^(n-1))

    // Suffix
    let unsigned_num = 13u8;
    println!("Value of unsigned_num is {unsigned_num}");

    // _ as seperator in number for easier read
    let money = 1_00_000_000;
    println!("Total money I have {money}");
    

    // integer overflow

    // u8 (0 to 255)
    let num:u8 = 200 + random_number();
    println!("Value of num is {num}");
    // i8 (-128 to 127)

    // check 
    let (a, b) =  random_number().overflowing_add(200);
    println!("Value of num is a {a} b {b}");


    // floats
        let x = 2.0; // f64
        let y: f32 = 3.1; // f32
        println!("floats are {x},{y}");


    // Number operation
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // Print the results of number operations
    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
    println!("Truncated Division: {}", truncated);
    println!("Remainder: {}", remainder);

    // booleans

    let x = true;
    let y  = false;
    println!("x->{x} y->{y}");

    // chars
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😔';
    println!("Chars-> {c}, {z}, {heart_eyed_cat}");
}

fn random_number() -> u8 {
    50
}