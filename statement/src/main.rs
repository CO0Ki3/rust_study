fn main() {
    //if statement
    let condition = true;
    let number = if condition {
        6
    } else {
        5
    };

    println!("number value is: {}", number);

    //loop statement
    let mut temp = 0;
    loop {
        println!("LOOP!");
        println!("{}", temp);
        temp = temp + 1;
        if temp == 5 {
            break;
        }
    }

    //while statement
    let mut temp = 5;
    while temp != 0 {
        println!("WHILE!");
        println!("{}", temp);
        temp = temp - 1;
    }

    //for statement
    let item = [0,1,2,3,4];

    for element in item.iter() {
        println!("item value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("number is: {}", number);
    }
}
