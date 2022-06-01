use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
pub fn guessing_game() {
    let n = rand::thread_rng().gen_range(0..100);
    println!("{}", "Guess the number".magenta());
    loop {
        println!("{}", "please input your guess".bright_purple());
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("faile to read line");
        let number: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guess :{}", guess);

        match number.cmp(&n) {
            Ordering::Equal => {
                println!("{}", "correlly gues you won !".green());
                break;
            }
            Ordering::Greater => println!("{}", "Big".red()),
            Ordering::Less => println!("{}", "Less".red()),
        }
    }
}
