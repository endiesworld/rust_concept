fn main() {
    let mut my_vec = Vec::new();
    println!("Empty Vector : {:?}", my_vec);
    my_vec.push(1);
    my_vec.push(2);
    my_vec.push(3);
    println!("Pushed elements 1 , 2 , 3 : {:?}", my_vec);
    my_vec.pop();
    println!("Popped value: {}", 3);
    println!("Popped element at last index : {:?}", my_vec);
    my_vec.remove(1);
    println!("Removed value: {}", 2);
    println!("Removed element at index 1 : {:?}", my_vec);
    println!("Size of vector is :{}", my_vec.len());
    println!("Does my vector contains 1 : {}", my_vec.contains(&1));

    my_vec = vec![1, 2, 3, 4, 5];
    match my_vec.get(10) {
        Some(x) => println!("Value at given index:{}", x),
        None => println!("Sorry, you are accessing a value out of bound")
    }
    match my_vec.get(3) {
        Some(x) => println!("Value at given index:{}", x),
        None => println!("Sorry, you are accessing a value out of bound")
    }

    vetor_iteration();
    loop_mutate_vector();
}

/// .iter() is the built-in function that iterates over the elements of the vector.
fn vetor_iteration(){

    //!.iter() is the built-in function that iterates over the elements of the vector.
    //! .position() is a built-in function that takes the element name to get the position of that element
    //! in the vector, i.e., (|&e| e == element_name) defines a variable e with the value equal to the name of the element that we want to find.
    //! .unwrap() is the built-in function.

    // defines a mutable vector
    let mut my_vec = vec![1, 2, 3, 4, 5];
    // define the value to be removed
    let value = 2; 
    // get the index of the value in the vector
    let index = my_vec.iter().position(|&r| r == value).unwrap();
    // call the built-in remove method
    for data in my_vec.iter(){
        println!("Present in the vector is: {} ", data) ;
    }
    my_vec.remove(index);
    // print the updated vector
    println!("Updated Vector: {:?}", my_vec);
}

fn loop_mutate_vector(){
    // define a vector of size 5
    let mut my_vec = vec![1, 2, 3, 4, 5];
    println!("Initial Vector : {:?}", my_vec);
    for x in my_vec.iter_mut(){
        *x *= 3;
    }
    // print the updated vector
    println!("Updated Vector : {:?}", my_vec);
}