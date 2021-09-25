fn main() {
    let mut mut_v: Vec<i32> = Vec::new();
    // mut_v can be declared            let mut mut_v = Vec::new();
    // Because rust can infer it's type.
    let v = vec![1,2,3];

    mut_v.push(5);
    mut_v.push(6);
    mut_v.push(7);
    mut_v.push(8);
    println!("{:?}", v);
    println!("{:?}", mut_v);


    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("{}", third);

    let third: Option<&i32> = v.get(2);
    println!("{:?}", third);

    // let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100];
    // println!("{:?}", does_not_exist);
    // let does_not_exist = v.get(100);
    // println!("{:?}", does_not_exist);

    let v = vec![1, 2, 3, 4, 5];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);
}  // <- 벡터 변수들이 스코프 밖으로 벗어났고, 여기서 해제됩니다
