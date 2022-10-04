#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn print(&self) {
        println!("Rectangle width:{}, height: {} ", self.width, self.height);
    }
}

pub fn struct2_test() {
    let r1 = Rectangle {
        width: 10,
        height: 20,
    };

    let r2 = Rectangle {
        width: 8,
        height: 10,
    };

    let r3 = Rectangle {
        width: 20,
        height: 30,
    };

    println!("{},{}", r1.width, r1.height);
    println!("{}", r1.area());

    println!("{}", r1.can_hold(&r2));
    println!("{}", r1.can_hold(&r3));

    let r4 = Rectangle::square(321);

    r4.print();
    println!("{}", r4.can_hold(&r3));
}
