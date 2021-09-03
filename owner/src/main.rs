fn main() {
    //string
    let s = "hello";           //immutable!
    println!("{}", s);

    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1;

    //ERROR! Because s1 move to s2
    //println!("{}, {}", s1, s2);
    println!("{}", s2);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1: {}, s2: {}", s1, s2);

    //copy가 가능한 타입들
    // 1. 정수형
    // 2. boolean
    // 3. 부동소수점
    // 4. copy 가능한 타입들로 이루어진 튜플
    // ex) (i32, i32) O
    //     (u32, string) X
}
