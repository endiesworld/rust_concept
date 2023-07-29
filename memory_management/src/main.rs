fn main() {
    let str = String::from("Rust"); // str comes into scope
                                    // str is a move type

    pass_string_object(str);        // str's value moves into the function...
                                    // ... and becomes in accessible here
    //println!("{}" , str);         // This line will give an error

    let my_int = 10;                // my_int comes into scope

    pass_integer(my_int);          // my_int value is a copy into the function,
                                    // but i32 is a copy type, so can my used
                                    // use my_int if desired
    let str_1 = move_return_value_str_1();  // gives_ownership to str_1                                  
    
    println!("The function gives ownership to string by returning a value \nstring 1 :{}",str_1); // print value of str_1

    let str_2 = String::from("Rust Language");     // assigns a string object to str_2
    
    println!("This is a string declared \nstring 2 :{}",str_2); // print value of str_2   

    let str_3 = moves_str_2_return_str_2(str_2);  // str_2 is moved into the function argument
                                            // return value moves to str_3 
    println!("string 2 passes to the function and returns its value to string 3 \nstring 3 :{}",str_3); // print value of str_3                         

} // Here, my_int and then str goes out of scope

fn pass_string_object(my_string: String) { // my_string comes into scope
    println!("{}", my_string);
} // Here, my_string goes out of scope and `drop` frees the memory

fn pass_integer(my_integer: i32) { // my_integer comes into scope
    println!("{}", my_integer);
} // Here, my_integer goes out of scope

fn move_return_value_str_1() -> String {     // gives ownership 
                                             // value goes to that calls the function
    let my_string = String::from("Rust"); // my_string comes into scope

    my_string                              // my_string is returned 
}

fn moves_str_2_return_str_2(my_string: String) -> String { // my_string comes into 
                                                      // scope
    my_string  // my_string is returned 
}