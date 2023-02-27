// use std::io;

// fn main() {
//     println!("Guess the number!");
//     println!("Please input your guess.");

//     let mut guess = String::new();

//     io::stdin().read_line(&mut guess).expect("Failed to read line");
//     // io::stdin().read_line(&mut guess);
//     println!("You guessed: {guess}");


//     // // test code 
//     // let x = 10;
//     // let y = 20;

//     // println!("x = {x}, y = {}, x = {}", y , x);
// }
///////////////////////////////////////////////////////////////////////////
// use std::io;
// use rand::Rng;

// fn main() {
//     println!("Guess the number!");
//     let secret_number = rand::thread_rng().gen_range(1..=100);
//     println!("The secret number is: {secret_number}");

//     println!("Please input your guess.");

//     let mut guess = String::new();

//     io::stdin().read_line(&mut guess).expect("Failed to read line");
//     // io::stdin().read_line(&mut guess);
//     println!("You guessed: {guess}");
// }
// ///////////////////////////////////////////////////////////////////////////
// use std::io;
// use std::cmp::Ordering;
// use rand::Rng;

// fn main() {
//     println!("Guess the number!");
//     let secret_number = rand::thread_rng().gen_range(1..=100);
//     println!("The secret number is: {secret_number}");

//     println!("Please input your guess.");

//     let mut guess = String::new();

//     io::stdin().read_line(&mut guess).expect("Failed to read line");

//     let guess: u32 = guess.trim().parse().expect("Please type a number!");
//     match guess.cmp(&secret_number) {
//         Ordering::Less => println!("Too small!"),
//         Ordering::Greater => println!("Too big!"),
//         Ordering::Equal => println!("You win!"),
//     }
// }
///////////////////////////////////////////////////////////////////////////
// use std::io;
// use std::cmp::Ordering;
// use rand::Rng;

// fn main() {
//     println!("Guess the number!");
//     let secret_number = rand::thread_rng().gen_range(1..=100);
//     println!("The secret number is: {secret_number}");

//     loop {
//         println!("Please input your guess.");

//         let mut guess = String::new();

//         io::stdin().read_line(&mut guess).expect("Failed to read line");

//         let guess: u32 = guess.trim().parse().expect("Please type a number!");
//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("Too small!"),
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Equal => {
//                 println!("You win!");
//                 break;
//             }
//         }
//     }
// }
// ///////////////////////////////////////////////////////////////////////////
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // underscore is a catchall value
            Err(_) => {
                println!("I kill you");
                continue;
            }
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
///////////////////////////////////////////////////////////////////////////