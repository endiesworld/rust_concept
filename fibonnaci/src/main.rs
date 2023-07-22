fn main() {
    println!("Hello, world!");
    println!("The fibonnaci series for 6 is: {}", fibonacci(6));
    println!("The fibonnaci series for 8 is: {}", fibonacci_form(8))
}

fn fibonacci(term: i32) -> i32 {
    // Write code here
    if term == 0{
        return 0 ;
    }
    let mut x = 1;
    let mut y = 1 ;
    if term > 2{
        for _ in 3..term + 1 {
            let z = y + x ;
            x = y ;
            y = z ;
        }
    }
    return y ;
}

fn fibonacci_form(term: i32) -> i32 {
    match term {
        0 =>  0,
        1 =>  1,
        _ => fibonacci(term-1) + fibonacci(term-2),
    }
}
