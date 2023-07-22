fn main() {
    let my_str = "This is a comprehensive course in Rust programming 
    language on the Educative. Read it with full concentration 
    to grasp the content of the course".to_string();

    println!("Result from string ops: {}", str_test(my_str));
}

fn str_test(my_str:String)-> String {
    // Write code here
    let mut my_out_str: String = "".to_string() ;

    for token in my_str.split_whitespace(){
            for tok in token.chars(){
                if tok == 'c' {
                    my_out_str = format!("{} {}", my_out_str, token);
                }
                break;
            }
        }

        return my_out_str.trim().to_string();
}

fn test(my_str:String)-> String {
    let mut my_updated_string = "".to_string(); 
    for word in my_str.split(" "){
        if word.starts_with("c"){
            my_updated_string.push_str(word);
            my_updated_string.push(' ');
        }
    }
    my_updated_string.pop();
    my_updated_string
}