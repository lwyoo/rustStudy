// // fn main() {
// //     let mut s = String::from("hello");
// //     // let mut s = "hello";
    
// //     s.push_str(", world!");

// //     println!("{}", s);
// // }

// fn main() {
//     // let mut test = 10;
//     // let test2 = test;
//     // test = 20;
//     // println!("test is {test}, test2 {test2}");
//     let mut s1 = String::from("testValue");
//     let s2 = s1.clone();

//     s1 = s1 + "asdasd";
//     println!("s1 is {s1}, s2 is {s2}");


//     // let s1 = String::from("hello");
//     // let s2 = s1;
//     // println!("{s2}")
// }
//
// fn main() {
//     let s = String::from("Hello");
//     takes_ownership(s.clone());
//     println!("{s}");
// }
//
// fn takes_ownership(some_string: String) {
//     println!("{some_string}");
// }

// fn main() {
//     let s1 = String::from("hellow");
//     let s2 = string_len(&s1);
//     println!("{}", s2);
//     println!("{}", s1);
// }

// fn string_len(value: &String) -> usize {
//     value.len()
// }

// fn main() {
    
//     let mut s = String::from("hello");

//     let r1 = &mut s;
//     let r2 = &r1;

//     println!("{}, {}", r1, r2);
// }

// fn main() {
//     // let mut s = String::from("hellow");

//     // let r1 = &s;
//     // let r2 = &s;
//     // println!("{}, {}", r1, r2);

//     // let r3 = &mut s;
//     // println!("{r3}");
//     let mut s = String::from("hello");

//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     println!("{} and {}", r1, r2);
//     // variables r1 and r2 will not be used after this point

//     let r3 = &mut s; // no problem
//     println!("{}", r1);
// }

// fn main() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

// fn main() {
//    let s = String::from("A BC DE");
//    let s2 = first_word(&s);

//    println!("{s2}");
//    println!("{s}");

// }

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         println!("index : {i}, byte : {item}");
//     }

//     s.len()
// }

// fn main() {
//     let s = String::from("hello");

//     let len = s.len();
//     println!("{len}");
//     let slice = &s[0..2];
//     println!("{}", slice);
//     let slice = &s[..];
//     println!("{}", slice);

// }

//
//
// fn main() {
//     let mut s = String::from("hello world");
//
//     let word = first_word(&s);
//     println!("{s}");
//     s.clear(); // error!
//
//     // println!("the first word is: {}", word);
// }
// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();
//
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//
//     &s[..]
// }

//
// fn main() {
//     let s = String::from("testCode");
//     takes_ownership(s.clone());
//     println!("{}", s);
//
//     let x = 5;
//     makes_copy(x);
//     println!("{}", x);
//
// }
//
// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }
//
// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }


fn main() {
    let s1 = gives_ownership();
    println!("{}", s1);
    let s2 = String::from("HHH");
    println!("{}", s2);
    let s3 = takes_and_gives_back(s2);
    println!("{}", s3);
}

fn gives_ownership() -> String {
    let some_string = String::from("gives_ownership");
    some_string
}


fn takes_and_gives_back(a_string: String) -> String {
    a_string
}