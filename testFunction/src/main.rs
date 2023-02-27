// fn main() {
//     println!("Hello, world!");
//     test_my_function();
//     let x = 0;
//     let y = x;
// }

// fn test_my_function() {
//     println!("call me ~~~~~~~~");
//     test_arg(90, 100);
// }

// fn test_arg(x: u32, y: u32) {
//     println!("input arg[0] is {x}");
//     println!("input arg[1] is {y}");
// }
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
    let z = test_retun_value(100);
    println!("z is {z}");
}

fn test_retun_value(x: u32) -> u32 {
    5 + x
}