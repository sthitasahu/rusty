fn main() {
    let mut a=0;
    let mut b=1;
    loop{
        let c=a+b;
        a=b;
        b=c;
        println!("{c}");
    }
}
