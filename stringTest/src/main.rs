fn main() {
    // 스트링 리터럴
    let data = "asdasd";
    // 스트링 리터럴로 부터  스트링 셍상하기
    let s = data.to_string();
    // 스트링 리터럴로 부터  스트링 셍상하기
    let s = "asdasd".to_string();
    // 스트링 리터럴로 부터  스트링 셍상하기
    let s = String::from("asdasd");
/*
    // update string
    let mut  updateStr = String::from("hellow");
    // updateStr = updateStr + "asdasd";
    let temp = String::from(" world");

    updateStr = updateStr + &temp;
    println!("{}", updateStr);
*/
/*
    let mut s = String::from("asd");
    s.push_str("qqqqq");
    println!("{}", s);
    let s2 = "aaaaa";
    s.push_str(&s2);

    println!("{}", s);
*/
/*
    let mut s = String::from("lo");
    s.push('a');
    println!("{s}");
*/

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1은 여기서 이동되어 더이상 쓸 수 없음을 유의하세요

    // println!("{}",s1);
    println!("{}",s2);
    println!("{}",s3);
}

