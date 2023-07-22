fn main() {
    let n = 10 ;
    let mut a: i32 = 8;
    let mut arr = [1, 2, 3, 4, 5];
    square_by_value(n) ;
    println!("The value of n outside function : {}", n);
    square_by_ref(&mut a) ;
    println!("The value of a outside function of pass by REF : {}", a);

    let x: i32 = square_return_val(10);
    println!("The value of x outside function : {}", x);

    modify_my_array(arr);
    println!("Array in Driver Function : {:?}", arr);

    modify_my_array_2(&mut arr);
    println!("Array in Driver Function : {:?}", arr);

}

fn square_by_value(mut n:i32){
    n = n * n;
    println!("The value of n inside function : {}", n);
}

fn square_by_ref(n:&mut i32){
    *n = *n * *n;
    println!("The value of n for ref inside function : {}", n);
}

fn square_return_val(n:i32)->i32{
    println!("The value of n inside function : {}", n);
    let m = n * n;
    m // return the square of the number n
} 

fn modify_my_array(mut arr:[i32;5]){
    arr[2] = 8;
    arr[3] = 9;
    println!("Array in my Function : {:?}", arr);
}

fn modify_my_array_2(arr:&mut [i32;5]){
    arr[2] = 8;
    arr[3] = 9;
    println!("Array in my Function : {:?}", arr);
}