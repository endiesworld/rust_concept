fn main() {
    println!("Hello, world!");
    // range_of_values();
    // string_array();
    // string_array_2();
    // count_and_variable();
    // while_loop();
    // loop_loop();
    normal_factorial(4) ;
    test_triangle(9);
}

// // DEFINITE LOOP

// fn range_of_values(){
//     for i in 5..10{
//         println!("{}", i);
//     }
// }

// fn string_array(){
//     //define a for loop 
//     let arr: [&str; 3] = ["me", "you", "us"] ;
//     for i in arr.iter() {
//         println!("{}", i);
//     }
// }

// fn string_array_2(){
//     //define a for loop 
//     let arr: [&str; 3] = ["me", "you", "us"] ;
//     for i in &arr {
//         println!("{}", i);
//     }
// }

// fn count_and_variable(){
//     for (count, variable) in (7..10).enumerate() {
//         println!("count = {}, variable = {}", count, variable);
//     }
// }

// // INDEFINITE LOOP

// fn while_loop(){
//     let mut var = 1; //define an integer variable
//     let mut found = false; // define a boolean variable
//     // define a while loop
//     println!("while_looping");
//     while !found {
//         var=var+1;
//         //print the variable
//         println!("{}", var);
//         // if the modulus of variable is 1 then found is equal to true
//         if var % 2 == 1 {
//             found = true; 
//         }
//         println!("Loop runs");
//     }
// }

// fn loop_loop(){
//     //define an integer variable
//     let mut var = 1; 
//     // define a while loop
//     println!("Looping with loop keyword");
//     loop {
//         var = var + 1;
//         println!("{}", var);
//         if var == 6 {
//             break;
//         }
//     }
// }

// Challenges
fn normal_factorial(n: i32){
    // Write code here! 
    let mut x = 1;
    let stop = n + 1  ;
    if n < 0 {
        println!("{}", 0) ;
    }
    else {
        for data in 2..stop{
            x *= data ;
        }
        println!("factorila result for {} is: {}", n, x)
    }
}

fn test_triangle(n:i32) {
 // Write code here!
    for breadth in 1..n+1 {
        for _ in 0..breadth{
            print!("&")
        }
        println!("")
    }
}