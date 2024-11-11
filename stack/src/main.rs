struct Stack<T>{
    items:Vec<T>

}

impl <T> Stack<T>{
    fn new() -> Stack<T> {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn size(&self) -> usize {
        self.items.len()
    }
}

fn main() {
    let mut stack = Stack::new();

    println!("Pushing 1.778");
    stack.push(1.778);
    println!("Pushing 2.890");
    stack.push(2.890);
    println!("Pushing 3.900");
    stack.push(3.900);

    println!("Size of the Stack: {}", stack.size());

    let first = stack.pop();
    println!("Got: {:?}", first);

    let second = stack.pop();
    println!("Got: {:?}", second);
    
    println!("Pushing 4.789");
    stack.push(4.789);
    
    let third = stack.pop();
    println!("Got: {:?}", third);
    
    let fourth = stack.pop();
    println!("Got: {:?}", fourth);
    
    let fifth = stack.pop();
    println!("Got: {:?}", fifth);
    
    println!("Is the stack empty? {}", stack.is_empty());
}