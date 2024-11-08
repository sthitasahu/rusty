fn main() {
    let mut vec=Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);

    println!("{:?}",&vec);

    let  mut new_vec =even_values(vec);
    even_values_by_reference(&mut new_vec);
    println!("New vector is {:?}",new_vec);
}

fn even_values(v:Vec<i32>)->Vec<i32>{
    let mut  new_vec=Vec::new();
    for value in v{
        if value%2==0{
            new_vec.push(value);
        }
    }
    new_vec
}

fn even_values_by_reference(v:&mut Vec<i32> ){

    let mut i=0;
    while i<v.len(){
        if v[i]%2!=0{
            v.remove(i);
        }
        else{
            i+=1;
        }
    }

}