mod order;
mod product;
use order::place_order;
use product::show_products;
use inquire::Select;

fn main() {
    println!("---------------------------------------------------");
    println!("Welcome to the supermarket billing system in rust");
    println!("--------------------------------------------------\n");
    println!("Menu \n1 .Press A to place an order \n2. Press P to show products list\n3. Press E to exit \n");
    menu();
}

fn menu(){
    let options=vec!["Place order","Select Product list","Exit"];
    let input=Select::new("Menu",options.clone()).prompt();

    match input{
        Ok(input)=>{
            if (input.eq(options[0])){
                place_order();
            }
            else if (input.eq(options[1])){
                show_products();
            }
            else if (input.eq(options[2])){
                println!("Exiting the program");
                return;
            }
        },
        Err(err)=>{
            println!("Err while reading choice: {}",err);
        },
    }

}

