use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    let mut heap = BinaryHeap::new();
    heap.push(Reverse(7));
    heap.push(Reverse(8));
    heap.push(Reverse(5));
    heap.push(Reverse(220));

    // Pop the first element
    match heap.pop() {
        Some(Reverse(x)) => println!("{}", x), // Dereference Reverse
        None => println!("Heap is empty"),
    }

    // Pop remaining elements and print them
    //while let Some(Reverse(x)) = heap.pop() {
       // println!("{}", x); // Dereference Reverse
    //}

    // Get the size of the heap (use len, not capacity)
    let size = heap.len();
    println!("Heap size: {}", size);

    // Check if the heap is empty
    println!("Is heap empty? {}", heap.is_empty());

    // Drain the heap and print each element (note: heap is now empty)
    // Drain extracts elements and empties the heap, so make sure to check the output.
    for Reverse(x) in heap.drain() {
        println!("{}", x); // Dereference Reverse
    }
}
