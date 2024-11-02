use std::io;

use rand::Rng;


fn main()
{
    let guess_list: [&str; 4]=["banana","apple","mango","kiwi"];

    let random= rand::thread_rng().gen_range(0..guess_list.len());
    println!("{}",random);
   

    loop {
        println!("enter the fruit");
        let mut fruit= String::new();

        io::stdin()
         .read_line(&mut fruit)
         .expect("failed");
        let fruit = fruit.trim();

      

        if fruit==guess_list[random]{
            println!("your guess is right wooooooh!!!");
            break;
        }
        else {
            println!("u failed to guess the right ohhhhh!!! try again");
        }
    }

}
