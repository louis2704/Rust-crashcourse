struc Square<T>::{
    x:<T>
}
struct Triangle<T>{
    x: T
}
struct Pyramid<T>{
    x: T
}
trait Volume {
    fn volume(&self) ->
}
trait Surface<T>{
    fn area(&self) -> T;   
}
trait Area {
    fn area(&self) -> f64;
}
impl Square<u32> {
    fn new( t: u32) -> Self {
        Square { x : t}
    }
}
impl Square<f64> {
    fn new( t: f64) -> Self {
        Square { x : t}
    }
}
impl Square<String> {
    fn new( t: &str) -> Self {
        Square { x : t}
    }
}
impl Triangle<f64> {
    fn new( t : f64) -> Self {
        Triangle { x : t}
    }
}
impl Pyramid<f64> {
    fn new( t : f64) -> Self {
        Pyramid { x : t}
    }
}
impl Square<T> for Surface<T>{
    fn area
    (x.&self*x.&self) -> f64;
}
impl Area for Square<u32> {
    fn area(&self) -> u32 {
        self.side * self.side
    }
}

impl Area for Square<f64> {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}
fn main() {
    let square = Square::<u32>::new(5);
    let square_float = Square::<f64>::new(5.4);
    let square_string = Square::<String>::new("6");

    println!("square area is {}", square.area());
    println!("square_float area is {}", square_float.area());
    println!("square_string area is {}", square_string.area());

    let triangle = Triangle::new(14.9, 20.1);
    let pyramid_square = Pyramid::<Square<u32>, f64>::new(square, 24.3);
    let pyramid_triangle = Pyramid::<Triangle<f64>, f64>::new(triangle, 24.3);

    println!("pyramid_square volume is {}", pyramid_square.volume());
    println!("pyramid_triangle volume is {}", pyramid_triangle.volume());
}

