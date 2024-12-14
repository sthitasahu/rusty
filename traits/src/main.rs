trait Eat{
    fn eat(&self);
}

struct Person{
   name:String,
}
impl Eat for Person{
   fn eat(&self){
      println!("{} is eating",self.name);
    }
}

fn main(){
   let person1=Person{
    name:String::from("Sthita"),
   };
   person1.eat();
   let person2=Person{
    name:String::from("Sahu"),
   };
   person2.eat();
}