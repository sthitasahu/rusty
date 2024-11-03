
#[derive(Debug)]

enum Direction{
    North,
    South,
    East,
    West,
    
}

fn main() {
    //let my_direction=Direction::North;
    let other_direction=Direction::South;
    move_around(other_direction);
    let third_direction=Direction::East;
    let fourth_direction=Direction::West;

    //move_around(my_direction);
    //move_around(other_direction);
    move_around(third_direction);
    move_around(fourth_direction);
    
}

fn move_around(direction:Direction){
    //
    println!("Hello there .The direction is:{:?}",direction);
}
