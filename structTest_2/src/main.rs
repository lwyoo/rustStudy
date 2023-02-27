// // test 1 - 함를 이용한 넓이 구기
// fn main() {
//     let length1 = 50;
//     let width1 = 30;
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(length1, width1)
//     );
// }
// fn area(length: u32, width: u32) -> u32 {
//     length * width
// }

// // test 2 - tuple 을 이용한 넓이 구기
// fn main() {
//     let rec1 = (50, 30);

//     println!("The area of the rectangle is {} square pixels.", area(rec1));
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// // test 3 - 구조체를 이용한 계산
// struct Rec {
//     length: u32,
//     width: u32,
// }

// fn main() {
//     let rec1 = Rec {
//         length: 10,
//         width: 30,
//     };
//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rec1)
//     );
// }

// fn area(sValue: &Rec) -> u32 {
//     sValue.length * sValue.width
// }

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let rec = Rectangle {length: 10, width: 20};
    println!("{:#?}", rec);
    println!("{:?}", rec);
}