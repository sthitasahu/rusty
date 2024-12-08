fn main() {
    let v1=vec![1,2,3,4,5,6,7,8,9,10];
    let v1_iter=v1.iter();
    for value in v1_iter {
        println!("Got: {value}");
    }
}
