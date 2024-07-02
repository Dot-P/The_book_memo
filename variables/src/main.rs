fn main() {
    let mut x: i32 = 5;
    const CONSTANT: usize = 100;
    println!("The value of CONSTANT is: {}", CONSTANT);
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }

    println!("The value of y is: {}", y);

    let some_string = "aaa";
    println!("The value of some_string is: {}", some_string);

    let some_string = some_string.len();
    println!("The value of some_string is: {}", some_string);

}
