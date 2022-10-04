struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn show(&self) {
        println!("width: {}, height: {}", self.width, self.height);
    }

    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

pub fn impl_test() {
    let r1 = Rectangle {
        width: 10,
        height: 20,
    };

    println!("{}, {}", r1.width, r1.height);
    r1.show();
    println!("area is {}", r1.area());
    println!("is square: {}", r1.is_square());
}
