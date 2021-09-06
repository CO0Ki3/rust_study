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

    let s = String::from("Hello");

    takes_ownership(s);

    //함수 안으로 값이 들어가고 소유권이 이전됨에 따라
    //이후로부터는 s는 drop, 따라서 메모리는 해제됨.
    //만약 값을 계속 쓰고싶다면 clone을 떠서 보내거나

    let s1 = "Hello";

    takes_ownership(s2.to_string());

    println!("{}", s2);

    //스트링 레터럴을 String으로 변환 시킨 후 사용

    let x = 5;

    makes_copy(x);

    let s1 = gives_ownership();

    let s2 = String::from("Hello2");

    let s3 = takes_and_gives_back(s2);

    println!("s1: {}, s2: 소유권 이전!, s3: {}", s1, s3);
}

fn takes_ownership(value: String) {
    println!("{}", value);
}

fn makes_copy(value: i32) {
    println!("{}", value);
}

fn gives_ownership() -> String {
    let temp_string = String::from("Hello1");

    temp_string
}

fn takes_and_gives_back(value: String) -> String {
    value
}