// fn main() {
//     // Variables and Mutability
//     // let x = 5; // error
//     let mut x = 5;
//     println!("x is {x}");
//     x = 6;
//     println!("update x is {x}");

//     // Constants
//     const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
//     println!("THREE_HOURS_IN_SECONDS is {THREE_HOURS_IN_SECONDS}");

//     // shadowing
//     let y = 5;

//     let y = y + 1;

//     {
//         let y = y * 2;
//         println!("The value of x in the inner scope is: {y}");
//     }

//     println!("The value of x is: {y}");

//     let spaces = "   ";
    
//     let spaces = spaces.len();
//     println!("space size is : {spaces}");

//     let testDecimal = 10_22;
//     println!("test value is : {testDecimal}");

//     let testHex = 0xff;
//     let testOctal = 0o77;
//     let testBinary = 0b111_000;
//     let testByte = b'A';

//     println!("testHex is : {testHex}");
//     println!("testOctal is : {testOctal}");
//     println!("testBinary is : {testBinary}");
//     println!("testByte is : {testByte}");


    
    
// }
// fn main() {
//     // addition
//     let sum = 5 + 10;
//     println!("sum {sum}");

//     // subtraction
//     let difference = 95.5 - 4.3;
//     println!("difference {difference}");

//     // multiplication
//     let product = 4 * 30;
//     println!("product {product}");

//     // division
//     let quotient = 56.7 / 32.2;
//     println!("quotient {quotient}");
//     let truncated = -5 / 3; // Results in -1
//     println!("truncated {truncated}");

//     // remainder
//     let remainder = 43 % 5;
//     println!("remainder {remainder}");
// }

fn main() {
    let myTuple: (i32, f64, u8) = (100, 100.22, 1);
    let fir = myTuple.2;
    println!("tuple[0] : {fir}");
}