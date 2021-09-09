#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    //함수의 인자는 읽는지(&self), 변형시키는지(&mut self), 소비하는지(self)에 따라 참조가 다르다는 것을 기억하자
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

fn main() {
    let rect1 = Rectangle {
        height: 50,
        width: 30,
    };

    println!(
        "The area of the rectangle is {} square pixels.\n rect1: {:#?}", 
        rect1.area(), rect1
    );
}