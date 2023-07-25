#![allow(dead_code)]
#[derive(Debug)]
// declare an enum
enum TrafficSignal{
    Red, Green, Yellow
}

//implement a Traffic Signal methods
impl TrafficSignal{
    // if the signal is red then return
    fn is_stop(&self)->bool{
        match self{
        TrafficSignal::Red=>return true,
        _=>return false
        }
    }
}

#[derive(Debug)]
// declare an enum
enum KnightMove{
    Horizontal,Vertical
}

#[derive(Debug)]
// make this `struct` print values of type `enum`  with `fmt::Debug`.
struct Player {
    color:String,
    knight:KnightMove
}

#[derive(Debug)]
struct Course {
    code:i32,
    name:String,
    level: Option<String>, 
}

// print function 
fn print_direction(direction:KnightMove) {
   // match statement
    match direction {
        //execute if knight move is horizontal
        KnightMove::Horizontal => {
            println!("Move in horizontal direction");
        },
        //execute if knight move is vertical
        KnightMove::Vertical => {
            println!("Move in vertical direction");
        }
    }
}

#[derive(Debug)]
// declare an enum
// make this `enum` printable with `fmt::Debug`.
enum KnightMoveData{
    Horizontal(String), Vertical(String)
}

fn lookup(str: String, index: usize) {
    let matched_index = match str.chars().nth(index){
        // execute if match found print the value at specified index 
        Some(c)=>c.to_string(),
        // execute if value not found
        None=>"No character at given index".to_string()
        };  
    println!("{}", matched_index);
}

fn print(my_val: Option<&str>){
     if my_val.is_some(){ // check if the value is equal to some value
        println!("my_val is equal to some value");
    }
    else{
        println!("my_val is equal to none");
    }
}

fn main() {
    // invoke an enum
    let horizontal_move = KnightMoveData::Horizontal("Left".to_string());
    let vertical_move = KnightMoveData::Vertical("Down".to_string());
    // print enum
    println!("Move 1: {:?}", horizontal_move);
    println!("Movw 2: {:?}", vertical_move);

    // define an enum instance
    let action = TrafficSignal::Red;
    //print the value of action
    println!("What is the signal value? - {:?}", action);
    //invoke the enum method 'is_stop' and print the value
    println!("Do we have to stop at signal? - {}", action.is_stop());

    // invoke function `print_direction`
    let knight1 = KnightMove::Horizontal;
    let knight2 = KnightMove::Vertical;
    print_direction(knight1);
    print_direction(knight2);

    // instance 1
    let p1 = Player{
    color:String::from("black"),
    knight:KnightMove::Horizontal
    };
    // instance 2
    let p2 = Player{
    color:String::from("white"),
    knight:KnightMove::Vertical
    };
    println!("{:?}", p1);
    println!("{:?}", p2);

    //initialize
    let course1 = Course  {
        name:String::from("Rust"),
        level:Some(String::from("beginner")),
        code:130
    };
    let course2 = Course  {
        name:String::from("Javascript"),
        level:None,
        code:122
    };
    //access
    println!("Name:{}, Level:{} ,code: {}", course1.name, course1.level.unwrap_or("Level".to_string()), course1.code);
    println!("Name:{}, Level:{} ,code: {}", course2.name, course2.level.unwrap_or("No level defined!".to_string()), course2.code);

    // define a variable
    let str_ = String :: from("Educative");
    // define the index value to be found
    let index = 12;
    lookup(str_, index);

    let my_val: Option<&str> = Some("Rust Programming!");
    print(my_val); // invoke the function
}