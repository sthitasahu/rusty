enum Shape{
  Circle(f64),
  Rectangle(f64,f64),
  Square(f64)

}


fn calculate_area(shape:Shape)->f64{
    match shape{
        Shape::Circle(radius)=>radius*radius*3.14,
        Shape::Square(side)=>side*side,
        Shape::Rectangle(length,breadth)=>length*breadth
    }
}
fn main() {
    println!("Hello, welcome to this basic enum exercise with Rust!");
    let circle=Shape::Circle(5.0);
    let square=Shape::Square(8.0);
    let rectangle=Shape::Rectangle(4.0,8.0);

    println!("Area of circle is :{}",calculate_area(circle));
    println!("Area of square is:{} ",calculate_area(square));
    println!("Area of rectangle is:{} ",calculate_area(rectangle));

}
