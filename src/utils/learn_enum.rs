use std::f32::consts::PI;

#[derive(Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

enum Shape {
    Circle(f32),  // Variant with associated data (radius)
    Square(f32),  // Variant with associated data (side length)
    Rectangle(f32, f32),  // Variant with associated data (width, height)
}

// Function to calculate area based on the shape
fn calculate_area(shape: Shape) -> f32 {
    // calculates the area of the shape
    let ans = match shape{
        Shape::Circle(radius) => PI* radius * radius,
        Shape::Rectangle(width, height, ) => {
            width * height
        },
        Shape::Square(side) => side * side,

    };
    return ans;
}

pub fn calculate() {
    // Create instances of different shapes
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);

    let circle_area = calculate_area(circle);
    println!("Calculate the area {}", circle_area);

}

pub fn use_direc_enum() {
    let my_direction = Direction::North;
    move_around(my_direction);
}

pub fn move_around(direction: Direction) {
    println!("{:?}", direction);
    return; direction;
}

pub fn log_direction(){
    let direction_to_move = move_around(Direction::North);
    println!("{:?}", direction_to_move);
}