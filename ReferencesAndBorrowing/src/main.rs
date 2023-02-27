
/*
fn main() {
    println!("Hello, world!");
    let (a, b) = testFunction();
    println!("String value : {a}, int value : {b}");
}

fn testFunction() -> (String, u32) {
    let s1 = String::from("hellow");
    let i1 = 36;
    (s1, i1)
}
*/
fn main() {
    let mut s1 = String::from("Hello");
    let i1 = testFunction(&mut s1);
    println!("string value : {s1}, string length : {i1}");

}

fn testFunction(s: &mut String) -> usize {
    s.push_str(", world");
    s.len()
}