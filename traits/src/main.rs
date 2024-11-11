
fn main() {
     let ans;
    let a=String::from("small");
    {
      let b=String::from("longer");   
      ans=longest_string(a,b);
    }

    println!("{}",ans);
}

fn longest_string(a:String,b:String)->String{
    if a.len()>b.len(){
        return a;
    }
    else{
        return b;
    }
}