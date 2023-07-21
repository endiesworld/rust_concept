#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    lifetime_1();
    lifetime_2();
}

fn get_oldest_1(person1: Person, person2: Person) -> Person {
    if person1.age > person2.age {
        person1
    } else {
        person2
    }
}

fn get_oldest_2<`a>(person1: &`a Person, person2: &`a Person) -> &`a Person {
    if person1.age > person2.age {
        person1
    } else {
        person2
    }
}

fn lifetime_2(){
    let p1: Person = Person { name: "Emmanuel".to_string(), age:36 };
    let p2: Person = Person { name: "Adaobi".to_string(), age:31};
    // let p3: Person = get_oldest_1(p1,  p2) ;

    // By doing "let p3: Person = get_oldest_1(p1,  p2)" ,
    // This will fail because p1 and p2 have been moved, i.e the ownership have been 
    //transferred to person1 and person2 parameters
    // println!("p1: {:?}", p1); 
    // println!("p2: {:?}", p2);

    let p3: &Person = get_oldest_2( &p1, &p2);
    println!("p3: {:?}", p3);
}


fn lifetime_1() {
    let x: String;
    {
        let y: String = "Hello, world!".to_string();
        x = y;
        // println!("{}", y); // This will fail because the value that y points to has been moved to x
        // x = &y; // Doing "println!("{}", x);" after the "}" will fail because "&y" makes it a reference and ref are only valid within their scope.
    }
    println!("{}", x);
}
