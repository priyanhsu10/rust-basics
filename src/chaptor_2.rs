use rand::Rng;
use std::cmp::Ordering;
use std::io;
pub fn guessing_game() {
    let n = rand::thread_rng().gen_range(0..100);
    println!("Guess the number");
    loop {
        println!("please input your gues");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("faile to read line");
        let number: i32 = guess.trim().parse().unwrap();

        println!("You guess :{}", guess);

        match number.cmp(&n) {
            Ordering::Equal => {
                println!("correlly gues you won !");
                break;
            }
            Ordering::Greater => println!("Big"),
            Ordering::Less => println!("Less"),
        }
    }
}
