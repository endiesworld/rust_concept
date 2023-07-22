fn main() {
    println!("Hello, world!");
    let mut my_vec_result: Vec<u32> = vec![1, 2, 1, 2, 6] ;
    let mut my_vec: Vec<u32> = vec![1, 2, 3, 1, 2, 6] ;
    assert_eq!(&mut my_vec_result , test(&mut my_vec));

}

fn test(my_vec: &mut Vec<u32>)-> &mut Vec<u32>{
   // Write code here!
    let rm_index = (my_vec.len() / 2) - 1 ;
    my_vec.pop() ;
    my_vec.remove(rm_index) ;
    let mut sum = 0 ;
    for data in my_vec.iter(){
        sum += data ;
    }

    my_vec.push(sum) ;

        return my_vec
}