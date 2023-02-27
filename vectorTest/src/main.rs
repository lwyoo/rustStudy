fn main() {
    println!("Hello, world!");
    let v1: Vec<i32> = Vec::new();
    let mut v2 = vec![1,2,3,4];

    let temp = v2[3];

    let popValue = v2.pop();
    println!("{:?}", popValue);
    v2.push(100);

    for x in &v2 {
        println!("{x}");
    }
    /*
    println!("=======");

    for x in v2 {
        println!("{x}")
    }
*/
    /*
    let third: Option<&i32> = v2.get(1);
    let four = third.unwrap();
    println!("{}", four);
    */
}
