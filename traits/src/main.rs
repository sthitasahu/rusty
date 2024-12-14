

// Define a generic trait for eating
trait Eat {
    fn eat(&self);
}

// Define a generic trait for greeting
trait Greet {
    fn say_hello(&self);
}

// Structs for different types of beings
struct Person {
    name: String,
}

struct Cat {
    breed: String,
}

struct Dog {
    breed: String,
}

struct Bird {
    species: String,
}

// Implement `Eat` trait for `Person`
impl Eat for Person {
    fn eat(&self) {
        println!("{} is eating a meal.", self.name);
    }
}

// Implement `Eat` trait for `Cat`
impl Eat for Cat {
    fn eat(&self) {
        println!("A {} cat is eating its food.", self.breed);
    }
}

// Implement `Eat` trait for `Dog`
impl Eat for Dog {
    fn eat(&self) {
        println!("A {} dog is eating its kibble.", self.breed);
    }
}

// Implement `Eat` trait for `Bird`
impl Eat for Bird {
    fn eat(&self) {
        println!("A {} bird is pecking at seeds.", self.species);
    }
}

// Implement `Greet` trait for `Person`
impl Greet for Person {
    fn say_hello(&self) {
        println!("Hello, my name is {}!", self.name);
    }
}

// Implement `Greet` trait for `Cat`
impl Greet for Cat {
    fn say_hello(&self) {
        println!("Meow! I am a {} cat.", self.breed);
    }
}

// Implement `Greet` trait for `Dog`
impl Greet for Dog {
    fn say_hello(&self) {
        println!("Woof! I am a {} dog.", self.breed);
    }
}

// Implement `Greet` trait for `Bird`
impl Greet for Bird {
    fn say_hello(&self) {
        println!("Chirp! I am a {} bird.", self.species);
    }
}

// Generic function to greet any entity that implements the Greet trait
fn greet<T: Greet>(entity: &T) {
    entity.say_hello();
}

// Generic function to make any entity that can eat, eat
fn eat<T: Eat>(entity: &T) {
    entity.eat();
}

fn main() {
    // Create instances of various beings
    let person = Person {
        name: String::from("Alice"),
    };
    let cat = Cat {
        breed: String::from("Persian"),
    };
    let dog = Dog {
        breed: String::from("Labrador"),
    };
    let bird = Bird {
        species: String::from("Parrot"),
    };

    // Call greet for each entity
    greet(&person); // Pass references to avoid value borrowing errors
    greet(&cat);
    greet(&dog);
    greet(&bird);

    // Call eat for each entity
    eat(&person); // Pass references to avoid value borrowing errors
    eat(&cat);
    eat(&dog);
    eat(&bird);
}
