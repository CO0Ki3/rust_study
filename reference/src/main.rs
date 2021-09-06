fn main() {
    //TOO MUCH!
    let s1 = String::from("Hello");
    let (s2, len) = calculate_length(s1);

    println!("s2: {}, len: {}", s2, len);

    let s1 = String::from("hello");
    //&s1을 함으로써 값을 참조하지만 소유하지 않는 참조자를 생성
    let len = calculate_length2(&s1);
    println!("s1: {}, len: {}", s1, len);

    let mut s1 = String::from("Hello");
    //only one!
    change(&mut s1);

    // let mut s1 = String::from("helllllllo mutable?");
    // let s2 = &s1;
    // let s4 = &mut s1;                                // Wrong!! Because It's already immutable ref.
    // println!("{}, {}", s2, s4);

    // make dangling references
    // let references_to_nothing = dangel();           
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn calculate_length2(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", World!");
}


//Wrong!! 이 함수의 반환 타입은 빌린 값을 포함하고 있는데, 빌려온 실제 값은 없습니다.
// fn dangle() -> &String {                                          //dangle은 String의 참조자를 반환
//     let s = String::from ("Hello");
//     &s                                                            //String s의 참조자를 반환
// }

// 1. 어떠한 경우이든 간에, 여러분은 아래 둘 다는 아니고 둘 중 하나만 가질 수 있습니다:
//    - 하나의 가변 참조자
//    - 임의 개수의 불변 참조자들
// 2. 참조자는 항상 유효해야만 한다.

//출처: https://rinthel.github.io/rust-lang-book-ko/ch04-02-references-and-borrowing.html#%EB%8C%95%EA%B8%80%EB%A7%81-%EC%B0%B8%EC%A1%B0%EC%9E%90dangling-references