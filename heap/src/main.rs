use std::collections::BinaryHeap;
fn main() {
    let mut heap=BinaryHeap::new();
    heap.push(9);
    heap.push(11);
    heap.push(10);
    heap.push(100);
    heap.push(13);
    //while let Some(x)=heap.pop(){
       // println!("{}",x);
    //}
    let first=heap.pop();
    match first{
        Some(x)=>println!("{}",x),
        None =>println!("Heap is empty"),
    }
    heap.clear();
    
    if check_is_empty(&heap){
        println!("Heap is empty");
    }    
    else{
        println!("Heap is not empty");
    }
    
    heap.push(276);
    if check_is_empty(&heap){
        println!("Heap is empty");
    }
    else{
        println!("Heap is not empty");
    }
    while  let Some(x) =heap.pop(){
         println!("{}",x);
    }

}


fn check_is_empty<T>(heap:&BinaryHeap<T>)->bool{
    return heap.is_empty();
}
