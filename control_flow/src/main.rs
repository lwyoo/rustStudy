// fn main() {
//     let x = 1111;

//     // let y = if x != 20 {
//     //     println!("x");
//     //     1
//     // } else {
//     //     println!("o");
//     //     2
//     // };

//     let y = if x != 20 {1} else {2};

//     println!("{y}");
// }
// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {result}");
// }

// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {count}");
// }
fn main() {
    let a = [1, 2, 3, 4];

    for value in a {
        println!("{value}");
    }
}