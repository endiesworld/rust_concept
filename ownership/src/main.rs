fn main() {
    println!("Hello, world!");
    regular_ops();
    ownership_test();
    immutable_ref();
    mutable_ref();
    mutable_ref_2();
    mutable_ref_3();
}

fn regular_ops(){
    let x:i32 = 20;
    let y = x; 
    println!("x: {:?}", x);
    println!("y: {:?}", y);
}


fn ownership_test(){
    let x: String = String::from("45"); // This creates a storage space in the Heap with a pointer address stored in the stack as the value that x refers to (pointer).
    // Ownership Error
    // let y: String = x ; // This transfers the pointer address stored in x to y, with x no longer having a value stored.
    // println!("x: {:?}", x); // This throws an error "value borrowed here after move"
    
    // SOLUTION ONE TO OWNERSHIP ERROR
    // let y: String = x.clone(); //Using the clone trait isn not the best approach as it encourages memory duplication
    
    //SOLUTION TWO TO OWNERSHIP ERROR (RECOMMENDED)---IMMUTABLE REFERENCE
    let y: &String = &x;
    
    println!("x: {:?}", x);
    println!("y: {:?}", y);

    //SOLUTION THREE TO OWNERSHIP ERROR (RECOMMENDED)---MUTABLE REFERENCE

}

fn immutable_ref(){
    let x: String = String::from("Hello world"); 
    let y: &String = &x;
    let z: &String = &x;
    
    println!("x: {:?}", x);
    println!("y: {:?}", y);
    println!("*y: {:?}", *y);
    println!("*z: {:?}", *z);

    print_length(&x);
    print_length(y);
    print_length(&y);
}

fn print_length(a: &String){
    println!("The length of '{}' is: {:?}", a, a.len());
}

fn update_string(x: &mut String){
    x.push_str(", I am new to the Rust programming language");
}

fn mutable_ref(){
    let mut x: String = String::from("Hello world"); // A variable must be made mutable before being used in a mutable reference operation.
    let y: &mut String = &mut x; //mutable reference
    y.push_str(" my name is Emmanuel");

    println!("The value of x is: {:?}", x);

    update_string(&mut x);

    println!("The value of x is: {:?}", x);
}

fn mutable_ref_2(){
    let mut x: i32 = 50; // A variable must be made mutable before being used in a mutable reference operation.
    // let mut z: i32 = 60; 
    let y: &mut i32 = &mut x; //mutable reference
    
    *y += 1;
    // y = &mut z; // This won't work as y itself is immutable

    // println!("The value of x is: {:?}", x); // This doesn't work as it fails one of the rules of using references
    println!("The value of y is: {:?}", y);
    println!("The value of x is: {:?}", x);
}

fn mutable_ref_3(){
    let mut x: i32 = 50; // A variable must be made mutable before being used in a mutable reference operation.
    let y: &mut i32 = &mut x;

    *y += 1;
    
    // Rust's borrow checker will detect that x has a mutable reference y pointing to it, 
    //and you cannot have immutable (non-mutable) references alongside a mutable reference 
    //to the same data in the same scope. This restriction is in place to prevent data races.
    println!("The value of x is: {:?}", x); // This doesn't work as it fails one of the rules of using references
    // println!("The value of y is: {:?}", y);

}