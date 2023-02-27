// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
// }
//
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }
//
// fn main() {
//     println!("Hello, world!");
//     let value = value_in_cents(Coin::Quarter(UsState::Alaska));
//     println!("value : {}", value);
// }
//
// fn value_in_cents(coin: Coin) -> u32 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 1,
//         Coin::Dime => 1,
//         Coin::Quarter(state) => {
//             match state {
//                 UsState::Alabama => {
//                     println!("State quarter from {:?}", state);
//                     225
//                 }
//                 UsState::Alaska => {
//                     println!("State quarter from {:?}", state);
//                     335
//                 }
//             }
//         }
//     }
// }


// fn main() {
//     let tempNone: Option<i32> = None;
//     let five: Option<i32> = Some(5);
//     let six= plus_one(five);
//     let seven= plus_one(six);
//
//     println!("test six : {:?}", six);
//     println!("test seven : {:?}", seven);
// }
//
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

fn main() {
    let some = 0x10;

    match some {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => println!("!!!!!!!!"),
    }
}

