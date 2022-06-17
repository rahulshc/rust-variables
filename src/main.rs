fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    //let spaces = "   ";
    //let spaces = spaces.len(); //good that we don't need create another variable

    //let mut spaces = "   ";
    //spaces = spaces.len(); error, we can't mutate a variable type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //type annotations above is optional

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
    //let number = if condition { 5 } else { "six" }; ERROR
    let (x,y,z) = tup;
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5//returns 5
}

fn plus_one(x: i32) -> i32 {
    x + 1;//error because of semicolon
}