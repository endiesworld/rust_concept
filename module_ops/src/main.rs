mod  my_mod;
mod pub_mod ;
mod book; // make use of `use` keyword

use book::chapter::lesson::heading ;
// main function
fn main() {
    println!("Let's go inside the module");
    outer_module::inner_module::my_public_function();
    my_mod::my_public_function();
    println!("I am a public function in my_mod.rs");
    pub_mod::module::my_public_function();
    heading::illustration(); 
}

// fn my_function(){
//     println!("Hi, you came inside the root function using super");
// }

// declare a module
mod outer_module {
  // function within outer module
    fn my_private_function() {
        println!("Hi, I got into the private function of outer module");
    }
    // declare a nested module
    pub mod inner_module {
        // function within nested module
        pub fn my_public_function() {
            println!("Hi, I got into the public function of inner module");
            println!("I'll invoke private function of outer module");
            super::my_private_function();
        }
    }
    // call the function
    // println!("Invoke root function");
    // super::my_function();
}

