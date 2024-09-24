use std::cmp::Ordering;
use std::io;
use rand::Rng;

pub fn bao_func() {
    let mut guess: String = String::new();
    let generated_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter a number: ");
        io::stdin().read_line(&mut guess).expect("Something wrong");
        let user_choice: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                guess.clear();
                continue;
            }
        };

        match generated_number.cmp(&user_choice) {
            Ordering::Less => println!("Загаданное число меньше"),
            Ordering::Equal => {
                println!("Загаданное число угадано");
                break;
            }
            Ordering::Greater => println!("Загаданное число больше"),
        }
        guess.clear();
    }
}