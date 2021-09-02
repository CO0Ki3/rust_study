fn main() {
    //     ERROR!!     
    //
    //    let x = 5;
    //    println!("The value of x is: {}", x);
    //    x = 6;
    //    println!("The value of x is: {}", x); 
    
    // mut means mutable

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);


    // constant
    const MAX_POINTS: u32 = 100_000;
    println!("Max point is: {}", MAX_POINTS);

    
    // Shadowing
    let t = 5;
    let t = t + 1;
    let t = t * 2;
    println!("Shadowing value is: {}", t);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("Space length is: {}", spaces);


    //Multiple Types
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Guess is: {}", guess);


    //Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("Tuple: {}, {}", tup.0, tup.2);
    println!("Tuple: {}, {}, {}", x, y, z);


    //expression
    let x = 5;
    let y = {
        let x = 2;
        x + 2
    };
    println!("x value is: {}", x);
    println!("y value is: {}", y);


    //function
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
