struct Point {
	x: i32,
	y: i32
}

fn main() {
    println!("Hello, world!");
    
    assert_eq!(1.4142135 , test(Point { x: 3, y: 4 }, Point { x: 2, y: 3}));
    assert_eq!(1.4142135 , test_2(Point { x: 3, y: 4 }, Point { x: 2, y: 3}));

}

fn test(_point1: Point, _point2: Point)-> f32 {
	// Write code here!
    let x1 = _point1.x ;
	let y1 = _point1.y ;

	let x2 = _point2.x ;
	let y2 = _point2.y ;

	let x1_minus_x2 = x1 - x2 ;
	let x1_minus_x2_square = x1_minus_x2 * x1_minus_x2 ;

	let y1_minus_y2 = y1 - y2 ;
	let y1_minus_y2_square = y1_minus_y2 * y1_minus_y2 ;

	let xs_square_plus_ys_square = x1_minus_x2_square as f32  + y1_minus_y2_square as f32 ;

	return xs_square_plus_ys_square.sqrt() ;
}


fn test_2(point1: Point, point2: Point)-> f32 {
    let distance = i32::pow(point1.x - point2.x,2) + i32::pow(point1.y - point2.y,2);
    let d = distance as f32;
    d.sqrt()
}