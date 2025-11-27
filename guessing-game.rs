use rand::random;
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();
    // 随机生成一个1-100的整数
    let secret_number = random::<u32>() % 100 + 1;
    // println!("The secret number is: {}", secret_number);

    loop {
        guess.clear();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let my_guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };
        match my_guess.cmp(&secret_number) {
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
        }
    }
    println!("The secret number is: {secret_number}");
}
