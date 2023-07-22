fn main() {

     // define a String object
    let str = String::from("Rust Programming"); 
    // split on whitespace
    for token in str.split_whitespace(){
        println!("{}", token);
    }

      // define a String object
    let str = String::from("Educative, course on, Rust, Programming");  
    // split on token
    for token in str.split(","){
        println!("{}", token);
    }

    // split on literal
    for token in str.chars(){
        println!("{}", token);
    }

    let course: &str = "Rust Programming";
    display_course_name(course); 
    println!("{}", course); // string literal is used after the function call

    display_course_name_(course.to_string()); 
   //cannot access course after display
}

fn display_course_name( my_course: &str){
    let mut new_str: String = my_course.to_string(); 
    new_str.push_str(" updated");
    println!("Course : {}",  new_str);
}

fn display_course_name_(mut my_course:String){
    let mut my_string = "Emmanuel".to_string();
    my_course.push_str(" updated");
    my_string.push_str(" Okoro");
    println!("Course : {}", my_course);
    println!("My name : {}", my_string);

    for token in my_string.split_whitespace(){
        for tok in token.chars(){
            if tok == 'O' {
                println!("My surname, {} starts with an {}", token, tok);
            }
            break;
        }

    }
    // println!("My Initials : {} . {}", my_string[0], my_string[9]); // You can't index a string here
}