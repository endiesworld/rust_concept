fn main(){
    println!("- Passing a string literal"); 
    concatenate(" Rust ", " Programming "); 
    println!("- Passing an integer"); 
    concatenate(10 as i32, 1 as i32);

    let int_vec = [1, 2, 3, 4, 5]; // define a vector of type integer
  
    println!("Call to the function with vector of integers");
    
    print_vec(& int_vec); // pass vector of type integer to the function
    
    println!("Call to the function with vector of strings");
    
    let str_vec = ["Rust", "Programming"]; // define a vector of type string

    print_vec(&str_vec); // pass vector of type String to the function

    //generic type of i32
    let r1:Rectangle<i32> = Rectangle{width:250, height:150};
    println!("Width:{}, Height:{}", r1.width, r1.height);
    //generic type of String
    let r2:Rectangle<f32> = Rectangle{width:240.0, height:250.0};
    println!("Width:{}, Height:{}", r2.width, r2.height);
}
use std::fmt::Display;
fn concatenate<T:Display>(t:T, s:T){
    let result = format!("{}{}", t , s);
    println!("{}", result);
}

use std::fmt::Display;
fn print_vec<T:Display>(v: &[T]) {
    for i in v.iter() {
        print!("{}", i)
    }
    println!("");
}

struct Rectangle<T> {
    width:T,
    height:T
}