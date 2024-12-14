use std::fs::File;
use std::io::ErrorKind;

fn main() {
    //let v=vec![1,2,3];
    let file_path="hello.txt";
    let contents=match File::open(file_path){
        Ok(file)=>file,
        Err(error)=>match error.kind(){
            ErrorKind::NotFound=>match File::create(file_path){
              Ok(fc)=>fc,
              Err(err)=>panic!("Problem creating the file:{err:?}"),
            },
            other_error=>{
                panic!("Problem opening the file: {other_error:?}");
            }
        },
        
        
    };
    println!("{:?}",contents);
    //v[100];
}
