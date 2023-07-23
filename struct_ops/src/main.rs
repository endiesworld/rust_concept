//declare a struct
struct Course {
    code:i32,
    name:String,
    level:String, 
}

//impl construct to define struct methods
impl Course {
    // static method
    fn my_static_method(n: String, l: String, c:i32) -> Course {
        Course { 
        name: n, 
        level:l,
        code:c
        }
    }
    //display
    fn display(&self){
        println!("name :{} code:{} of type: {}", self.name, self.code, self.level );
    }
}

fn display_mycourse_info(c: &Course) {
    println!("Name:{}, Level:{} ,code: {}", c.name, c.level, c.code);
}

fn return_rust_course_info(c1:Course, c2:Course)-> Course{
    println!("I got into function and return values from there");
    if c1.name == "Rust" {
        return c1;
    }
    else{
        return c2;
    }
}

fn main() {
   //initialize
    let course1 = Course  {
        name:String::from("Rust"),
        level:String::from("beginner"),
        code:130
    };

    let course2 = Course  {
        name:String::from("Java"),
        level:String::from("beginner"),
        code:130
    };

    display_mycourse_info(& course1);
    display_mycourse_info(& course2);

    let choose_course = return_rust_course_info(course1, course2);
    
    // THIS WON"T WORK BECAUSE THE VALUE WAS MOVED ABOVE
    // display_mycourse_info(& course1);
    // display_mycourse_info(& course2); 

    //call the non-static method
    choose_course.display();

    
     // call the static method
    let c1 = Course::my_static_method("Rust".to_string(), "beginner".to_string(), 132);
    c1.display();
}
