fn main() {
    // let and mut
    let mut age = 24;
    println!("Age is {age}");

    
    age = 25;
    println!("Age is {age}");

    // const
    const PI:f64 = 3.14;
    println!("Value of PI is {PI}");

    // Shadowing

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
         println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
